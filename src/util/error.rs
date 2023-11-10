use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("need environment variable: {0}")]
    BadEnv(#[from] std::env::VarError),

    #[error("sqlx error: {0}")]
    SqlError(#[from] sqlx::Error),

    #[error("format error: {0}")]
    FormatError(#[from] std::fmt::Error),

    #[error("parse int error: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("params error: {0}")]
    ParamsErr(String),
}