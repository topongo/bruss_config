use std::collections::HashSet;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use mongodb::options::{ClientOptions, Credential, ServerAddress};
use tt::TTClient;
use bruss_data::RoutingType;

#[derive(Serialize, Deserialize, Debug)]
pub struct BrussConfig {
    pub db: DBConfig,
    pub tt: TTConfig,
    pub routing: RoutingConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBConfig {
    host: String,
    db: String,
    user: String,
    password: String,
    port: Option<u16>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoutingConfig {
    pub host: String,
    pub port: Option<u16>,
    pub url_bus: String,
    pub url_rail: String,
    #[serde(default = "get_true")]
    pub exit_on_err: bool,
    pub get_trips: bool,
    #[serde(default)]
    pub skip_routing_types: HashSet<RoutingType>
}

fn get_true() -> bool { true }

#[derive(Debug)]
pub enum ParseError {
    IO(std::io::Error),
    Decode(toml::de::Error)
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::IO(value)
    }
}

impl From<toml::de::Error> for ParseError {
    fn from(value: toml::de::Error) -> Self {
        ParseError::Decode(value)
    }
}

impl BrussConfig {
    pub fn from_file(path: &str) -> Result<BrussConfig, ParseError> {
        // print pwd
        let file = std::fs::read_to_string(path)?;
        match toml::from_str(&file) {
            Ok(x) => Ok(x),
            Err(y) => Err(ParseError::Decode(y))
        }
    }
}

impl DBConfig {
    pub fn gen_mongodb_options(&self) -> ClientOptions {
        ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp { host: self.host.to_string(), port: self.port.clone() }])
            .credential(Credential::builder()
                .username(self.user.to_string())
                .password(self.password.to_string())
                .build())
            .build()
    }

    pub fn get_db(&self) -> &str {
        &self.db
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TTConfig {
    secret: String,
    base_url: String
}

impl TTConfig {
    pub fn client(&self) -> TTClient {
        TTClient::new(self.base_url.clone(), self.secret.clone())
    }
}

lazy_static! {
    pub static ref CONFIGS: BrussConfig = BrussConfig::from_file("bruss.toml").expect("!!cannot load static configs");
}
