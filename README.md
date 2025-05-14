# weather_cli

A simple command-line tool written in Rust to fetch and display weather forecasts from [api.weather.gov](https://www.weather.gov/documentation/services-web-api).

## 🚀 Features

- Fetches weather forecast using latitude and longitude
- Displays human-readable daily forecasts
- CLI powered by [`clap`](https://docs.rs/clap/)
- Fast and efficient thanks to Rust and `reqwest`

## 🛠 Requirements

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70+ recommended)
- Internet connection (for API calls)

## 📦 Installation

Clone the repo and build it:

```bash
git clone https://github.com/your-username/weather_cli.git
cd weather_cli
cargo build --release
```

This will produce the optimized binary at:

```bash
target/release/weather_cli
```

## 💻 Usage

Run the CLI with default or custom coordinates:

```bash
cargo run -- --lat 39.7392 --lon -104.9903
```

Or run the compiled binary directly:

```bash
./target/release/weather_cli --lat 39.7392 --lon -104.9903
```

### ✅ Options

| Option      | Short | Description              |
|-------------|-------|--------------------------|
| `--lat`     | `-a`  | Latitude of location     |
| `--lon`     | `-o`  | Longitude of location    |
| `--help`    |       | Show help message        |

## 🧪 Example Output

```text
Weather Summary:
This Afternoon: A slight chance of rain showers before 3pm, then a slight chance of showers and thunderstorms. Partly sunny. High near 76, with temperatures falling to around 67 in the afternoon. West northwest wind 0 to 15 mph, with gusts as high as 33 mph. Chance of precipitation is 20%.
Tonight: A slight chance of rain showers before 10pm. Partly cloudy, with a low around 44. West wind 6 to 22 mph, with gusts as high as 35 mph. Chance of precipitation is 20%.

12-Hour Hourly Forecast:
+----------+-------------+--------+------------+-----------------------------------------+
| time     | temperature | precip | wind       | forecast                                |
+----------+-------------+--------+------------+-----------------------------------------+
| 14:00:00 | 72°F        | 18%    | 5 mph SE   | Slight Chance Rain Showers              |
+----------+-------------+--------+------------+-----------------------------------------+
| 15:00:00 | 71°F        | 19%    | 0 mph      | Slight Chance Showers And Thunderstorms |
+----------+-------------+--------+------------+-----------------------------------------+
| 16:00:00 | 70°F        | 19%    | 10 mph WNW | Slight Chance Showers And Thunderstorms |
+----------+-------------+--------+------------+-----------------------------------------+
| 17:00:00 | 67°F        | 20%    | 15 mph NW  | Slight Chance Showers And Thunderstorms |
+----------+-------------+--------+------------+-----------------------------------------+
| 18:00:00 | 60°F        | 20%    | 22 mph NW  | Slight Chance Rain Showers              |
+----------+-------------+--------+------------+-----------------------------------------+
| 19:00:00 | 57°F        | 19%    | 17 mph NW  | Slight Chance Rain Showers              |
+----------+-------------+--------+------------+-----------------------------------------+
| 20:00:00 | 55°F        | 18%    | 15 mph NW  | Slight Chance Rain Showers              |
+----------+-------------+--------+------------+-----------------------------------------+
| 21:00:00 | 52°F        | 17%    | 10 mph NNW | Slight Chance Rain Showers              |
+----------+-------------+--------+------------+-----------------------------------------+
| 22:00:00 | 52°F        | 12%    | 6 mph WNW  | Mostly Cloudy                           |
+----------+-------------+--------+------------+-----------------------------------------+
| 23:00:00 | 52°F        | 7%     | 6 mph W    | Mostly Cloudy                           |
+----------+-------------+--------+------------+-----------------------------------------+
| 00:00:00 | 51°F        | 2%     | 8 mph SW   | Partly Cloudy                           |
+----------+-------------+--------+------------+-----------------------------------------+
| 01:00:00 | 50°F        | 2%     | 9 mph SW   | Partly Cloudy                           |
+----------+-------------+--------+------------+-----------------------------------------+
```

## 📚 API Reference

Data is sourced from:

- [https://api.weather.gov](https://api.weather.gov)
- U.S. National Weather Service

## 📝 License

MIT License © Jason Kuo
