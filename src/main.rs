use anyhow::Result;
use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Parser)]
#[command(name = "weather")]
#[command(about = "CLI to get weather from weather.gov", long_about = None)]
struct Args {
    /// Latitude
    #[arg(short = 'a', long)]
    lat: f64,

    /// Longitude
    #[arg(short = 'o', long)]
    lon: f64,
}

#[derive(Debug, Deserialize)]
struct PointsResponse {
    properties: PointProperties,
}

#[derive(Debug, Deserialize)]
struct PointProperties {
    forecast: String,
    #[serde(rename = "forecastHourly")]
    forecast_hourly: String,
    // #[serde(rename = "gridId")]
    // grid_id: String,
    // #[serde(rename = "gridX")]
    // grid_x: i32,
    // #[serde(rename = "gridY")]
    // grid_y: i32,
}

#[derive(Debug, Deserialize)]
struct ForecastResponse {
    properties: ForecastProperties,
}

#[derive(Debug, Deserialize)]
struct ForecastProperties {
    periods: Vec<ForecastPeriod>,
}

#[derive(Debug, Deserialize)]
struct ForecastPeriod {
    name: String,
    #[serde(rename = "detailedForecast")]
    detailed_forecast: String,
    // temperature: i32,
    // #[serde(rename = "temperatureUnit")]
    // temperature_unit: String,
    // #[serde(rename = "windSpeed")]
    // wind_speed: String,
    // #[serde(rename = "windDirection")]
    // wind_direction: String,
}

#[derive(Debug, Deserialize)]
struct HourlyForecastResponse {
    properties: HourlyForecastProperties,
}

#[derive(Debug, Deserialize)]
struct HourlyForecastProperties {
    periods: Vec<HourlyForecastPeriod>,
}

#[derive(Debug, Deserialize)]
struct HourlyForecastPeriod {
    #[serde(rename = "startTime")]
    start_time: String,
    temperature: i32,
    #[serde(rename = "temperatureUnit")]
    temperature_unit: String,
    #[serde(rename = "shortForecast")]
    short_forecast: String,
    #[serde(rename = "windSpeed")]
    wind_speed: String,
    #[serde(rename = "windDirection")]
    wind_direction: String,
    #[serde(rename = "probabilityOfPrecipitation")]
    probability_of_precipitation: PrecipitationValue,
}

#[derive(Debug, Deserialize)]
struct PrecipitationValue {
    value: Option<i32>,
}

#[derive(Tabled)]
struct HourlyTableForecast {
    time: String,
    temperature: String,
    precip: String,
    wind: String,
    forecast: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new();

    // Get gridpoint info
    let points_url = format!("https://api.weather.gov/points/{},{}", args.lat, args.lon);
    let points_resp: PointsResponse = client.get(&points_url)
        .header("User-Agent", "rust-weather-cli")
        .send()?
        .json()?;

    // Get regular forecast
    let forecast_resp: ForecastResponse = client.get(&points_resp.properties.forecast)
        .header("User-Agent", "rust-weather-cli")
        .send()?
        .json()?;

    // Print first forecast in regular format (today and tonight)
    println!("Weather Summary:");
    for period in &forecast_resp.properties.periods[..2] {
        println!("{}: {}", period.name, period.detailed_forecast);
    }
    
    // Get hourly forecast
    let hourly_forecast_resp: HourlyForecastResponse = client.get(&points_resp.properties.forecast_hourly)
        .header("User-Agent", "rust-weather-cli")
        .send()?
        .json()?;
    
    println!("\n12-Hour Hourly Forecast:");
    
    // Format time strings from ISO format
    let table_data: Vec<HourlyTableForecast> = hourly_forecast_resp.properties.periods
        .iter()
        .take(12) // Take next 12 hours
        .map(|period| {
            // Parse the ISO datetime and format it as a readable time
            let datetime = period.start_time.clone();
            let time = if let Some(t) = datetime.split('T').nth(1) {
                t.split('-').next().unwrap_or(&datetime).split('+').next().unwrap_or(&datetime)
            } else {
                &datetime
            };
            
            // Format precipitation chance
            let precip_chance = match period.probability_of_precipitation.value {
                Some(value) => format!("{}%", value),
                None => "N/A".to_string(),
            };
            
            HourlyTableForecast {
                time: time.to_string(),
                temperature: format!("{}°{}", period.temperature, period.temperature_unit),
                precip: precip_chance,
                wind: format!("{} {}", period.wind_speed, period.wind_direction),
                forecast: period.short_forecast.clone(),
            }
        })
        .collect();
    
    let table = Table::new(table_data).to_string();
    println!("{}", table);

    Ok(())
}
