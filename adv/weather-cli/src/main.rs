use std::env;

use clap::Parser;

use dotenv;
use reqwest;

// using hardcoded values for now
const LAT: f32 = 30.7333;
const LON: f32 = 76.7794;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    // Number of days for the forcast
    #[arg(short, default_value_t = 0)]
    days: u8,
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

    //    let args: Args = Args::parse();

    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={LAT}&lon={LON}&appid={api_key}&units=metric"
    );

    let body: String = reqwest::blocking::get(url)?.text()?;

    println!("{:?}", body);

    Ok(())
}
