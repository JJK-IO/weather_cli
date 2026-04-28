use crate::types::{ForecastPeriod, HourlyForecastPeriod, PointProperties};
use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct HourlyTableForecast {
    pub time: String,
    pub icon: String,
    pub temperature: String,
    pub precip: String,
    pub humidity: String,
    pub dewpoint: String,
    pub wind: String,
    pub forecast: String,
}

#[derive(Tabled)]
pub struct DailyTableForecast {
    pub day: String,
    pub icon: String,
    pub high_low: String,
    pub precip: String,
    pub wind: String,
    pub forecast: String,
}

fn extract_time(datetime: &str) -> String {
    if let Some(t) = datetime.split('T').nth(1) {
        t.split('-')
            .next()
            .unwrap_or(datetime)
            .split('+')
            .next()
            .unwrap_or(datetime)
            .to_string()
    } else {
        datetime.to_string()
    }
}

fn format_measurement(value: Option<f64>, unit_code: Option<&str>) -> String {
    match value {
        Some(v) => {
            let unit = match unit_code {
                Some(u) if u.ends_with("degC") => "C",
                Some(u) if u.ends_with("degF") => "F",
                Some(u) if u.ends_with("percent") => "%",
                _ => "",
            };
            if unit.is_empty() {
                format!("{v:.1}")
            } else if unit == "%" {
                format!("{v:.0}%")
            } else {
                format!("{v:.1} {unit}")
            }
        }
        None => "N/A".to_string(),
    }
}

fn convert_temp(value: f64, from_unit_code: Option<&str>, to_unit: &str) -> Option<f64> {
    let from = match from_unit_code {
        Some(code) if code.ends_with("degC") => "C",
        Some(code) if code.ends_with("degF") => "F",
        _ => return None,
    };

    match (from, to_unit) {
        ("C", "C") | ("F", "F") => Some(value),
        ("C", "F") => Some((value * 9.0 / 5.0) + 32.0),
        ("F", "C") => Some((value - 32.0) * 5.0 / 9.0),
        _ => None,
    }
}

fn format_dewpoint(value: Option<f64>, from_unit_code: Option<&str>, temp_unit: &str) -> String {
    match value {
        Some(v) => {
            if let Some(converted) = convert_temp(v, from_unit_code, temp_unit) {
                format!("{converted:.1} {temp_unit}")
            } else {
                format_measurement(Some(v), from_unit_code)
            }
        }
        None => "N/A".to_string(),
    }
}

fn icon_code(icon_url: Option<&str>, is_daytime: Option<bool>) -> String {
    match icon_url {
        Some(url) => {
            let last = url.rsplit('/').next().unwrap_or(url);
            let code = last
                .split(',')
                .next()
                .unwrap_or(last)
                .split('?')
                .next()
                .unwrap_or(last)
                .to_lowercase();
            let day = is_daytime.unwrap_or(true);

            if code.contains("tornado") || code.contains("hurricane") || code.contains("tropical") {
                "🌪".to_string()
            } else if code.contains("tsra") || code.contains("lightning") {
                "⛈".to_string()
            } else if code.contains("snow") || code.contains("blizzard") {
                "🌨".to_string()
            } else if code.contains("fzra") || code.contains("sleet") || code.contains("ice") {
                "🧊".to_string()
            } else if code.contains("rain") || code.contains("showers") || code.contains("drizzle") {
                "🌧".to_string()
            } else if code.contains("fog")
                || code.contains("haze")
                || code.contains("smoke")
                || code.contains("dust")
            {
                "🌫".to_string()
            } else if code.contains("wind") {
                "💨".to_string()
            } else if code.contains("hot") {
                "🔥".to_string()
            } else if code.contains("cold") {
                "🥶".to_string()
            } else if code.contains("ovc") || code.contains("bkn") {
                "☁".to_string()
            } else if code.contains("few") || code.contains("sct") {
                if day {
                    "🌤".to_string()
                } else {
                    "☁".to_string()
                }
            } else if code.contains("skc") || code.contains("clr") || code.contains("clear") {
                if day {
                    "☀".to_string()
                } else {
                    "🌙".to_string()
                }
            } else {
                "❔".to_string()
            }
        }
        None => "❔".to_string(),
    }
}

