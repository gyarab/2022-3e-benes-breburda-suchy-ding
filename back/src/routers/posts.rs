use std::sync::Arc;

use crate::{fileman::FileManager, state::StateWithDb, models::{User, Post, PostPub, ClientError, Comment}, utils::{resp, parse_uuid, get_post_likes, get_comment_count}, security, worker::WorkQueue};
use serde::Deserialize;
use sqlx::{PgPool, PgExecutor};
use tide::{Request, StatusCode, prelude::json};
use uuid::Uuid;



#[derive(Clone)]
pub struct WebState {
    pub pool: Arc<PgPool>,
    pub fileman: Arc<FileManager>,
    pub worker: Arc<WorkQueue>,
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

    resp(200, &PostPub::from(post))
}

pub async fn get_post(req: Request<WebState>) -> tide::Result {
    let post_id = parse_uuid(req.param("post_id").expect("post_id parameter not set"))?;

    let mut pub_post = json!(PostPub::from(get_post_or_err(req.state().db(), &post_id).await?));

    pub_post["likes"] = get_post_likes(req.state().db(), &post_id).await?.into();
    pub_post["comments"] = get_comment_count(req.state().db(), &post_id).await?.into();

    resp(200, &pub_post)
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

    let posts = sqlx::query_as!(Post, "
        WITH post_rankings AS (
            SELECT post_id, rank
            FROM post_rank
            WHERE user_id = $1
        )
        SELECT p.* 
        FROM posts p
        LEFT JOIN post_rankings pr ON p.post_id = pr.post_id
        ORDER BY pr.rank DESC, p.created DESC
        LIMIT 10
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

    let posts_pub: Vec<PostPub> = posts.into_iter().map(|post| PostPub::from(post)).collect();

    resp(200, &json!(posts_pub))
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
    get_post_or_err(req.state().db(), &post_id).await?;


    let comment = sqlx::query_as!(Comment, "INSERT INTO comments (post_id, author_id, content) VALUES ($1, $2, $3) RETURNING *", &post_id, &user.user_id, body.content).fetch_one(req.state().db()).await?;

    resp(200, &json!(comment))
}


pub async fn get_router(pool: Arc<PgPool>, fileman: Arc<FileManager>, worker: Arc<WorkQueue>) -> tide::Server<WebState> {
    let mut app = tide::with_state(WebState { pool, fileman, worker });

    app.at("/").with(security::session_guard).post(create_post);
    app.at("/:post_id").get(get_post).with(security::session_guard).delete(delete_post);
    app.at("/:post_id/content").get(get_post_content);
    app.at("/:post_id/comments").get(get_post_comments).with(security::session_guard).post(create_post_comment);

    app.at("/:post_id/like").with(security::session_guard).post(like_post).delete(unlike_post);

    app.at("/feed").with(security::session_guard).get(get_posts);

    app
}
