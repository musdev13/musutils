use super::HttpError;
use super::get_file::get_file;

pub async fn get_json_str(url: impl Into<String>) -> Result<String, HttpError> {
    let text = String::from_utf8(get_file(url).await?)
        .map_err(|e| HttpError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| HttpError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;
    serde_json::to_string_pretty(&value)
        .map_err(|e| HttpError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
}
