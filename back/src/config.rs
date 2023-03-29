use anyhow::{anyhow, Result};
use async_std::{fs::File, io::ReadExt};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub async fn from_file(file: &mut File) -> Result<Config> {
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        match toml::from_str::<Config>(&buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("failed to parse config file: {}", e)),
        }
    }
}
