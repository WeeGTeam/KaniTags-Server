use byte_unit::{Byte, Unit};
use figment::providers::{Env, Format, Yaml};
use figment::Figment;
use pantsu_domain::common::error::Error;
use serde::Deserialize;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub server_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_url: String,
    pub library_path: PathBuf,
    #[serde(deserialize_with = "parse_byte")]
    pub request_body_limit: Byte,
}

fn parse_byte<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Byte, D::Error> {
    let value: String = String::deserialize(deserializer)?;
    Byte::from_str(value.as_str()).map_err(serde::de::Error::custom)
}

impl ServerConfig {

    pub fn load_config() -> Result<Self, Error> {
        Figment::default()
            .merge(Yaml::file("/etc/pantsu-server/config.yaml"))
            .merge(Yaml::file("/config/config.yaml"))
            .merge(Yaml::file("./config.yaml"))
            .merge(Env::prefixed("PANTSU_SERVER_"))
            .extract::<ServerConfig>()
            .or_else(|_| Err(Error::TodoError()))
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            server_port: 8000,
            db_username: "pantsu_maid".to_string(),
            db_password: "password".to_string(),
            db_url: "localhost:4269".to_string(),
            library_path: PathBuf::from("pantsu_library"),
            request_body_limit: Byte::from_u64_with_unit(25, Unit::MB).unwrap(),
        }
    }
}
