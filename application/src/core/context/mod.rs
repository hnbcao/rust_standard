use std::sync::Arc;
use crate::configs::AppConfig;
use crate::core::errors::AppResult;
use crate::core::version::Version;

pub struct Context {
    pub config: Arc<AppConfig>,
    version: Arc<Version>,
}

impl Context {
    pub(crate) fn new(config: AppConfig) -> AppResult<Context> {
        let config = config.into();
        Ok(Context { config, version: Arc::new(Version::default()) })
    }
}