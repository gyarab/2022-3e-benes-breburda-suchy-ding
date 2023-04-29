use std::{sync::Arc, process::Stdio};

use crate::{fileman::FileManager, state::StateWithDb, models::{User, Post, PostPub, ClientError, Comment, PostPubExt}, utils::{resp, parse_uuid, log_err}, security, worker::WorkQueue, config::Config};
use async_std::process::Command;
use serde::Deserialize;
use sqlx::{PgPool, PgExecutor};
use tide::{Request, StatusCode, prelude::json, log};
use uuid::Uuid;



#[derive(Clone)]
pub struct WebState {
    pub pool: Arc<PgPool>,
    pub fileman: Arc<FileManager>,
    pub worker: Arc<WorkQueue>,
    pub config: Config,
}


impl StateWithDb for WebState {
    fn db(&self) -> &PgPool {
        &self.pool
    }
}

#[inline(always)]
pub async fn get_post_or_err<'a, T: PgExecutor<'a>>(executor: T, post_id: &Uuid) -> anyhow::Result<Post> {
    match Post::from_db(executor, &post_id).await? {
        Some(post) => Ok(post),
        None => Err(ClientError::new(404, "post_not_found", "Post does not exist").into())
    }
}
 
pub async fn create_post(mut req: Request<WebState>) -> tide::Result {
    let file = {
        let mut body = req.take_body();
        req.state().fileman.create_file(&mut body).await?
    };

    let user: &User = req.ext().unwrap();

    let post = sqlx::query_as!(Post,
        "INSERT INTO posts (author_id, content) VALUES ($1, $2) RETURNING *",
        user.user_id,
        file,
    ).fetch_one(req.state().db()).await?;

    // run deferred checks against file
    let db = req.state().pool.clone();
    let whisper_root = req.state().config.whisper_cpp_root.clone();
    let disable_checks = req.state().config.disable_speech_checks;
    let fileman = req.state().fileman.clone();
    log_err(req.state().worker.send(Box::pin(async move {
        let status = Command::new("bash")
            .arg("run-speech-checks.sh")
            .arg(fileman.get_path(&file))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("WHISPER_ROOT", whisper_root)
            .env("DISABLE_CHECKS", disable_checks.to_string())
            .output().await;
        match status {
            Ok(status) => {
                match status.status.code() {
                    Some(0) => {},
                    Some(1) => {
                        // process exited with code 1 => bad audio => delete post
                        log_err(sqlx::query!("DELETE FROM posts WHERE post_id = $1", post.post_id).execute(db.as_ref()).await);
                        log_err(fileman.delete_file(&file).await);
                    },
                    _ => {
                        log::error!("checking speech process killed");
                        log::info!("{}", String::from_utf8(status.stdout).unwrap_or("failed to unwrap stdout".to_owned()));
                    }
                }
            },
            Err(e) => {
                log::error!("error checking speech: {:?}", e);
            }
        }
    })).await);

    resp(200, &PostPub::from(post))
}

