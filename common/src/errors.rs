use sea_orm::DbErr;
use thiserror::Error;

pub type CommonResult<T> = Result<T, CommonError>;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("{0}")]
    Db(#[from] DbErr)
}