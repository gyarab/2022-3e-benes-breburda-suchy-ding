use std::str::FromStr;

use uuid::Uuid;

use crate::models::ClientError;

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
