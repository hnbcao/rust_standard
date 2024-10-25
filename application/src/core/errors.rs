use opentelemetry::metrics::MetricsError;
use opentelemetry::trace::TraceError;
use sea_orm::DbErr;
use thiserror::Error;
use tracing_subscriber::filter::LevelParseError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    ApiRequestParam(String),

    #[error("{0}")]
    ApiRequestParamStr(&'static str),

    #[error("{0}")]
    StdIo(#[from] std::io::Error),

    #[error("{0}")]
    Config(#[from] config::ConfigError),

    #[error("{0}")]
    Db(#[from] DbErr),

    #[error("{0}")]
    Serde(#[from] serde_json::Error),

    #[error("{0}")]
    Metrics(#[from] MetricsError),

    #[error("{0}")]
    Trace(#[from] TraceError),

    #[error("{0}")]
    LevelParse(#[from] LevelParseError),


}