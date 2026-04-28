use anyhow::Result;
use clap::Parser;
use reqwest::blocking::Client;
use weather_cli::{display, location, weather};

#[derive(Parser)]
#[command(name = "weather")]
#[command(about = "CLI to get weather from weather.gov", long_about = None)]
struct Args {
    /// Latitude
    #[arg(short = 'a', long)]
    lat: Option<f64>,

    /// Longitude
    #[arg(short = 'o', long)]
    lon: Option<f64>,

    /// Show verbose diagnostics (IP/location resolution details)
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new();

    // Get latitude and longitude
    let (lat, lon) = match (args.lat, args.lon) {
        (Some(lat), Some(lon)) => (lat, lon),
        _ => {
            // If lat/lon not provided, fetch from IP
            location::get_location_from_ip(&client, args.verbose)?
        }
    };

    if args.verbose {
        println!("Using location: lat={}, lon={}\n", lat, lon);
    }

    // Get gridpoint info
    let points_resp = weather::get_points(&client, lat, lon)?;

    display::print_location_header(&points_resp.properties);

    // Get regular forecast
    let forecast_resp = weather::get_forecast(&client, &points_resp.properties.forecast)?;

    // Print first forecast in regular format (today and tonight)
    println!("Weather Summary:");
    for period in forecast_resp.properties.periods.iter().take(2) {
        println!("{}: {}", period.name, period.detailed_forecast);
    }

    println!("\nNext 7 Days:");
    let seven_day: Vec<display::DailyTableForecast> =
        display::build_seven_day_forecast(&forecast_resp.properties.periods);

    display::print_daily_forecast(&seven_day);

    // Get hourly forecast
    let hourly_forecast_resp =
        weather::get_hourly_forecast(&client, &points_resp.properties.forecast_hourly)?;

    println!("\n12-Hour Hourly Forecast:");

    // Format and display hourly forecast
    let table_data: Vec<display::HourlyTableForecast> = hourly_forecast_resp
        .properties
        .periods
        .iter()
        .take(12)
        .map(|period| display::format_hourly_forecast(period))
        .collect();

    display::print_hourly_forecast(&table_data);

    Ok(())
}
