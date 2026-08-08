use serde_json::Value;

pub async fn get_github_json_str(url: String) -> std::io::Result<String> {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "rml")
        .send()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let value: Value = response
        .json()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    serde_json::to_string_pretty(&value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