fn daily_icon(period: &ForecastPeriod) -> String {
    let text = period
        .short_forecast
        .as_deref()
        .unwrap_or(&period.detailed_forecast)
        .to_lowercase();

    if text.contains("sunny") || text.contains("clear") {
        return if period.is_daytime.unwrap_or(true) {
            "☀".to_string()
        } else {
            "🌙".to_string()
        };
    }

    icon_code(period.icon.as_deref(), period.is_daytime)
}

pub fn print_location_header(points: &PointProperties) {
    let location = points
        .relative_location
        .as_ref()
        .map(|loc| format!("{}, {}", loc.properties.city, loc.properties.state))
        .unwrap_or_else(|| "Unknown location".to_string());
    let timezone = points.time_zone.as_deref().unwrap_or("Unknown timezone");
    let radar = points.radar_station.as_deref().unwrap_or("Unknown radar");

    println!("Location: {} | Timezone: {} | Radar: {}", location, timezone, radar);
}

pub fn format_hourly_forecast(period: &HourlyForecastPeriod) -> HourlyTableForecast {
    let time = extract_time(&period.start_time);

    let precip_chance = match period.probability_of_precipitation.value {
        Some(value) => format!("{}%", value),
        None => "N/A".to_string(),
    };

    let humidity = format_measurement(
        period.relative_humidity.as_ref().and_then(|v| v.value),
        period
            .relative_humidity
            .as_ref()
            .and_then(|v| v.unit_code.as_deref()),
    );

    let dewpoint = format_dewpoint(
        period.dewpoint.as_ref().and_then(|v| v.value),
        period.dewpoint.as_ref().and_then(|v| v.unit_code.as_deref()),
        &period.temperature_unit,
    );

    HourlyTableForecast {
        time,
        icon: icon_code(period.icon.as_deref(), period.is_daytime),
        temperature: format!("{}°{}", period.temperature, period.temperature_unit),
        precip: precip_chance,
        humidity,
        dewpoint,
        wind: format!("{} {}", period.wind_speed, period.wind_direction),
        forecast: period.short_forecast.clone(),
    }
}

pub fn format_daily_forecast(period: &ForecastPeriod) -> DailyTableForecast {
    let precip = period
        .probability_of_precipitation
        .as_ref()
        .and_then(|v| v.value)
        .map(|v| format!("{}%", v))
        .unwrap_or_else(|| "N/A".to_string());

    let temp = match (period.temperature, period.temperature_unit.as_deref()) {
        (Some(value), Some(unit)) => format!("{}°{}", value, unit),
        _ => "N/A".to_string(),
    };

    let wind = match (period.wind_speed.as_deref(), period.wind_direction.as_deref()) {
        (Some(speed), Some(dir)) => format!("{} {}", speed, dir),
        (Some(speed), None) => speed.to_string(),
        _ => "N/A".to_string(),
    };

    DailyTableForecast {
        day: period.name.clone(),
        icon: daily_icon(period),
        high_low: temp,
        precip,
        wind,
        forecast: period
            .short_forecast
            .clone()
            .unwrap_or_else(|| period.detailed_forecast.clone()),
    }
}

pub fn build_seven_day_forecast(periods: &[ForecastPeriod]) -> Vec<DailyTableForecast> {
    let mut rows: Vec<DailyTableForecast> = Vec::new();

    for day_period in periods
        .iter()
        .filter(|p| p.is_daytime.unwrap_or(false))
        .take(7)
    {
        let night_period = periods
            .iter()
            .skip_while(|p| p.name != day_period.name)
            .skip(1)
            .find(|p| !p.is_daytime.unwrap_or(true));

        let high = match (day_period.temperature, day_period.temperature_unit.as_deref()) {
            (Some(value), Some(unit)) => format!("{}°{}", value, unit),
            _ => "N/A".to_string(),
        };

        let low = match night_period.and_then(|p| p.temperature).zip(
            night_period.and_then(|p| p.temperature_unit.as_deref()),
        ) {
            Some((value, unit)) => format!("{}°{}", value, unit),
            None => "N/A".to_string(),
        };

        let day_precip = day_period
            .probability_of_precipitation
            .as_ref()
            .and_then(|v| v.value)
            .unwrap_or(0);
        let night_precip = night_period
            .and_then(|p| p.probability_of_precipitation.as_ref())
            .and_then(|v| v.value)
            .unwrap_or(0);
        let precip = format!("{}%", day_precip.max(night_precip));

        let wind = match (day_period.wind_speed.as_deref(), day_period.wind_direction.as_deref()) {
            (Some(speed), Some(dir)) => format!("{} {}", speed, dir),
            (Some(speed), None) => speed.to_string(),
            _ => "N/A".to_string(),
        };

        rows.push(DailyTableForecast {
            day: day_period.name.clone(),
            icon: daily_icon(day_period),
            high_low: format!("{}/{}", high, low),
            precip,
            wind,
            forecast: day_period
                .short_forecast
                .clone()
                .unwrap_or_else(|| day_period.detailed_forecast.clone()),
        });
    }

    rows
}

