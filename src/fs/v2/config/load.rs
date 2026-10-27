use std::path::Path;

use serde_json::Value;

use super::Config;
use super::ConfigError;
use super::paths::config_path;
use crate::fs::write_atomic;

impl Config {
    pub fn load(app_name: &str, file_name: &str) -> Result<Self, ConfigError> {
        Self::load_path(config_path(app_name, file_name))
    }

    pub fn load_path<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref().to_path_buf();
        let text = std::fs::read_to_string(&path)?;
        let value: Value = serde_json::from_str(&text)?;
        Ok(Self { path, value })
    }

    pub fn load_or_create(
        app_name: &str,
        file_name: &str,
        default: &str,
    ) -> Result<Self, ConfigError> {
        let path = config_path(app_name, file_name);
        if !path.exists() {
            let value: Value = serde_json::from_str(default)?;
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            write_atomic(&path, default)?;
            return Ok(Self { path, value });
        }
        Self::load_path(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn roundtrip_via_file() {
        let dir = std::env::temp_dir().join(format!("musutils_cfg_v2_{}", std::process::id()));
        let file = dir.join("cfg.json");
        let cfg = Config::from_value(json!({"a": 1}));
        crate::fs::write(&file, serde_json::to_vec(&cfg.as_value().clone()).unwrap()).unwrap();
        let loaded = Config::load_path(&file).unwrap();
        assert_eq!(loaded.as_value()["a"], json!(1));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
