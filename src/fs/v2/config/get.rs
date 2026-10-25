use serde::de::DeserializeOwned;
use serde_json::Value;

use super::Config;
use super::ConfigError;

impl Config {
    pub fn get(&self, key_path: &str) -> Option<&Value> {
        let mut current = &self.value;
        for segment in key_path.split('.') {
            if segment.is_empty() {
                return None;
            }
            current = current.as_object()?.get(segment)?;
        }
        Some(current)
    }

    pub fn get_as<T: DeserializeOwned>(&self, key_path: &str) -> Result<T, ConfigError> {
        let value = self
            .get(key_path)
            .ok_or_else(|| ConfigError::KeyNotFound(key_path.to_string()))?;
        serde_json::from_value(value.clone()).map_err(|_| ConfigError::TypeMismatch {
            key: key_path.to_string(),
            expected: std::any::type_name::<T>(),
        })
    }

    pub fn get_or<T: DeserializeOwned>(&self, key_path: &str, default: T) -> T {
        self.get_as::<T>(key_path).unwrap_or(default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn nested_get() {
        let cfg = Config::from_value(json!({"a": {"b": {"c": 42}}}));
        assert_eq!(cfg.get_as::<i32>("a.b.c").unwrap(), 42);
    }

    #[test]
    fn missing_key() {
        let cfg = Config::from_value(json!({}));
        assert!(matches!(
            cfg.get_as::<i32>("x"),
            Err(ConfigError::KeyNotFound(_))
        ));
    }
}
