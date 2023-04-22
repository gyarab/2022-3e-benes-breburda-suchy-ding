use crate::fileman::FileManager;
use crate::mail::Mailer;
use crate::models::ClientError;
use crate::models::User;
use crate::models::UserPub;
use crate::utils::parse_uuid;
use crate::utils::resp;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::QueryBuilder;
use tide::StatusCode;
use tide::prelude::*;
use tide::Request;
use uuid::Uuid;
use validator::Validate;
use crate::state::StateWithDb;
use crate::security;

#[derive(Clone)]
pub struct WebState {
    pub pool: Box<PgPool>,
    pub mailer: Box<Mailer>,
    pub fileman: Box<FileManager>,
}


impl StateWithDb for WebState {
    fn db(&self) -> &PgPool {
        &self.pool
    }
}

async fn get_me(req: Request<WebState>) -> tide::Result {
    // pub user with email added
    let user: User = req.ext::<User>().unwrap().to_owned();
    let email = user.email.to_owned();
    let mut json = json!(UserPub::from(user));
    json["email"] = email.into();
    resp(200, &json)
}

async fn create_user(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize, Validate)]
    struct ReqBody {
        #[validate(length(min = 3, max = 30))]
        pub name: String,
        #[validate(email)]
        pub email: String,
        #[validate(length(min = 8))]
        pub password: String,
    }
    let body: ReqBody = req.body_json().await?;
    body.validate()?;


    // check user does not exist
    let result = sqlx::query!(
        "SELECT 1 as exists FROM users WHERE email = $1",
        body.email
    )
    .fetch_optional(req.state().db())
    .await?;
    if result.is_some() {
        return Err(ClientError::new(409, "user_exists", "A user with this email already exists").into());
    }

    let usr = sqlx::query_as!(User, "
        INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *
    ",
        body.name,
        body.email,
        security::hash(&body.password).await?
    ).fetch_one(req.state().db()).await?;

    // NOTIME: should be done after the request has been sent
    req.state().mailer.send_mail(&body.email, "Thank you for registering", "Your email will be used for notifications and resetting your password in the case you lose it.\n\nDing dong!".to_owned()).await?;

    resp(200, &UserPub::from(usr))
}

async fn update_me(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize, Validate)]
    struct ReqBody {
        #[validate(length(min = 3, max = 30))]
        pub name: Option<String>,
        #[validate(email)]
        pub email: Option<String>,
        #[validate(length(max = 600))]
        pub bio: Option<String>,
    }
    let body: ReqBody = req.body_json().await?;
    body.validate()?;
    let user: &User = req.ext().unwrap();

    let mut query = QueryBuilder::<Postgres>::new(
        "UPDATE users SET "
    );

    let mut sep = query.separated(", ");
    if let Some(name) = body.name {
        sep.push("name = ").push_bind_unseparated(name);
    }
    if let Some(email) = body.email {
        sep.push("email = ").push_bind_unseparated(email);
    }
    if let Some(bio) = body.bio {
        sep.push("bio = ").push_bind_unseparated(bio);
    }
    query.push(" WHERE user_id = ").push_bind(user.user_id);
    query.build().execute(req.state().db()).await?;

    Ok(StatusCode::Ok.into())
}

async fn delete_me(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize)]
    struct ReqBody {
        pub password: String,
    }
    let body: ReqBody = req.body_json().await?;
    let user: &User = req.ext().unwrap();

    // check password
    if !security::verify(&body.password, &user.password).await? {
        return Err(ClientError::new(401, "incorrect_password", "Password provided is incorrect").into())
    }

    sqlx::query!("DELETE FROM users WHERE user_id = $1", user.user_id).execute(req.state().db()).await?;

    Ok(StatusCode::Ok.into())
}

async fn update_password(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize, Validate)]
    struct ReqBody {
        pub old_password: String,
        #[validate(length(min = 8))]
        pub password: String,
    }
    let body: ReqBody = req.body_json().await?;
    body.validate()?;
    let user: &User = req.ext().unwrap();

    // check password
    if !security::verify(&body.old_password, &user.password).await? {
        return Err(ClientError::new(401, "incorrect_password", "Password provided is incorrect").into())
    }

    sqlx::query!(
        "UPDATE users SET password = $1 WHERE user_id = $2",
        security::hash(&body.password).await?,
        user.user_id
    ).execute(req.state().db()).await?;

    Ok(StatusCode::Ok.into())
}

