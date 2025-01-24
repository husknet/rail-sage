use maxminddb::Reader;
use serde::Deserialize;
use std::net::IpAddr;

// Define the structure for MaxMind's country-level response
#[derive(Debug, Deserialize)]
pub struct GeoLocation {
    pub country: Option<Country>,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub names: Option<std::collections::HashMap<String, String>>,
}

/// Resolves the country name for a given IP address using the GeoLite2-Country.mmdb database.
///
/// # Arguments
/// - `ip`: The IP address to look up.
///
/// # Returns
/// - `Ok(String)`: The name of the country, if resolved successfully.
/// - `Err`: If there was an error reading the database or looking up the IP.
pub fn get_country(ip: &str) -> Result<String, maxminddb::MaxMindDBError> {
    // Open the GeoLite2-Country database
    let reader = Reader::open_readfile("static/GeoLite2-Country.mmdb")?;
    
    // Parse the IP address
    let ip: IpAddr = ip.parse().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    
    // Look up the IP address in the database
    let location: GeoLocation = reader.lookup(ip)?;

    // Extract the English name of the country (or return "Unknown" if not found)
    let country = location
        .country
        .and_then(|c| c.names)
        .and_then(|n| n.get("en"))
        .cloned()
        .unwrap_or("Unknown".to_string());

    Ok(country)
}
