use super::HttpError;
use super::client::client;

pub async fn get_github_json_str(url: impl Into<String>) -> Result<String, HttpError> {
    let response = client()
        .get(url.into())
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "musutils")
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(HttpError::Status(response.status().as_u16()));
    }
    let value: serde_json::Value = response.json().await?;
    serde_json::to_string_pretty(&value)
        .map_err(|e| HttpError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
}
