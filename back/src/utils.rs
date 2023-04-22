use std::str::FromStr;

use sqlx::PgExecutor;
use uuid::Uuid;

use crate::models::ClientError;
use crate::models::User;

pub fn resp(status: u16, body: &impl serde::Serialize) -> tide::Result {
    let body = tide::Body::from_json(body)?;
    Ok(tide::Response::builder(status).body(body).build())
}

pub fn parse_uuid(string: &str) -> Result<Uuid, ClientError> {
    match Uuid::from_str(string) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(ClientError::new(400, "invalid_uuid", "Provided UUID was incorrect"))
    }
}

pub async fn get_listeners<'a, T: PgExecutor<'a>>(executor: T, user_id: &Uuid) -> anyhow::Result<Vec<User>> {
    let listeners = sqlx::query_as!(User,"
        WITH data AS (
            SELECT listener
            FROM listeners
            WHERE listenee = $1
        )
        SELECT u.*
        FROM users u
        JOIN data d ON d.listener = u.user_id
    ",
        user_id
    ).fetch_all(executor).await?;

    Ok(listeners)
}

pub async fn get_listening<'a, T: PgExecutor<'a>>(executor: T, user_id: &Uuid) -> anyhow::Result<Vec<User>> {
    let listenees = sqlx::query_as!(User,"
        WITH data AS (
            SELECT listenee
            FROM listeners
            WHERE listener = $1
        )
        SELECT u.*
        FROM users u
        JOIN data d ON d.listenee = u.user_id
    ",
        user_id
    ).fetch_all(executor).await?;

    Ok(listenees)
}

pub async fn get_post_likes<'a, T: PgExecutor<'a>>(executor: T, post_id: &Uuid) -> anyhow::Result<i64> {
    Ok(sqlx::query!("SELECT COALESCE(COUNT(*), 0) as count FROM post_likes WHERE post_id = $1", &post_id).fetch_one(executor).await?.count.unwrap())
}