pub fn print_hourly_forecast(periods: &[HourlyTableForecast]) {
    let table = Table::new(periods).to_string();
    println!("{}", table);
}

pub fn print_daily_forecast(periods: &[DailyTableForecast]) {
    let table = Table::new(periods).to_string();
    println!("{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        ForecastPeriod, HourlyForecastPeriod, MeasurementValue, PrecipitationValue,
    };

    #[test]
    fn format_hourly_forecast_formats_common_fields() {
        let period = HourlyForecastPeriod {
            start_time: "2026-04-28T14:30:00+00:00".to_string(),
            end_time: Some("2026-04-28T15:30:00+00:00".to_string()),
            is_daytime: Some(true),
            temperature: 72,
            temperature_unit: "F".to_string(),
            short_forecast: "Sunny".to_string(),
            wind_speed: "10 mph".to_string(),
            wind_direction: "E".to_string(),
            icon: Some("https://api.weather.gov/icons/land/day/few".to_string()),
            dewpoint: Some(MeasurementValue {
                unit_code: Some("wmoUnit:degC".to_string()),
                value: Some(11.3),
            }),
            relative_humidity: Some(MeasurementValue {
                unit_code: Some("wmoUnit:percent".to_string()),
                value: Some(55.0),
            }),
            probability_of_precipitation: PrecipitationValue { value: Some(20) },
        };

        let row = format_hourly_forecast(&period);
        assert_eq!(row.time, "14:30:00");
        assert_eq!(row.temperature, "72°F");
        assert_eq!(row.precip, "20%");
        assert_eq!(row.humidity, "55%");
        assert_eq!(row.dewpoint, "52.3 F");
        assert_eq!(row.wind, "10 mph E");
        assert_eq!(row.icon, "🌤");
        assert_eq!(row.forecast, "Sunny");
    }

    #[test]
    fn format_hourly_forecast_uses_na_when_precip_missing() {
        let period = HourlyForecastPeriod {
            start_time: "2026-04-28T08:00:00-06:00".to_string(),
            end_time: None,
            is_daytime: Some(false),
            temperature: 55,
            temperature_unit: "F".to_string(),
            short_forecast: "Cloudy".to_string(),
            wind_speed: "7 mph".to_string(),
            wind_direction: "NW".to_string(),
            icon: None,
            dewpoint: None,
            relative_humidity: None,
            probability_of_precipitation: PrecipitationValue { value: None },
        };

        let row = format_hourly_forecast(&period);
        assert_eq!(row.time, "08:00:00");
        assert_eq!(row.precip, "N/A");
        assert_eq!(row.humidity, "N/A");
        assert_eq!(row.dewpoint, "N/A");
        assert_eq!(row.icon, "❔");
    }

    #[test]
    fn format_daily_forecast_uses_daily_columns() {
        let period = ForecastPeriod {
            name: "Wednesday".to_string(),
            detailed_forecast: "Mostly sunny and mild".to_string(),
            is_daytime: Some(true),
            icon: None,
            temperature: Some(67),
            temperature_unit: Some("F".to_string()),
            short_forecast: Some("Mostly Sunny".to_string()),
            wind_speed: Some("8 mph".to_string()),
            wind_direction: Some("NW".to_string()),
            probability_of_precipitation: Some(PrecipitationValue { value: Some(15) }),
        };

        let row = format_daily_forecast(&period);
        assert_eq!(row.day, "Wednesday");
        assert_eq!(row.icon, "☀");
        assert_eq!(row.high_low, "67°F");
        assert_eq!(row.precip, "15%");
        assert_eq!(row.wind, "8 mph NW");
        assert_eq!(row.forecast, "Mostly Sunny");
    }

    #[test]
    fn clear_sky_icon_is_sun_in_daytime() {
        let period = HourlyForecastPeriod {
            start_time: "2026-04-28T12:00:00+00:00".to_string(),
            end_time: None,
            is_daytime: Some(true),
            temperature: 70,
            temperature_unit: "F".to_string(),
            short_forecast: "Clear".to_string(),
            wind_speed: "5 mph".to_string(),
            wind_direction: "N".to_string(),
            icon: Some("https://api.weather.gov/icons/land/day/skc".to_string()),
            dewpoint: None,
            relative_humidity: None,
            probability_of_precipitation: PrecipitationValue { value: Some(0) },
        };

        let row = format_hourly_forecast(&period);
        assert_eq!(row.icon, "☀");
    }

    #[test]
    fn clear_sky_icon_is_moon_at_night() {
        let period = HourlyForecastPeriod {
            start_time: "2026-04-28T23:00:00+00:00".to_string(),
            end_time: None,
            is_daytime: Some(false),
            temperature: 55,
            temperature_unit: "F".to_string(),
            short_forecast: "Clear".to_string(),
            wind_speed: "4 mph".to_string(),
            wind_direction: "NE".to_string(),
            icon: Some("https://api.weather.gov/icons/land/night/skc".to_string()),
            dewpoint: None,
            relative_humidity: None,
            probability_of_precipitation: PrecipitationValue { value: Some(0) },
        };

        let row = format_hourly_forecast(&period);
        assert_eq!(row.icon, "🌙");
    }

    #[test]
    fn build_seven_day_forecast_shows_high_low() {
        let periods = vec![
            ForecastPeriod {
                name: "Wednesday".to_string(),
                detailed_forecast: "Warm".to_string(),
                is_daytime: Some(true),
                temperature: Some(67),
                temperature_unit: Some("F".to_string()),
                short_forecast: Some("Sunny".to_string()),
                wind_speed: Some("8 mph".to_string()),
                wind_direction: Some("NW".to_string()),
                icon: Some("https://api.weather.gov/icons/land/day/skc".to_string()),
                probability_of_precipitation: Some(PrecipitationValue { value: Some(10) }),
            },
            ForecastPeriod {
                name: "Wednesday Night".to_string(),
                detailed_forecast: "Cool".to_string(),
                is_daytime: Some(false),
                temperature: Some(42),
                temperature_unit: Some("F".to_string()),
                short_forecast: Some("Clear".to_string()),
                wind_speed: Some("5 mph".to_string()),
                wind_direction: Some("W".to_string()),
                icon: Some("https://api.weather.gov/icons/land/night/skc".to_string()),
                probability_of_precipitation: Some(PrecipitationValue { value: Some(20) }),
            },
        ];

        let rows = build_seven_day_forecast(&periods);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].day, "Wednesday");
        assert_eq!(rows[0].icon, "☀");
        assert_eq!(rows[0].high_low, "67°F/42°F");
        assert_eq!(rows[0].precip, "20%");
    }

    #[test]
    fn daily_icon_prefers_sunny_for_daily_forecast() {
        let period = ForecastPeriod {
            name: "Saturday".to_string(),
            detailed_forecast: "Dry and mild".to_string(),
            is_daytime: Some(true),
            temperature: Some(68),
            temperature_unit: Some("F".to_string()),
            short_forecast: Some("Sunny".to_string()),
            wind_speed: Some("5 mph".to_string()),
            wind_direction: Some("S".to_string()),
            icon: Some("https://api.weather.gov/icons/land/day/few".to_string()),
            probability_of_precipitation: Some(PrecipitationValue { value: Some(5) }),
        };

        let row = format_daily_forecast(&period);
        assert_eq!(row.icon, "☀");
    }

    #[test]
    fn icon_mapping_handles_query_string_suffix() {
        let period = HourlyForecastPeriod {
            start_time: "2026-04-28T12:00:00+00:00".to_string(),
            end_time: None,
            is_daytime: Some(true),
            temperature: 70,
            temperature_unit: "F".to_string(),
            short_forecast: "Clear".to_string(),
            wind_speed: "5 mph".to_string(),
            wind_direction: "N".to_string(),
            icon: Some("https://api.weather.gov/icons/land/day/skc?size=small".to_string()),
            dewpoint: None,
            relative_humidity: None,
            probability_of_precipitation: PrecipitationValue { value: Some(0) },
        };

        let row = format_hourly_forecast(&period);
        assert_eq!(row.icon, "☀");
    }
}
