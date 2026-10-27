pub async fn get_file(url: String) -> std::io::Result<Vec<u8>> {
    let response = reqwest::get(url).await.map_err(std::io::Error::other)?;

    let bytes = response.bytes().await.map_err(std::io::Error::other)?;

    Ok(bytes.to_vec())
}
