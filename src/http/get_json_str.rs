use serde_json::Value;

pub async fn get_json_str(url: String) -> std::io::Result<String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let value: Value = response.json()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    serde_json::to_string_pretty(&value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
