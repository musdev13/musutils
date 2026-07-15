pub async fn get_json_str(url: String) -> String{
    serde_json::to_string_pretty(
    reqwest::get(url)
        .await
        .expect("Failed to send request")
        .json()
        .await
        .expect("Failed to parse JSON response");
    )
}
