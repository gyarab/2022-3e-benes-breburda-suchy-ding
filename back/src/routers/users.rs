use crate::models::ClientError;
use crate::models::User;
use crate::models::UserPub;
use crate::utils::resp;
use lettre::AsyncSmtpTransport;
use lettre::AsyncStd1Executor;
use lettre::AsyncTransport;
use lettre::Message;
use sqlx::PgPool;
use tide::prelude::*;
use tide::Request;
use validator::Validate;
use crate::state::StateWithDb;
use crate::security;
use anyhow::anyhow;

type SmtpExecutor = AsyncSmtpTransport<AsyncStd1Executor>;

#[derive(Clone)]
pub struct WebState {
    pub pool: Box<PgPool>,
    pub smtp: Box<SmtpExecutor>,
}

impl WebState {
    async fn send_mail(&self, msg: Message) -> anyhow::Result<()> {
        match self.smtp.send(msg).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("failed to send email {}", e))
        }
    }
}

impl StateWithDb for WebState {
    fn db(&self) -> &PgPool {
        &self.pool
    }
}

async fn get_me(req: Request<WebState>) -> tide::Result {
    let user: User = req.ext::<User>().unwrap().to_owned();
    resp(200, &UserPub::from(user))
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


    resp(200, &UserPub::from(usr))
}

async fn update_me(mut req: Request<WebState>) -> tide::Result {
    #[derive(Deserialize, Validate)]
    struct ReqBody {
        #[validate(length(min = 3, max = 30))]
        pub name: String,
        #[validate(email)]
        pub email: String,
    }
    let body: ReqBody = req.body_json().await?;
    body.validate()?;

    resp(200, &json!({}))
}

pub async fn get_router(pool: Box<PgPool>, smtp: Box<SmtpExecutor>) -> tide::Server<WebState> {
    let mut app = tide::with_state(WebState { pool, smtp });

    app.at("/me").with(security::session_guard).get(get_me);
    app.at("/").post(create_user);

    app
}
