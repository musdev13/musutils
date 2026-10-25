use serde_json::Value;

pub async fn get_github_json_str(url: String) -> std::io::Result<String> {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "rml")
        .send()
        .await
        .map_err(std::io::Error::other)?;

    let value: Value = response.json().await.map_err(std::io::Error::other)?;

    serde_json::to_string_pretty(&value).map_err(std::io::Error::other)
}
