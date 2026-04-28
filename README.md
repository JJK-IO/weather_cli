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

| Option      | Short | Description                  |
| ----------- | ----- | ---------------------------- |
| `--lat`     | `-a`  | Latitude of location         |
| `--lon`     | `-o`  | Longitude of location        |
| `--verbose` | `-v`  | Show IP/location diagnostics |
| `--help`    |       | Show help message            |

## 🧪 Example Output

```text
Location: Sheridan, CO | Timezone: America/Denver | Radar: KFTG
Weather Summary:
Today: A slight chance of rain showers before noon, then a chance of showers and thunderstorms. Partly sunny, with a high near 58. North northeast wind 3 to 9 mph. Chance of precipitation is 30%. New rainfall amounts less than a tenth of an inch possible.
Tonight: A slight chance of rain showers before 9pm. Partly cloudy, with a low around 38. West southwest wind 2 to 8 mph. Chance of precipitation is 20%.

Next 7 Days:
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| day       | icon | high_low  | precip | wind           | forecast                                                  |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| Today     | ⛈    | 58°F/38°F | 28%    | 3 to 9 mph NNE | Chance Showers And Thunderstorms                          |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| Wednesday | ⛈    | 61°F/39°F | 52%    | 2 to 8 mph NNE | Chance Rain Showers then Chance Showers And Thunderstorms |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| Thursday  | ⛈    | 48°F/36°F | 80%    | 3 to 9 mph NNE | Showers And Thunderstorms                                 |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| Friday    | ⛈    | 56°F/34°F | 41%    | 2 to 6 mph ENE | Chance Rain Showers                                       |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| Saturday  | ☀    | 68°F/41°F | 7%     | 2 to 6 mph SE  | Sunny                                                     |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| Sunday    | ☀    | 75°F/44°F | 20%    | 5 to 8 mph SW  | Mostly Sunny then Slight Chance Showers And Thunderstorms |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+
| Monday    | ☀    | 72°F/43°F | 45%    | 3 to 9 mph NNW | Mostly Sunny then Chance Showers And Thunderstorms        |
+-----------+------+-----------+--------+----------------+-----------------------------------------------------------+

12-Hour Hourly Forecast:
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| time     | icon | temperature | precip | humidity | dewpoint | wind      | forecast                         |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 10:00:00 | 🌧    | 46°F        | 16%    | 65%      | 35.0 F   | 5 mph NE  | Slight Chance Rain Showers       |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 11:00:00 | 🌧    | 50°F        | 16%    | 56%      | 35.0 F   | 6 mph NNE | Slight Chance Rain Showers       |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 12:00:00 | ⛈    | 52°F        | 28%    | 50%      | 34.0 F   | 7 mph N   | Chance Showers And Thunderstorms |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 13:00:00 | ⛈    | 54°F        | 28%    | 47%      | 34.0 F   | 8 mph N   | Chance Showers And Thunderstorms |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 14:00:00 | ⛈    | 56°F        | 28%    | 42%      | 33.0 F   | 9 mph N   | Chance Showers And Thunderstorms |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 15:00:00 | ⛈    | 56°F        | 25%    | 42%      | 33.0 F   | 9 mph N   | Chance Showers And Thunderstorms |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 16:00:00 | ⛈    | 57°F        | 25%    | 38%      | 32.0 F   | 9 mph N   | Chance Showers And Thunderstorms |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 17:00:00 | ⛈    | 57°F        | 25%    | 36%      | 30.0 F   | 9 mph NNW | Chance Showers And Thunderstorms |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 18:00:00 | 🌧    | 56°F        | 22%    | 34%      | 28.0 F   | 8 mph NW  | Slight Chance Rain Showers       |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 19:00:00 | 🌧    | 56°F        | 22%    | 35%      | 29.0 F   | 7 mph NNW | Slight Chance Rain Showers       |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 20:00:00 | 🌧    | 53°F        | 22%    | 39%      | 29.0 F   | 6 mph N   | Slight Chance Rain Showers       |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
| 21:00:00 | ☁    | 51°F        | 12%    | 42%      | 29.0 F   | 5 mph N   | Mostly Cloudy                    |
+----------+------+-------------+--------+----------+----------+-----------+----------------------------------+
```

## 📚 API Reference

Data is sourced from:

- [https://api.weather.gov](https://api.weather.gov)
- U.S. National Weather Service
- [http://ip-api.com/json/](http://ip-api.com/json/) (IP-based location lookup when `--lat/--lon` are not provided)

## 📝 License

MIT License © Jason Kuo
