use std::env;
use std::error::Error;

use eventsource::reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

const URL: &str = "https://api-v3.mbta.com/predictions/?filter[route]=CR-Worcester&filter[stop]=place-sstat&stop_sequence=1";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let api_key = match env::args().nth(1) {
        Some(api_key) => api_key,
        None => panic!("usage: sse-play API-KEY"),
    };

    let mut headers = HeaderMap::new();
    headers.insert("X-API-Key", HeaderValue::from_str(&api_key)?);
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let client = Client::new_with_client(URL.parse()?, client);

    for event in client {
        println!("{:?}", event);
    }

    Ok(())
}
