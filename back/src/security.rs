use anyhow::anyhow;
use sqlx::PgPool;
use std::{num::NonZeroU32, future::Future, pin::Pin};
use ring::{
    digest, pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use base64::{Engine as _, engine::general_purpose as base64_coder};

use crate::state::StateWithDb;
use crate::models::User;

#[derive(Clone)]
pub struct PasswordHasherSpec {
    name: String,
    pbkdf2_alg: pbkdf2::Algorithm,
    credential_length: usize,
    pbkdf2_iterations: NonZeroU32,
}

impl From<&str> for PasswordHasherSpec {
    fn from(val: &str) -> Self {
        match val {
            "hmac_sha512_100000" => Self {
                name: "hmac_sha512_100000".to_owned(),
                pbkdf2_alg: pbkdf2::PBKDF2_HMAC_SHA512,
                credential_length: digest::SHA512_OUTPUT_LEN,
                pbkdf2_iterations: NonZeroU32::new(100_000).unwrap(),
            },
            e => panic!("invalid hashing algorithm: {}", val),
        }
    }
}

pub async fn hash(password: &str) -> anyhow::Result<String> {
    let password = password.to_owned();
    async_std::task::spawn_blocking(move || {
        let spec = PasswordHasherSpec::from("hmac_sha512_100000");
        let random = SystemRandom::new();
        let mut out = vec![0u8; spec.credential_length];
        let mut salt = [0u8; 8];
        random.fill(&mut salt)?;
        ring::pbkdf2::derive(
            spec.pbkdf2_alg,
            spec.pbkdf2_iterations,
            &salt,
            password.as_bytes(),
            &mut out,
        );
        Ok(format!(
            "${}${}${}",
            spec.name,
            base64_coder::STANDARD_NO_PAD.encode(&salt),
            base64_coder::STANDARD_NO_PAD.encode(&out)
        ))
    }).await
}

pub async fn verify(password: &str, hash: &str) -> anyhow::Result<bool> {
    let password = password.to_owned();
    let hash = hash.to_owned();
    async_std::task::spawn_blocking(move || {
        let parts: Vec<&str> = hash.split("$").collect();
        if let [_, name, salt, hash] = parts[..4] {
            let spec = PasswordHasherSpec::from(name);
            let salt = base64_coder::STANDARD_NO_PAD.decode(&salt)?;
            let hash = base64_coder::STANDARD_NO_PAD.decode(&hash)?;
            let res = ring::pbkdf2::verify(
                spec.pbkdf2_alg,
                spec.pbkdf2_iterations,
                &salt,
                password.as_bytes(),
                &hash,
            );
            Ok(res.is_ok())
        } else {
            Err(anyhow!("wrong number of parts in hash"))
        }
    }).await
}

pub async fn create_token(bytes: usize) -> anyhow::Result<String> {
    async_std::task::spawn_blocking(move || {
        let rand = SystemRandom::new();
        let mut buf = vec![0u8; bytes];
        rand.fill(&mut buf)?;
        Ok(base64_coder::STANDARD_NO_PAD.encode(buf))
    })
    .await
}

pub async fn check_token(db: &PgPool, token: &str) -> bool {
    let session = sqlx::query_as!(User, "
        SELECT users.*
        FROM users
        JOIN sessions ON users.user_id = sessions.user_id
        WHERE sessions.token = $1
    ", token);
    false
}

fn session_guard<'a, T: Clone + Send + Sync + StateWithDb + 'static>(
    mut request: tide::Request<T>,
    next: tide::Next<'a, T>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let cookie = request.cookie("session_ding");
        if cookie.is_none() || !check_token(request.state().db(), cookie.unwrap().value()).await {
            Ok(tide::Response::new(tide::StatusCode::Unauthorized))
        } else {
            Ok(next.run(request).await)
        }
    })
}
