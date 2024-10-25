use std::sync::Arc;
use sea_orm::{Database, DatabaseConnection};
use common::{env, migration};
use crate::configs::{AppConfig, DataSource};
use crate::core::errors::AppResult;
use crate::core::version::Version;

#[derive(Debug)]
pub struct Context {
    pub config: Arc<AppConfig>,
    pub version: Arc<Version>,
    pub db: Arc<DatabaseConnection>,
}

impl Context {
    pub(crate) async fn new(config: AppConfig) -> AppResult<Context> {
        let config: Arc<AppConfig> = config.into();
        let db: DatabaseConnection = Database::connect(init_pool_opt(&config.data_source)).await?;
        // config
        Ok(Context { config, version: Arc::new(Version::default()), db: db.into() })
    }

    pub async fn run_database_migration(&self) -> AppResult<()> {
        if env::enable_migration() {
            let db = &self.db;
            migration::migrations(db).await?;
        }
        Ok(())
    }
}

/// 初始化连接池各项配置
fn init_pool_opt(source: &DataSource) -> sea_orm::ConnectOptions {
    let mut opt = sea_orm::ConnectOptions::new(&source.url);
    if let Some(max_connections) = source.max_connections {
        opt.max_connections(max_connections);
    }
    if let Some(min_connections) = source.min_connections {
        opt.min_connections(min_connections);
    }
    if let Some(idle_timeout) = source.idle_timeout {
        opt.idle_timeout(idle_timeout);
    }
    if let Some(acquire_timeout) = source.acquire_timeout {
        opt.acquire_timeout(acquire_timeout);
    }
    if let Some(max_lifetime) = source.max_lifetime {
        opt.max_lifetime(max_lifetime);
    }
    opt
}