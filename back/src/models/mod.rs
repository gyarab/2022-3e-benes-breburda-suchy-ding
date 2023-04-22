use serde::{Deserialize, Serialize};
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
    verify_email,
    password_restore
}

#[derive(Deserialize, Serialize)]
pub struct EmailToken {
    pub user_id: Uuid,
    pub token: String,
    pub token_type: EmailTokenType,
}
