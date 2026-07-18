pub async fn get_file(url: String) -> std::io::Result<Vec<u8>> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let bytes = response.bytes()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(bytes.to_vec())
}
