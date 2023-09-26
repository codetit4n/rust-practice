use clap::Parser;
use dotenv;
use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Parser, Debug)]
#[command(name = "weather")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    #[clap(long)]
    lat: f32,
    #[clap(long)]
    lon: f32,
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
