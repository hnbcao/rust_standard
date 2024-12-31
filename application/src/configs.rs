use crate::core::errors::AppResult;
use crate::logging::Logging;
use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::time::Duration;

const DEFAULT_CONFIG: &str = include_str!("../config/application.yaml");

const APP_CONFIG_PATH: &str = "/home/app/conf/application.yaml";

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub data_source: DataSource,
    pub logging: Logging,
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
        self.server.path.clone().trim().trim_end_matches(|c: char| c == '/').to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    /// 服务器的IP地址：`0.0.0.0`/`127.0.0.1`
    #[serde(default = "default_address")]
    pub address: String,
    /// 服务器端口：`80`
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_path")]
    pub path: String,
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.address, self.port)
    }
}

#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
pub struct DataSource {
    /// 数据库连接：`sqlite::memory:`,`sqlite://path/to/db.sqlite?mode=rwc`,`mysql://username:password@host/database?currentSchema=my_schema`
    pub url: String,
    /// 连接池最大连接 (默认为10)
    #[serde(default)]
    pub max_connections: Option<u32>,
    /// 连接池最大连接 (默认为0)
    #[serde(default)]
    pub min_connections: Option<u32>,
    /// 连接的最大空闲时间，以防止网络资源耗尽 (默认为10m)
    #[serde_as(as = "Option<serde_with::DurationSeconds<u64>>")]
    #[serde(default)]
    pub idle_timeout: Option<Duration>,
    /// 设置等待获取连接所花费的最长时间&连接的超时时间 (默认为30s)
    #[serde_as(as = "Option<serde_with::DurationSeconds<u64>>")]
    #[serde(default)]
    pub acquire_timeout: Option<Duration>,
    /// 设置单个连接的最长生命周期 (默认为30m)
    #[serde_as(as = "Option<serde_with::DurationSeconds<u64>>")]
    #[serde(default)]
    pub max_lifetime: Option<Duration>,
}

fn default_address() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_path() -> String {
    "".to_string()
}
