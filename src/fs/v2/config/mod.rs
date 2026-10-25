use std::path::{Path, PathBuf};

use serde_json::Value;

mod error;
mod file;
mod get;
mod get_value;
mod load;
mod paths;
mod save;
mod set;

pub use error::ConfigError;
pub use file::{get, rewrite};
pub use get_value::{get_value, parse_value_as};
pub use paths::{config_dir, config_path};

pub struct Config {
    pub(crate) path: PathBuf,
    pub(crate) value: Value,
}

impl Config {
    pub fn from_value(value: Value) -> Self {
        Self {
            path: PathBuf::new(),
            value,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn as_value(&self) -> &Value {
        &self.value
    }
}
