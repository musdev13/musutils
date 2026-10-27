use serde_json::Value;

pub(crate) fn render_json_as_table(json_vec: &[Value]) -> String {
    if json_vec.is_empty() {
        return "No data to display.".to_string();
    }

    let keys: Vec<String> = match json_vec[0].as_object() {
        Some(obj) => obj.keys().cloned().collect(),
        None => vec![],
    };

    if keys.is_empty() {
        return "No fields to display.".to_string();
    }

    let header = keys
        .iter()
        .map(|k| format!("{:<25}", k))
        .collect::<Vec<String>>()
        .join(" | ");

    let line_len = header.chars().count();
    let sep = "-".repeat(line_len);

    let mut out = String::new();
    out.push_str(&header);
    out.push('\n');
    out.push_str(&sep);
    out.push('\n');

    for v in json_vec {
        if let Some(obj) = v.as_object() {
            let row = keys
                .iter()
                .map(|k| {
                    let val_str = match obj.get(k) {
                        Some(Value::String(s)) => s.clone(),
                        Some(other) => other.to_string(),
                        None => "N/A".to_string(),
                    };
                    format!("{:<25}", val_str)
                })
                .collect::<Vec<String>>()
                .join(" | ");
            out.push_str(&row);
            out.push('\n');
        }
    }
    out.push_str(&sep);
    out
}

pub fn print_json_as_table(json_vec: &[Value]) {
    println!("{}", render_json_as_table(json_vec));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn empty_returns_message() {
        assert_eq!(render_json_as_table(&[]), "No data to display.");
    }

    #[test]
    fn no_object_keys_message() {
        let data = vec![json!("hello")];
        assert_eq!(render_json_as_table(&data), "No fields to display.");
    }

    #[test]
    fn ascii_header_separator_matches_width() {
        let data = vec![json!({"a": 1}), json!({"a": 22})];
        let out = render_json_as_table(&data);
        let mut lines = out.lines();
        let header = lines.next().unwrap();
        let sep = lines.next().unwrap();
        assert_eq!(header.chars().count(), sep.chars().count());
    }

    #[test]
    fn unicode_header_separator_matches_char_width() {
        let data = vec![json!({"имя": "x"}), json!({"имя": "y"})];
        let out = render_json_as_table(&data);
        let mut lines = out.lines();
        let header = lines.next().unwrap();
        let sep = lines.next().unwrap();
        assert_eq!(header.chars().count(), sep.chars().count());
        assert!(header.contains("имя"));
    }

    #[test]
    fn missing_key_renders_na() {
        let data = vec![json!({"a": 1, "b": 2}), json!({"a": 3})];
        let out = render_json_as_table(&data);
        assert!(out.contains("N/A"));
    }

    #[test]
    fn string_and_non_string_values_rendered() {
        let data = vec![json!({"s": "hi", "n": 5, "b": true, "z": null})];
        let out = render_json_as_table(&data);
        assert!(out.contains("hi"));
        assert!(out.contains('5'));
        assert!(out.contains("true"));
        assert!(out.contains("null"));
    }

    #[test]
    fn non_object_rows_skipped() {
        let data = vec![json!({"a": 1}), json!("skip"), json!({"a": 2})];
        let out = render_json_as_table(&data);
        assert!(out.contains('1'));
        assert!(out.contains('2'));
        assert!(!out.contains("skip"));
    }
}
