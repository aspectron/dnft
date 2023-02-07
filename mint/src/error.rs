use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    String(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
