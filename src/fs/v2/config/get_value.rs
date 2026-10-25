use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn parse_value_as<T: DeserializeOwned>(val: &Value) -> Option<T> {
    serde_json::from_value(val.clone()).ok()
}

pub fn get_value(json_element: &Value, key: &str) -> Option<Value> {
    json_element
        .as_object()
        .and_then(|obj| obj.get(key))
        .cloned()
}
