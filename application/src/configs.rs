use std::{env, net};
use config::{Config, File, FileFormat};
use std::path::Path;
use serde::Deserialize;
use crate::core::errors::AppResult;

const DEFAULT_CONFIG: &str = include_str!("../config/application.yaml");

const APP_CONFIG_PATH: &str = "/home/app/conf/application.yaml";

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: Server,
    // database: Database,
}

impl AppConfig {
    pub fn load() -> AppResult<AppConfig> {
        let config_path_env = env::var("APP_CONFIG_PATH").unwrap_or_else(|_| APP_CONFIG_PATH.into());
        let config = if Path::new(&config_path_env).exists() {
            Config::builder().add_source(File::with_name(&config_path_env).required(false))
        } else {
            Config::builder().add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Yaml).required(false))
        };

        let config = config.build()?;
        let settings: AppConfig = config.try_deserialize()?;
        Ok(settings)
    }
}

impl AppConfig {
    pub fn addrs(&self) -> (String, u16) {
        (self.server.address.clone(), self.server.port)
    }

    pub fn path(&self) -> String {
        self.server.path.clone()
    }
}

#[derive(Debug, Deserialize)]
struct Server {
    /// 服务器的IP地址：`0.0.0.0`/`127.0.0.1`
    #[serde(default = "default_address")]
    pub address: String,
    /// 服务器端口：`80`
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_path")]
    pub path: String,
}

#[derive(Debug, Deserialize)]
struct Database {
    user: String,
    password: String,
    database: String,
}

fn default_address() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_path() -> String {
    "/".to_string()
}