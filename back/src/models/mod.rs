use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}
