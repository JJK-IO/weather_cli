use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IpApiResponse {
    pub query: String,
    pub lat: f64,
    pub lon: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationCache {
    pub ip: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct PointsResponse {
    pub properties: PointProperties,
}

#[derive(Debug, Deserialize)]
pub struct PointProperties {
    pub forecast: String,
    #[serde(rename = "forecastHourly")]
    pub forecast_hourly: String,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "radarStation")]
    pub radar_station: Option<String>,
    #[serde(rename = "relativeLocation")]
    pub relative_location: Option<RelativeLocation>,
}

#[derive(Debug, Deserialize)]
pub struct RelativeLocation {
    pub properties: RelativeLocationProperties,
}

#[derive(Debug, Deserialize)]
pub struct RelativeLocationProperties {
    pub city: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub properties: ForecastProperties,
}

#[derive(Debug, Deserialize)]
pub struct ForecastProperties {
    pub periods: Vec<ForecastPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastPeriod {
    pub name: String,
    #[serde(rename = "detailedForecast")]
    pub detailed_forecast: String,
    #[serde(rename = "isDaytime")]
    pub is_daytime: Option<bool>,
    pub icon: Option<String>,
    pub temperature: Option<i32>,
    #[serde(rename = "temperatureUnit")]
    pub temperature_unit: Option<String>,
    #[serde(rename = "shortForecast")]
    pub short_forecast: Option<String>,
    #[serde(rename = "windSpeed")]
    pub wind_speed: Option<String>,
    #[serde(rename = "windDirection")]
    pub wind_direction: Option<String>,
    #[serde(rename = "probabilityOfPrecipitation")]
    pub probability_of_precipitation: Option<PrecipitationValue>,
}

#[derive(Debug, Deserialize)]
pub struct HourlyForecastResponse {
    pub properties: HourlyForecastProperties,
}

#[derive(Debug, Deserialize)]
pub struct HourlyForecastProperties {
    pub periods: Vec<HourlyForecastPeriod>,
}

#[derive(Debug, Deserialize)]
pub struct HourlyForecastPeriod {
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "isDaytime")]
    pub is_daytime: Option<bool>,
    pub temperature: i32,
    #[serde(rename = "temperatureUnit")]
    pub temperature_unit: String,
    #[serde(rename = "shortForecast")]
    pub short_forecast: String,
    #[serde(rename = "windSpeed")]
    pub wind_speed: String,
    #[serde(rename = "windDirection")]
    pub wind_direction: String,
    pub icon: Option<String>,
    pub dewpoint: Option<MeasurementValue>,
    #[serde(rename = "relativeHumidity")]
    pub relative_humidity: Option<MeasurementValue>,
    #[serde(rename = "probabilityOfPrecipitation")]
    pub probability_of_precipitation: PrecipitationValue,
}

#[derive(Debug, Deserialize)]
pub struct PrecipitationValue {
    pub value: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct MeasurementValue {
    #[serde(rename = "unitCode")]
    pub unit_code: Option<String>,
    pub value: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_ip_api_response() {
        let json = r#"{"query":"1.1.1.1","lat":40.1,"lon":-70.2,"status":"success"}"#;
        let value: IpApiResponse = serde_json::from_str(json).expect("ip api json should parse");

        assert_eq!(value.query, "1.1.1.1");
        assert_eq!(value.lat, 40.1);
        assert_eq!(value.lon, -70.2);
        assert_eq!(value.status, "success");
    }

    #[test]
    fn deserialize_points_response() {
        let json = r#"{"properties":{"forecast":"https://x/forecast","forecastHourly":"https://x/hourly","timeZone":"America/Denver","radarStation":"KFTG","relativeLocation":{"properties":{"city":"Glendale","state":"CO"}}}}"#;
        let value: PointsResponse = serde_json::from_str(json).expect("points json should parse");

        assert_eq!(value.properties.forecast, "https://x/forecast");
        assert_eq!(value.properties.forecast_hourly, "https://x/hourly");
        assert_eq!(value.properties.time_zone.as_deref(), Some("America/Denver"));
        assert_eq!(value.properties.radar_station.as_deref(), Some("KFTG"));
        assert_eq!(
            value
                .properties
                .relative_location
                .as_ref()
                .map(|loc| loc.properties.city.as_str()),
            Some("Glendale")
        );
    }

    #[test]
    fn deserialize_forecast_response() {
        let json = r#"{"properties":{"periods":[{"name":"Tonight","detailedForecast":"Clear and cool","isDaytime":false,"icon":"https://api.weather.gov/icons/land/night/skc","temperature":39,"temperatureUnit":"F","shortForecast":"Clear","windSpeed":"5 mph","windDirection":"NW","probabilityOfPrecipitation":{"value":5}}]}}"#;
        let value: ForecastResponse = serde_json::from_str(json).expect("forecast json should parse");

        assert_eq!(value.properties.periods.len(), 1);
        assert_eq!(value.properties.periods[0].name, "Tonight");
        assert_eq!(
            value.properties.periods[0].detailed_forecast,
            "Clear and cool"
        );
        assert_eq!(value.properties.periods[0].is_daytime, Some(false));
        assert_eq!(
            value.properties.periods[0].icon.as_deref(),
            Some("https://api.weather.gov/icons/land/night/skc")
        );
        assert_eq!(value.properties.periods[0].temperature, Some(39));
    }

    #[test]
    fn deserialize_hourly_forecast_response() {
        let json = r#"{"properties":{"periods":[{"startTime":"2026-04-28T10:00:00+00:00","endTime":"2026-04-28T11:00:00+00:00","isDaytime":true,"temperature":61,"temperatureUnit":"F","shortForecast":"Mostly Sunny","windSpeed":"8 mph","windDirection":"S","icon":"https://api.weather.gov/icons/land/day/few","dewpoint":{"unitCode":"wmoUnit:degC","value":7.2},"relativeHumidity":{"unitCode":"wmoUnit:percent","value":44.0},"probabilityOfPrecipitation":{"value":5}}]}}"#;
        let value: HourlyForecastResponse =
            serde_json::from_str(json).expect("hourly forecast json should parse");

        assert_eq!(value.properties.periods.len(), 1);
        let period = &value.properties.periods[0];
        assert_eq!(period.start_time, "2026-04-28T10:00:00+00:00");
        assert_eq!(period.temperature, 61);
        assert_eq!(period.temperature_unit, "F");
        assert_eq!(period.short_forecast, "Mostly Sunny");
        assert_eq!(period.wind_speed, "8 mph");
        assert_eq!(period.wind_direction, "S");
        assert_eq!(period.is_daytime, Some(true));
        assert_eq!(period.icon.as_deref(), Some("https://api.weather.gov/icons/land/day/few"));
        assert_eq!(
            period.dewpoint.as_ref().and_then(|v| v.value),
            Some(7.2)
        );
        assert_eq!(
            period.relative_humidity.as_ref().and_then(|v| v.value),
            Some(44.0)
        );
        assert_eq!(period.probability_of_precipitation.value, Some(5));
    }
}
