use serde_json::Value;

pub async fn get_json_str(url: String) -> String {
    let response: Value = reqwest::get(url)
        .await
        .expect("Failed to send request")
        .json()
        .await
        .expect("Failed to parse JSON response");

    serde_json::to_string_pretty(&response)
        .expect("Failed to serialize JSON to string")
}
