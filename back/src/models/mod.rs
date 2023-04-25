use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgExecutor;
use uuid::Uuid;
mod client_error;
pub use client_error::ClientError;

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub bio: String,
}

impl User {
    pub async fn from_db<'a, T: PgExecutor<'a>>(db: T, user_id: &Uuid) -> anyhow::Result<Option<Self>>{
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE user_id = $1", user_id).fetch_optional(db).await?;
        Ok(user)
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserPub {
    pub user_id: Uuid,
    pub name: String,
    pub bio: String,
}
impl From<User> for UserPub {
    fn from(u: User) -> Self {
        Self {
            user_id: u.user_id,
            name: u.name,
            bio: u.bio,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}
#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "email_token_type")]
#[allow(non_camel_case_types)]
pub enum EmailTokenType {
    password_restore
}

#[derive(Deserialize, Serialize)]
pub struct EmailToken {
    pub user_id: Uuid,
    pub token: String,
    pub token_type: EmailTokenType,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created: DateTime<Utc>,
}
impl Post {
    pub async fn from_db<'a, T: PgExecutor<'a>>(db: T, item_id: &Uuid) -> anyhow::Result<Option<Self>>{
        let item = sqlx::query_as!(Self, "SELECT * FROM posts WHERE post_id = $1", item_id).fetch_optional(db).await?;
        Ok(item)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostPub {
    pub post_id : Uuid,
    pub author_id: Uuid,
    pub created: DateTime<Utc>,
}
impl From<Post> for PostPub {
    fn from(p: Post) -> Self {
        Self {
            post_id: p.post_id,
            author_id: p.author_id,
            created: p.created,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostPubExt {
    pub post_id : Uuid,
    pub author_id: Uuid,
    pub created: DateTime<Utc>,
    pub likes: i64,
    pub comments: i64,
    pub liked: bool,
    pub saved: bool,
}

impl PostPubExt {
    pub fn new(p: Post, likes: i64, comments: i64, liked: bool, saved: bool) -> Self {
        Self {
            post_id: p.post_id,
            author_id: p.author_id,
            created: p.created,
            likes,
            comments,
            liked,
            saved
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub comment_id : Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created: DateTime<Utc>,
}
