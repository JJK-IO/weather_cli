use crate::cache::{read_cache, write_cache};
use crate::types::{IpApiResponse, LocationCache};
use anyhow::Result;
use reqwest::blocking::Client;

const DEFAULT_IP_API_URL: &str = "http://ip-api.com/json/";

pub fn get_location_from_ip(client: &Client, verbose: bool) -> Result<(f64, f64)> {
    get_location_from_ip_with_url(client, DEFAULT_IP_API_URL, verbose)
}

pub fn get_location_from_ip_with_url(
    client: &Client,
    api_url: &str,
    verbose: bool,
) -> Result<(f64, f64)> {
    // Try to read from cache first
    if let Ok(Some(cache)) = read_cache() {
        if verbose {
            println!("Using cached location for IP: {}", cache.ip);
        }
        return Ok((cache.lat, cache.lon));
    }

    // Fetch current IP and location
    if verbose {
        println!("Fetching current location...");
    }
    let cache = fetch_ip_location(client, api_url)?;

    write_cache(&cache)?;
    Ok((cache.lat, cache.lon))
}

pub fn fetch_ip_location(client: &Client, api_url: &str) -> Result<LocationCache> {
    let resp: IpApiResponse = client.get(api_url).send()?.json()?;

    if resp.status != "success" {
        return Err(anyhow::anyhow!(
            "Failed to get location from IP API: {}",
            resp.status
        ));
    }

    Ok(LocationCache {
        ip: resp.query,
        lat: resp.lat,
        lon: resp.lon,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_ip_location_success() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"status":"success","query":"1.2.3.4","lat":40.0,"lon":-74.0}"#,
            )
            .create();

        let client = Client::new();
        let cache = fetch_ip_location(&client, &format!("{}/json", server.url()))
            .expect("ip lookup should succeed");

        assert_eq!(cache.ip, "1.2.3.4");
        assert_eq!(cache.lat, 40.0);
        assert_eq!(cache.lon, -74.0);
    }

    #[test]
    fn fetch_ip_location_failure_status() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"fail","query":"1.2.3.4","lat":0.0,"lon":0.0}"#)
            .create();

        let client = Client::new();
        let err = fetch_ip_location(&client, &format!("{}/json", server.url()))
            .expect_err("failed status should return an error");

        assert!(err
            .to_string()
            .contains("Failed to get location from IP API"));
    }
}
