use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("queue service has been closed.")]
    Closed,
    #[error("send data error.")]
    Send,
}
