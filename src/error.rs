use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum R03bustError {
    Io(std::io::Error),
    StorageMissing,
    LedgerMissing(PathBuf),
    LogsMissing(PathBuf),
    EmptyCommand,
    Usage(String),
}

impl fmt::Display for R03bustError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::StorageMissing => write!(
                formatter,
                "r03bust storage is missing; run `r03bust init` first"
            ),
            Self::LedgerMissing(path) => {
                write!(formatter, "ledger is missing at {}", path.display())
            }
            Self::LogsMissing(path) => {
                write!(formatter, "logs directory is missing at {}", path.display())
            }
            Self::EmptyCommand => write!(formatter, "no command was provided after `run --`"),
            Self::Usage(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for R03bustError {}

impl From<std::io::Error> for R03bustError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, R03bustError>;
