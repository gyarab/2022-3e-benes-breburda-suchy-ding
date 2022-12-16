use crate::models::User;
use sqlx::PgPool;
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Clone)]
pub struct WebState {
    pool: Box<PgPool>,
}

impl WebState {
    pub fn db(&self) -> &PgPool {
        &self.pool
    }
}

async fn get_all_users(req: Request<WebState>) -> tide::Result {
    let res = sqlx::query_as!(User, "SELECT * FROM users;")
        .fetch_all(req.state().db())
        .await?;
    Ok(tide::Body::from_json(&res)?.into())
}

async fn create_user(mut req: Request<WebState>) -> tide::Result {
    // check user does not exist
    let to_create: User = req.body_json().await?;
    let result = sqlx::query!(
        "SELECT 1 as exists FROM users WHERE email = $1",
        to_create.email
    )
    .fetch_all(req.state().db())
    .await?;
    if result.len() > 0 {
        Ok(tide::Response::new(409))
    } else {
        Ok(tide::Response::new(200))
    }
}

pub async fn get_router(pool: Box<PgPool>) -> tide::Server<WebState> {
    let mut app = tide::with_state(WebState { pool });
    app.at("/").get(get_all_users);
    app.at("/").post(create_user);
    app
}
