use std::collections::HashSet;

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use mongodb::options::{ClientOptions, Credential, ServerAddress};
use tt::{AreaType, TTClient};
use bruss_data::RoutingType;

static DEFAULT_API_DEFAULT_LIMIT: i64 = 20;
static DEFAULT_API_MAX_RT_AGE: u64 = 0;

fn get_default_api_default_limit() -> i64 { DEFAULT_API_DEFAULT_LIMIT }
fn get_default_api_max_rt_age() -> u64 { DEFAULT_API_MAX_RT_AGE }

#[derive(Serialize, Deserialize, Debug)]
pub struct BrussConfig {
    pub db: DBConfig,
    pub tt: TTConfig,
    pub routing: RoutingConfig,
    pub api: ApiConfig,
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
    pub skip_routing_types: HashSet<RoutingType>,
    #[serde(default)]
    pub deep_trip_check: bool,
    pub parallel_downloads: Option<usize>,
    #[serde(default)]
    pub dry_run: bool,
    pub filter_area: Option<HashSet<u16>>,
    pub filter_area_type: Option<HashSet<AreaType>>,
    pub filter_code: Option<HashSet<String>>,
    pub max_trip_requests: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiConfig {
    pub cors_allowed_origin: Option<String>,
    pub cors_allowed_methods: Option<Vec<String>>,
    pub cors_allowed_headers: Option<Vec<String>>,
    pub cors_allow_credentials: Option<bool>,
    #[serde(default = "get_default_api_default_limit")]
    pub default_limit: i64,
    #[serde(default = "get_default_api_max_rt_age")]
    pub max_rt_age: u64,
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
            .hosts(vec![ServerAddress::Tcp { host: self.host.to_string(), port: self.port }])
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

#[test]
fn test_config() {
    let config = BrussConfig::from_file("bruss.toml")
        .unwrap();
    println!("{:?}", config);
}
