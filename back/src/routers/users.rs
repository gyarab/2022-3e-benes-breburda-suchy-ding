use crate::models::ClientError;
use crate::models::User;
use sqlx::PgPool;
use tide::prelude::*;
use tide::Request;
use crate::state::{WebState, StateWithDb as _};

async fn get_all_users(req: Request<WebState>) -> tide::Result {
    let res = sqlx::query_as!(User, "SELECT * FROM users;")
        .fetch_all(req.state().db())
        .await?;
    Ok(tide::Body::from_json(&res)?.into())
}

async fn create_user(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize)]
    struct ReqBody {
        pub name: String,
        pub email: String,
        pub password: String,
    }
    let body: ReqBody = req.body_json().await?;

    // validate input


    // check user does not exist
    let result = sqlx::query!(
        "SELECT 1 as exists FROM users WHERE email = $1",
        body.email
    )
    .fetch_all(req.state().db())
    .await?;
    if result.len() > 0 {
        return Err(ClientError::new(409, "user_exists", "A user with this email already exists").into());
    }

    return Ok(tide::Response::new(200));
}

pub async fn get_router(pool: Box<PgPool>) -> tide::Server<WebState> {
    let mut app = tide::with_state(WebState { pool });
    app.at("/").get(get_all_users);
    app.at("/").post(create_user);
    app
}
