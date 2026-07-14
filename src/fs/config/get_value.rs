use serde_json::Value;

pub fn get_value(json_element: &Value, key: &str) -> Option<Value> {
    json_element
        .as_object()
        .and_then(|obj| obj.get(key))
        .cloned()
}
