mod models;
mod routers;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tide::log;
use tide::prelude::*;
use tide::Request;

async fn demo(mut _req: Request<()>) -> tide::Result {
    Ok("hello world".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    let mut app = tide::new();
    let pool = Box::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DB_URL").expect("DB_URL env variable not found"))
            .await?,
    );
    app.at("/").get(demo);
    app.at("/api/users")
        .nest(routers::users::get_router(pool.clone()).await);
    app.at("/api/users2")
        .nest(routers::users::get_router(pool.clone()).await);
    app.listen("0.0.0.0:8000").await?;
    Ok(())
}
