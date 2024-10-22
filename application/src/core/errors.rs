use sea_orm::DbErr;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    StdIo(#[from] std::io::Error),

    #[error("{0}")]
    Config(#[from] config::ConfigError),

    #[error("{0}")]
    Db(#[from] DbErr)
}