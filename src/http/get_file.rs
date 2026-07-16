pub async fn get_file(url: String) -> Vec<u8> {
    let response = reqwest::get(url)
        .await
        .expect("Failed to send request")
        .bytes()
        .await
        .expect("Failed to get bytes from response");

    response.to_vec()
}
