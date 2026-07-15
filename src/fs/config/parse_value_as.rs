use serde::de::DeserializeOwned; 
use serde_json::Value;

pub fn parse_value_as<T: DeserializeOwned>(val: &Value) -> Option<T> {
    serde_json::from_value(val.clone()).ok()
}
