use serde_json::Value;

pub fn print_json_as_table(json_vec: &[Value]) {
    if json_vec.is_empty() {
        println!("No data to display.");
        return;
    }

    let keys: Vec<String> = match json_vec[0].as_object() {
        Some(obj) => obj.keys().cloned().collect(),
        None => vec![],
    };

    if keys.is_empty() {
        println!("No fields to display.");
        return;
    }

    let header = keys
        .iter()
        .map(|k| format!("{:<25}", k))
        .collect::<Vec<String>>()
        .join(" | ");
    
    let line_len = header.len();
    println!("{}", header);
    println!("{}", "-".repeat(line_len));

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
            
            println!("{}", row);
        }
    }
    println!("{}", "-".repeat(line_len));
}
