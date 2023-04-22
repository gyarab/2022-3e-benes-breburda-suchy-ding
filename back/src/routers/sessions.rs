use crate::{
    models::{ClientError, Session, User},
    security, utils::resp,
};
use serde::Deserialize;
use sqlx::PgPool;
use crate::state::{WebState, StateWithDb as _};

pub async fn create_session(mut req: tide::Request<WebState>) -> tide::Result {
    #[derive(Deserialize)]
    struct ReqBody {
        email: String,
        password: String,
    }
    let body: ReqBody = req.body_json().await?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1;", &body.email)
        .fetch_optional(req.state().db())
        .await?;
    match user {
        None => Err(ClientError::new(404, "user_not_exist", "The user does not exist").into()),
        Some(user) => {
            if !security::verify(&body.password, &user.password).await? {
                return Err(
                    ClientError::new(401, "invalid_password", "Invalid password").into(),
                );
            }
            let token = security::create_token(24).await?;
            let res = sqlx::query_as!(
                Session,
                "INSERT INTO sessions (user_id, token) VALUES ($1, $2) RETURNING *;",
                user.user_id,
                token
            )
            .fetch_one(req.state().db())
            .await?;

            resp(200, &res)
        }
    }
}
/// to be mounted at /sessions
pub async fn get_router(pool: Box<PgPool>) -> tide::Server<WebState> {
    let mut app = tide::with_state(WebState { pool });
    app.at("/").post(create_session);
    app
}
