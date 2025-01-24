use reqwest::Client;
use serde_json::json;

/// Calls the Python API to determine if a request comes from a bot.
///
/// # Arguments
/// - `ip`: The IP address of the user making the request.
/// - `user_agent`: The User-Agent header from the user's request.
///
/// # Returns
/// - `Ok(true)`: If the request is flagged as coming from a bot.
/// - `Ok(false)`: If the request is not flagged as coming from a bot.
/// - `Err`: If there was an error communicating with the API.
pub async fn detect_bot(ip: &str, user_agent: &str) -> Result<bool, reqwest::Error> {
    let client = Client::new();

    // Construct the API request
    let response = client
        .post("https://py-detect.vercel.app/api/detect_bot") // Replace with your Python API endpoint
        .json(&json!({
            "user_agent": user_agent,
            "ip": ip
        }))
        .send()
        .await?;

    // Parse the response as JSON
    let data = response.json::<serde_json::Value>().await?;

    // Extract the "is_bot" value from the API response
    Ok(data["is_bot"].as_bool().unwrap_or(false))
}
