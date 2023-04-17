mod models;
mod routers;
mod security;
mod state;
mod config;
mod utils;
use lettre::AsyncSmtpTransport;
use lettre::AsyncStd1Executor;
use lettre::transport::smtp::authentication::Credentials;
use sqlx::postgres::PgPoolOptions;
use tide::http::Method;
use tide::prelude::json;
use validator::ValidationErrors;
use std::env;
use std::future::Future;
use async_std::fs::File;
use tide::log;
use tide::Request;
use std::pin::Pin;

async fn demo(mut _req: Request<()>) -> tide::Result {
    Ok("Ding API".into())
}

async fn error_handler(mut res: tide::Response) -> tide::Result {
    if let Some(err) = res.downcast_error::<models::ClientError>() {
        let status = err.status_code;
        res.set_body(json!({
            "code": err.code,
            "message": err.message,
        }));
        res.set_status(status);
    }
    if let Some(err) = res.downcast_error::<ValidationErrors>() {
        res.set_body(json!({
            "code": "asdf",
            "message": "sdafsdf",
            "errors": err
        }));
        res.set_status(422);
    }
    Ok(res)
}

async fn cors(mut res: tide::Response) -> tide::Result {
    res.append_header("Access-Control-Allow-Origin", "*");
    Ok(res)
}

pub fn cors_preflight<'a>(
    request: tide::Request<()>,
    next: tide::Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        if request.method() == Method::Options {
            let resp = tide::Response::builder(204)
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .build();
            Ok(resp)
        } else {
            Ok(next.run(request).await)
        }
    })
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();

    let mut conf_file = File::open(env::var("CONFIG_FILE").unwrap_or("config.toml".to_owned())).await?;
    let config = config::Config::from_file(&mut conf_file).await?;

    let pool = Box::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?,
    ); 

    let smtp: AsyncSmtpTransport<AsyncStd1Executor> = AsyncSmtpTransport::<AsyncStd1Executor>::relay(&config.smtp_host)
        .unwrap()
        .credentials(Credentials::new(config.smtp_username, config.smtp_password))
        .build();

    let mut app = tide::new();

    app.with(tide::utils::After(error_handler));
    app.with(tide::utils::After(cors));
    app.with(cors_preflight);

    app.at("/").get(demo);
    app.at("/api/users")
        .nest(routers::users::get_router(pool.clone(), Box::new(smtp)).await);
    app.at("/api/sessions")
        .nest(routers::sessions::get_router(pool.clone()).await);

    app.listen(format!("{}:{}", config.host, config.port)).await?;

    Ok(())
}
