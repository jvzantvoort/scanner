use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Invalid target address: {0}")]
    InvalidTarget(String),

    #[error("Invalid port specification: {0}")]
    InvalidPort(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Output error: {0}")]
    OutputError(String),
}

pub type Result<T> = std::result::Result<T, ScannerError>;
