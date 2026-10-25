use std::io;
use std::path::Path;

use super::Config;
use super::ConfigError;
use crate::fs::write_atomic;

impl Config {
    pub fn save(&self) -> Result<(), ConfigError> {
        if self.path.as_os_str().is_empty() {
            return Err(ConfigError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "config has no path; use save_to",
            )));
        }
        let text = serde_json::to_string_pretty(&self.value)?;
        write_atomic(&self.path, text)?;
        Ok(())
    }

    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let text = serde_json::to_string_pretty(&self.value)?;
        write_atomic(path, text)?;
        Ok(())
    }
}
