use std::path::Path;

use super::HttpError;
use super::get_file::get_file;

pub async fn download_to(url: impl Into<String>, path: impl AsRef<Path>) -> Result<(), HttpError> {
    let bytes = get_file(url).await?;
    let path = path.as_ref();
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(path, bytes).await?;
    Ok(())
}
