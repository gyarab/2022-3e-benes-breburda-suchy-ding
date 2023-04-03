use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tide::prelude::*;
use anyhow::anyhow;
use std::fmt;
use std::error::Error;

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserPub {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
}
impl From<User> for UserPub {
    fn from(u: User) -> Self {
        Self {
            user_id: u.user_id,
            name: u.name,
            email: u.email,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Session {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}

#[derive(Debug)]
pub struct ClientError {
    pub status_code: u16,
    pub code: String,
    pub message: String,
}

impl ClientError {
    pub fn new(status_code: u16, code: &str, message: &str) -> Self {
        Self {
            status_code,
            code: code.to_owned(),
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.status_code, self.code, self.message)
    }
}

impl Error for ClientError {}
