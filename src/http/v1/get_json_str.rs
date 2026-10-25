use serde_json::Value;

pub async fn get_json_str(url: String) -> std::io::Result<String> {
    let response = reqwest::get(url).await.map_err(std::io::Error::other)?;

    let value: Value = response.json().await.map_err(std::io::Error::other)?;

    serde_json::to_string_pretty(&value).map_err(std::io::Error::other)
}
