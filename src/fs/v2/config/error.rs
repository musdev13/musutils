use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(serde_json::Error),
    KeyNotFound(String),
    TypeMismatch { key: String, expected: &'static str },
    InvalidKeyPath(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "io error: {e}"),
            ConfigError::Parse(e) => write!(f, "parse error: {e}"),
            ConfigError::KeyNotFound(k) => write!(f, "key not found: {k}"),
            ConfigError::TypeMismatch { key, expected } => {
                write!(f, "type mismatch for key `{key}`, expected {expected}")
            }
            ConfigError::InvalidKeyPath(k) => write!(f, "invalid key path: {k}"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            ConfigError::Parse(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(e: serde_json::Error) -> Self {
        ConfigError::Parse(e)
    }
}
