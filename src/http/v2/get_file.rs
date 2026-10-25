use super::HttpError;
use super::client::client;

pub async fn get_file(url: impl Into<String>) -> Result<Vec<u8>, HttpError> {
    let response = client().get(url.into()).send().await?;
    if !response.status().is_success() {
        return Err(HttpError::Status(response.status().as_u16()));
    }
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}