pub async fn get_post(req: Request<WebState>) -> tide::Result {
    let user: &User = req.ext().unwrap();

    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let post = get_post_or_err(req.state().db(), &post_id).await?;

    // NOTIME: inconsistencies -> transaction
    let add_info = sqlx::query!("
        SELECT
            p.likes, p.comments,
            is_liked(p.post_id, $1) as liked,
            is_saved(p.post_id, $1) as saved
        FROM post_ext_info p
        WHERE p.post_id = $2
    ", user.user_id, post.post_id).fetch_one(req.state().db()).await?;

    resp(200, &PostPubExt::new(
        post,
        add_info.likes.unwrap(),
        add_info.comments.unwrap(),
        add_info.liked.unwrap(),
        add_info.saved.unwrap()
    ))
}

pub async fn get_post_content(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let post = Post::from_db(req.state().db(), &post_id).await?;
    match post {
        Some(post) => Ok(req.state().fileman.get_body(&post.content).await?.into()),
        None => Err(ClientError::new(404, "post_not_found", "Post does not exist").into())
    }
}

pub async fn delete_post(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let mut db = req.state().db().begin().await?;

    let post = get_post_or_err(&mut db, &post_id).await?;
    let user: &User = req.ext().unwrap();

    if post.author_id != user.user_id {
        return Err(ClientError::new(403, "not_author", "You're not the author of this post").into());
    }

    sqlx::query!("DELETE FROM posts WHERE post_id = $1", post.post_id).execute(&mut db).await?;
    req.state().fileman.delete_file(&post.content).await?;

    db.commit().await?;

    Ok(StatusCode::Ok.into())
}

pub async fn like_post(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let mut db = req.state().db().begin().await?;

    let post = get_post_or_err(&mut db, &post_id).await?;
    let user: &User = req.ext().unwrap();

    sqlx::query!("INSERT INTO post_likes (post_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING", post.post_id, user.user_id).execute(&mut db).await?;

    db.commit().await?;

    Ok(StatusCode::Ok.into())
}
pub async fn unlike_post(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let mut db = req.state().db().begin().await?;

    let post = get_post_or_err(&mut db, &post_id).await?;
    let user: &User = req.ext().unwrap();

    sqlx::query!("DELETE FROM post_likes WHERE post_id = $1 AND user_id = $2", post.post_id, user.user_id).execute(&mut db).await?;

    db.commit().await?;

    Ok(StatusCode::Ok.into())
}

pub async fn get_posts(req: Request<WebState>) -> tide::Result {
    let user: &User = req.ext().unwrap();

    // NOTIME: another mess of a query
    let posts = sqlx::query!("
        WITH post_rankings AS (
            SELECT post_id, rank
            FROM post_rank
            WHERE user_id = $1
        ),
        selected_posts AS (
            SELECT p.*, COALESCE(pr.rank, 0) as rank
            FROM posts p
            LEFT JOIN post_rankings pr ON p.post_id = pr.post_id
            ORDER BY COALESCE(pr.rank, 0) DESC, p.created DESC
            LIMIT 10
        )
        SELECT
            sp.*,
            ext.likes,
            ext.comments,
            l.liked,
            s.saved
        FROM selected_posts sp
        JOIN post_ext_info ext ON sp.post_id = ext.post_id
        JOIN get_liked($1) l ON sp.post_id = l.post_id
        JOIN get_saved($1) s ON sp.post_id = s.post_id
        ORDER BY sp.rank DESC, sp.created DESC
    ", &user.user_id).fetch_all(req.state().db()).await?;

    // update post ranks
    // NOTIME: sloooooow
    for post in &posts {
        sqlx::query!("
            INSERT INTO post_rank (post_id, user_id, rank)
            VALUES ($1, $2, -1)
            ON CONFLICT (post_id, user_id)
            DO UPDATE SET rank = post_rank.rank - 1
        ",
            post.post_id,
            user.user_id
        ).execute(req.state().db()).await?;
    }

    let posts_pub: Vec<PostPubExt> = posts.into_iter().map(|p| PostPubExt {
        post_id: p.post_id,
        author_id: p.author_id,
        created: p.created,
        likes: p.likes.unwrap(),
        comments: p.comments.unwrap(),
        liked: p.liked.unwrap(),
        saved: p.saved.unwrap()
    }).collect();

    resp(200, &json!(posts_pub))
}

pub async fn get_saved_posts(req: Request<WebState>) -> tide::Result {
    let user: &User = req.ext().unwrap();

    // NOTIME: paginate / after_post query and limit, oh and could be faster
    let posts = sqlx::query!("
        SELECT
            p.*,
            ext.likes,
            ext.comments,
            l.liked
        FROM posts p
        JOIN saved_posts sp ON p.post_id = sp.post_id
        JOIN post_ext_info ext ON p.post_id = ext.post_id
        JOIN get_liked($1) l ON p.post_id = l.post_id
        WHERE sp.user_id = $1
        ORDER BY sp.save_time DESC
    ", &user.user_id).fetch_all(req.state().db()).await?;

    let posts_pub: Vec<PostPubExt> = posts.into_iter().map(|p| PostPubExt {
        post_id: p.post_id,
        author_id: p.author_id,
        created: p.created,
        likes: p.likes.unwrap(),
        comments: p.comments.unwrap(),
        liked: p.liked.unwrap(),
        saved: true
    }).collect();

    resp(200, &json!(posts_pub))
}

pub async fn save_post(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let mut db = req.state().db().begin().await?;

    let post = get_post_or_err(&mut db, &post_id).await?;
    let user: &User = req.ext().unwrap();

    sqlx::query!("INSERT INTO saved_posts (post_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING", post.post_id, user.user_id).execute(&mut db).await?;

    db.commit().await?;

    Ok(StatusCode::Ok.into())
}

pub async fn unsave_post(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let mut db = req.state().db().begin().await?;

    let post = get_post_or_err(&mut db, &post_id).await?;
    let user: &User = req.ext().unwrap();

    sqlx::query!("DELETE FROM saved_posts WHERE post_id = $1 AND user_id = $2", post.post_id, user.user_id).execute(&mut db).await?;

    db.commit().await?;

    Ok(StatusCode::Ok.into())
}

pub async fn get_post_comments(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    get_post_or_err(req.state().db(), &post_id).await?;

    let comments = sqlx::query_as!(Comment, "SELECT * FROM comments WHERE post_id = $1 ORDER BY created DESC", &post_id).fetch_all(req.state().db()).await?;

    resp(200, &json!(comments))
}

pub async fn create_post_comment(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize)]
    struct ReqBody {
        pub content: String,
    }
    let body: ReqBody = req.body_json().await?;

    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;
    let user: &User = req.ext().unwrap();

    // check post exists
    let mut db = req.state().db().begin().await?;
    get_post_or_err(&mut db, &post_id).await?;

    let comment = sqlx::query_as!(Comment, "INSERT INTO comments (post_id, author_id, content) VALUES ($1, $2, $3) RETURNING *", &post_id, &user.user_id, body.content).fetch_one(&mut db).await?;

    db.commit().await?;

    resp(200, &json!(comment))
}


pub async fn get_router(pool: Arc<PgPool>, fileman: Arc<FileManager>, worker: Arc<WorkQueue>, config: Config) -> tide::Server<WebState> {
    let mut app = tide::with_state(WebState { pool, fileman, worker, config });

    app.at("/").with(security::session_guard).post(create_post);
    app.at("/:post_id").with(security::session_guard).get(get_post).delete(delete_post);
    app.at("/:post_id/content").get(get_post_content);
    app.at("/:post_id/comments").get(get_post_comments).with(security::session_guard).post(create_post_comment);

    app.at("/:post_id/like").with(security::session_guard).post(like_post).delete(unlike_post);
    app.at("/:post_id/save").with(security::session_guard).post(save_post).delete(unsave_post);

    app.at("/feed").with(security::session_guard).get(get_posts);
    app.at("/saved").with(security::session_guard).get(get_saved_posts);

    app
}
