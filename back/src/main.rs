mod models;
mod routers;
mod security;
mod state;
mod config;
use sqlx::postgres::PgPoolOptions;
use tide::prelude::json;
use std::env;
use async_std::fs::File;
use tide::log;
use tide::Request;

async fn demo(mut _req: Request<()>) -> tide::Result {
    Ok("hello world".into())
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

    let mut app = tide::new();

    app.with(tide::utils::After(|mut res: tide::Response| async {
        if let Some(err) = res.downcast_error::<models::ClientError>() {
            let code = err.code.to_owned();
            let message = err.message.to_owned();
            res.set_status(err.status_code);
            res.set_body(json!({
                "code": code,
                "message": message,
            }));
        }
        Ok(res)
    }));

    app.at("/").get(demo);
    app.at("/api/users")
        .nest(routers::users::get_router(pool.clone()).await);
    app.at("/api/sessions")
        .nest(routers::sessions::get_router(pool.clone()).await);
    app.listen(format!("{}:{}", config.host, config.port)).await?;
    Ok(())
}
