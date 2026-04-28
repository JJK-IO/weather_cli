use crate::types::{ForecastResponse, HourlyForecastResponse, PointsResponse};
use anyhow::Result;
use reqwest::blocking::Client;

const WEATHER_API_BASE: &str = "https://api.weather.gov";

pub fn get_points(client: &Client, lat: f64, lon: f64) -> Result<PointsResponse> {
    get_points_with_base_url(client, WEATHER_API_BASE, lat, lon)
}

pub fn get_points_with_base_url(
    client: &Client,
    api_base: &str,
    lat: f64,
    lon: f64,
) -> Result<PointsResponse> {
    let points_url = format!("{}/points/{},{}", api_base.trim_end_matches('/'), lat, lon);
    let resp = client
        .get(&points_url)
        .header("User-Agent", "rust-weather-cli")
        .send()?
        .json()?;
    Ok(resp)
}

pub fn get_forecast(client: &Client, url: &str) -> Result<ForecastResponse> {
    let resp = client
        .get(url)
        .header("User-Agent", "rust-weather-cli")
        .send()?
        .json()?;
    Ok(resp)
}

pub fn get_hourly_forecast(client: &Client, url: &str) -> Result<HourlyForecastResponse> {
    let resp = client
        .get(url)
        .header("User-Agent", "rust-weather-cli")
        .send()?
        .json()?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_points_with_base_url_returns_points_response() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/points/39.1,-104.8")
            .match_header("user-agent", "rust-weather-cli")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"properties":{"forecast":"https://x/forecast","forecastHourly":"https://x/hourly"}}"#,
            )
            .create();

        let client = Client::new();
        let points = get_points_with_base_url(&client, &server.url(), 39.1, -104.8)
            .expect("points response should parse");

        assert_eq!(points.properties.forecast, "https://x/forecast");
        assert_eq!(points.properties.forecast_hourly, "https://x/hourly");
    }

    #[test]
    fn get_forecast_returns_forecast_response() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/forecast")
            .match_header("user-agent", "rust-weather-cli")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"properties":{"periods":[{"name":"Today","detailedForecast":"Sunny"}]}}"#,
            )
            .create();

        let client = Client::new();
        let forecast = get_forecast(&client, &format!("{}/forecast", server.url()))
            .expect("forecast response should parse");

        assert_eq!(forecast.properties.periods.len(), 1);
        assert_eq!(forecast.properties.periods[0].name, "Today");
    }

    #[test]
    fn get_hourly_forecast_returns_hourly_response() {
        let mut server = mockito::Server::new();
        let _mock = server
            .mock("GET", "/hourly")
            .match_header("user-agent", "rust-weather-cli")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"properties":{"periods":[{"startTime":"2026-04-28T12:00:00+00:00","temperature":70,"temperatureUnit":"F","shortForecast":"Clear","windSpeed":"5 mph","windDirection":"NW","probabilityOfPrecipitation":{"value":10}}]}}"#,
            )
            .create();

        let client = Client::new();
        let hourly = get_hourly_forecast(&client, &format!("{}/hourly", server.url()))
            .expect("hourly response should parse");

        assert_eq!(hourly.properties.periods.len(), 1);
        assert_eq!(hourly.properties.periods[0].temperature, 70);
    }
}
