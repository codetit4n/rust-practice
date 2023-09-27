#![allow(dead_code)]
use clap::Parser;
use dotenv;
use reqwest;
use serde::Deserialize;
use std::{env, ops::RangeInclusive};

const VALID_LAT_RANGE: RangeInclusive<f32> = -90.0..=90.0;
const VALID_LON_RANGE: RangeInclusive<f32> = -180.0..=180.0;

#[derive(Parser, Debug)]
#[command(name = "weather")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    #[arg(long, allow_hyphen_values = true, value_parser = parse_lat)]
    lat: f32,
    #[arg(long, allow_hyphen_values = true, value_parser = parse_lon)]
    lon: f32,
}

fn parse_lat(s: &str) -> Result<f32, String> {
    let lat: f32 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid latitude"))?;

    if VALID_LAT_RANGE.contains(&lat) {
        Ok(lat as f32)
    } else {
        Err(format!(
            "latitude should be between {}° and {}°",
            VALID_LAT_RANGE.start(),
            VALID_LAT_RANGE.end()
        ))
    }
}

fn parse_lon(s: &str) -> Result<f32, String> {
    let lon: f32 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid longitude"))?;

    if VALID_LON_RANGE.contains(&lon) {
        Ok(lon as f32)
    } else {
        Err(format!(
            "longitude should be between {}° and {}°",
            VALID_LON_RANGE.start(),
            VALID_LON_RANGE.end()
        ))
    }
}
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct CurrentWeatherMain {
    temp: f32,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    weather: Vec<Weather>,
    main: CurrentWeatherMain,
}

fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().expect("Failed to load .env file");

    let mut api_key: Option<String> = None;

    for (key, value) in env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value)
    }

    if api_key.is_none() {
        panic!("Need an API key!!!");
    }

    let api_key = api_key.unwrap();

    let args: Args = Args::parse();

    let lat_val = args.lat;
    let lon_val = args.lon;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat_val}&lon={lon_val}&appid={api_key}&units=metric"
    );

    let weather: CurrentWeather = reqwest::blocking::get(url)?.json()?;

    println!(
        "Current weather: {}, {}°C",
        weather.weather[0].description, weather.main.temp
    );

    Ok(())
}
