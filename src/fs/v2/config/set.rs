use serde_json::Value;

use super::Config;
use super::ConfigError;

impl Config {
    pub fn set(&mut self, key_path: &str, new_value: Value) -> Result<(), ConfigError> {
        if key_path.is_empty() {
            return Err(ConfigError::InvalidKeyPath(key_path.to_string()));
        }
        let segments: Vec<&str> = key_path.split('.').collect();
        if segments.iter().any(|s| s.is_empty()) {
            return Err(ConfigError::InvalidKeyPath(key_path.to_string()));
        }
        if !self.value.is_object() {
            self.value = Value::Object(serde_json::Map::new());
        }

        let mut current = &mut self.value;
        for (i, seg) in segments.iter().enumerate() {
            let obj = current
                .as_object_mut()
                .ok_or_else(|| ConfigError::InvalidKeyPath(key_path.to_string()))?;
            if i == segments.len() - 1 {
                obj.insert((*seg).to_string(), new_value);
                return Ok(());
            }
            let entry = obj
                .entry((*seg).to_string())
                .or_insert_with(|| Value::Object(serde_json::Map::new()));
            if !entry.is_object() {
                return Err(ConfigError::InvalidKeyPath(key_path.to_string()));
            }
            current = entry;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn set_creates_path() {
        let mut cfg = Config::from_value(json!({}));
        cfg.set("a.b.c", json!(1)).unwrap();
        assert_eq!(cfg.get_as::<i32>("a.b.c").unwrap(), 1);
    }
}
