use serde_json::Value;

use super::TableError;

fn width_of(s: &str) -> usize {
    s.chars().count()
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

pub fn json_as_table(json_vec: &[Value]) -> Result<String, TableError> {
    if json_vec.is_empty() {
        return Err(TableError::Empty);
    }
    let mut keys: Vec<String> = Vec::new();
    for value in json_vec {
        if let Some(obj) = value.as_object() {
            for k in obj.keys() {
                if !keys.iter().any(|existing| existing == k) {
                    keys.push(k.clone());
                }
            }
        }
    }
    if keys.is_empty() {
        return Err(TableError::NoFields);
    }

    let mut widths: Vec<usize> = keys.iter().map(|k| width_of(k)).collect();
    for value in json_vec {
        if let Some(obj) = value.as_object() {
            for (i, k) in keys.iter().enumerate() {
                let cell = obj.get(k).map(value_to_string).unwrap_or_default();
                let w = width_of(&cell);
                if w > widths[i] {
                    widths[i] = w;
                }
            }
        }
    }

    let pad = |s: &str, w: usize| -> String {
        let len = width_of(s);
        if len >= w {
            s.to_string()
        } else {
            let mut out = String::with_capacity(s.len() + (w - len));
            out.push_str(s);
            for _ in 0..(w - len) {
                out.push(' ');
            }
            out
        }
    };

    let header = keys
        .iter()
        .enumerate()
        .map(|(i, k)| pad(k, widths[i]))
        .collect::<Vec<_>>()
        .join(" | ");
    let total_width = header.chars().count();

    let mut out = String::new();
    out.push_str(&header);
    out.push('\n');
    out.push_str(&"-".repeat(total_width));
    out.push('\n');
    for value in json_vec {
        if let Some(obj) = value.as_object() {
            let row = keys
                .iter()
                .enumerate()
                .map(|(i, k)| {
                    let cell = obj
                        .get(k)
                        .map(value_to_string)
                        .unwrap_or_else(|| "N/A".to_string());
                    pad(&cell, widths[i])
                })
                .collect::<Vec<_>>()
                .join(" | ");
            out.push_str(&row);
            out.push('\n');
        }
    }
    out.push_str(&"-".repeat(total_width));
    Ok(out)
}

pub fn print_json_as_table(json_vec: &[Value]) {
    match json_as_table(json_vec) {
        Ok(s) => println!("{s}"),
        Err(TableError::Empty) => println!("No data to display."),
        Err(TableError::NoFields) => println!("No fields to display."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn table_has_headers() {
        let data = vec![json!({"a": 1, "b": "x"}), json!({"a": 22, "b": "yy"})];
        let t = json_as_table(&data).unwrap();
        assert!(t.contains("a"));
        assert!(t.contains("b"));
        assert!(t.contains("22"));
    }

    #[test]
    fn table_empty_err() {
        assert!(json_as_table(&[]).is_err());
    }
}