async fn send_restore_password(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize, Validate)]
    struct ReqBody {
        #[validate(email)]
        pub email: String,
    }
    let body: ReqBody = req.body_json().await?;
    body.validate()?;

    let mut db = req.state().db().begin().await?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email).fetch_optional(&mut db).await?;

    if let Some(user) = user {
        let res = sqlx::query!(
            "INSERT INTO email_tokens (user_id, token_type) VALUES ($1, 'password_restore') RETURNING token",
            user.user_id
        ).fetch_one(&mut db).await?;

        req.state().mailer.send_mail(
            &user.email,
            "Forgotten password",
            format!("Please click the following link to reset your password: https://ding.ecko.ga/password_reset?token={}", res.token)
        ).await?;
    }

    db.commit().await?;

    Ok(StatusCode::Ok.into())
}

async fn restore_password(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize, Validate)]
    struct ReqBody {
        pub token: String,
        #[validate(length(min = 8))]
        pub password: String,
    }
    let body: ReqBody = req.body_json().await?;
    body.validate()?;

    let mut db = req.state().db().begin().await?;

    let user = sqlx::query_as!(User, "
        SELECT u.*
        FROM users u
        JOIN email_tokens e ON u.user_id = e.user_id
        WHERE e.token = $1 AND e.token_type = 'password_restore'
    ",
        body.token
    ).fetch_optional(&mut db).await?;

    if user.is_none() {
        return Err(ClientError::new(404, "invalid_token", "Token is invalid").into());
    }
    let user = user.unwrap();

    sqlx::query!(
        "UPDATE users SET password = $1 WHERE user_id = $2",
        security::hash(&body.password).await?,
        &user.user_id
    ).execute(&mut db).await?;

    db.commit().await?;

    Ok(StatusCode::Ok.into())
}

async fn get_profile_pic(req: Request<WebState>, user_id: &Uuid) -> tide::Result {
    let res = sqlx::query!(
        "SELECT profile_picture FROM users WHERE user_id = $1",
        user_id
    ).fetch_optional(req.state().db()).await?;

    match res {
        Some(res) => {
            match res.profile_picture {
                Some(pfp) => {
                    Ok(req.state().fileman.get_body(&pfp).await?.into())
                }
                None => {
                    Err(ClientError::new(404, "no_pfp", "User does not have a profile picture").into())
                }
            }
        },
        None => {
            Err(ClientError::new(404, "user_not_found", "User was not found").into())
        }
    }

}

async fn me_get_profile_pic(req: Request<WebState>) -> tide::Result {
    let user = req.ext::<User>().unwrap().user_id;

    get_profile_pic(req, &user).await
}

async fn upload_profile_pic(mut req: Request<WebState>) -> tide::Result {
    let mut body = req.take_body();

    // consume body and create file
    let file = req.state().fileman.create_file(&mut body).await?;

    let user: &User = req.ext().unwrap();
    sqlx::query!(
        "UPDATE users SET profile_picture = $1 WHERE user_id = $2",
        file,
        user.user_id
    ).execute(req.state().db()).await?;

    Ok(StatusCode::Ok.into())
}

async fn get_user(req: Request<WebState>) -> tide::Result {
    let user_id = parse_uuid(req.param("user_id").expect("user_id parameter not set"))?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id).fetch_optional(req.state().db()).await?;

    match user {
        Some(user) => resp(200, &UserPub::from(user)),
        None => Err(ClientError::new(404, "user_not_found", "User does not exist").into()),
    }
}

async fn get_user_profile_pic(req: Request<WebState>) -> tide::Result {
    let user_id = parse_uuid(req.param("user_id").expect("user_id parameter not set"))?;

    get_profile_pic(req, &user_id).await
}

pub async fn get_router(pool: Box<PgPool>, mailer: Box<Mailer>, fileman: Box<FileManager>) -> tide::Server<WebState> {
    let mut app = tide::with_state(WebState { pool, mailer, fileman });

    app.at("/me").with(security::session_guard)
        .get(get_me)
        .patch(update_me)
        .delete(delete_me);

    app.at("/me/password").with(security::session_guard)
        .put(update_password);

    app.at("/restore_password")
        .post(send_restore_password)
        .put(restore_password);

    // NOTIME: this does not have to be a .jpg
    app.at("/me/profile.jpg").with(security::session_guard)
        .get(me_get_profile_pic)
        .put(upload_profile_pic);

    app.at("/").post(create_user);
    app.at("/:user_id").get(get_user);
    app.at("/:user_id/profile.jpg").get(get_user_profile_pic);

    app
}
