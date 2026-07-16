use std::path::PathBuf;

pub async fn download_to(url: String, path: PathBuf) -> std::io::Result<()> {
    let bytes = crate::http::get_file(url).await;
    crate::fs::write(path, bytes)?;
    Ok(())
}
