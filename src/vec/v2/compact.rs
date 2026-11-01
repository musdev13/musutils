use serde_json::Value;

pub fn compact_json_value(v: &Value, max_len: usize) -> String {
    let s = match v {
        Value::String(s) => s.clone(),
        Value::Null => "-".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Array(arr) => compact_array(arr),
        Value::Object(obj) => format!("{{{} fields}}", obj.len()),
    };
    truncate(&s, max_len)
}

fn compact_array(arr: &[Value]) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    }
    if arr.iter().all(|v| v.is_string()) {
        return arr
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(", ");
    }
    format!("{} items", arr.len())
}

fn truncate(s: &str, max_len: usize) -> String {
    if max_len == 0 || s.chars().count() <= max_len {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max_len.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn string_passthrough() {
        assert_eq!(compact_json_value(&json!("hello"), 64), "hello");
    }

    #[test]
    fn string_truncated() {
        let out = compact_json_value(&json!("abcdefghij"), 5);
        assert_eq!(out.chars().count(), 5);
        assert!(out.ends_with('…'));
    }

    #[test]
    fn array_of_strings_joined() {
        assert_eq!(compact_json_value(&json!(["a", "b", "c"]), 64), "a, b, c");
    }

    #[test]
    fn array_of_objects_counted() {
        assert_eq!(compact_json_value(&json!([{"a":1},{"b":2}]), 64), "2 items");
    }

    #[test]
    fn object_counted() {
        assert_eq!(compact_json_value(&json!({"a":1,"b":2}), 64), "{2 fields}");
    }

    #[test]
    fn null_becomes_dash() {
        assert_eq!(compact_json_value(&json!(null), 64), "-");
    }
}
