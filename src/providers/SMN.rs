use serde::{self, Deserialize};
use reqwest::{header::{USER_AGENT, HeaderMap, HOST, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, ORIGIN, CONNECTION, REFERER, HeaderName, AUTHORIZATION}, Client};

#[derive(Deserialize)]
struct SMNWeather{
    description: String,
    id: usize
}

#[derive(Deserialize)]
struct SMNWind{
    direction: String,
    deg: f32,
    speed: f32,
}

#[derive(Deserialize)]
struct SMNCoords {
    long: f32,
    lat: f32,
    distance: f32
}

#[derive(Deserialize)]
struct SMNLocation {
    id: usize,
    name:String,
    department:String,
    province:String,
    #[serde(rename = "type")]
    smn_type: String,
    coord: SMNCoords
}

#[derive(Deserialize)]
struct SMNParser {
    date: String,
    pressure: f32,
    feels_line: Option<f32>,
    temperature: f32,
    visibility: f32,
    weather: SMNWeather,
    wind: SMNWind,
    station_id: usize,
}


pub async fn now() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(HOST, "ws1.smn.gob.ar".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/111.0".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "es-AR".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "GZIP, DEFLATE, br".parse().unwrap());
    headers.insert(ORIGIN, "https://www.smn.gob.ar".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(REFERER, "https://www.smn.gob.ar/".parse().unwrap());
    headers.insert(HeaderName::from_static("sec-fetch-dest"), "empty".parse().unwrap());
    headers.insert(HeaderName::from_static("sec-fetch-mode"), "cors".parse().unwrap());
    headers.insert(HeaderName::from_static("sec-fetch-site"), "same-site".parse().unwrap());

    let token_response = Client::new().get("https://www.smn.gob.ar/")
        .header(USER_AGENT, headers[USER_AGENT].clone())
        .send()
        .await?
        .text()
        .await?;

    let mut token: String = "JWT ".to_owned();

    if let Some(i) = token_response.find("token', '").map(|i| i+9) {
        if let Some(j) = token_response[i..].find("'").map(|j| i + j) {
            token.push_str(&token_response[i..j]);
        }
        else {
            eprintln!("ERROR: Parsing end of token");
        }
    }
    else {
        eprintln!("ERROR: Parsing start of token");
    }

    headers.append(AUTHORIZATION, token.parse().unwrap());


    let client = reqwest::Client::new();

    let now_resp = client
        .get("https://ws1.smn.gob.ar/v1/weather/location/7706")
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/111.0")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let now: SMNParser = serde_json::from_str(now_resp.as_str()).unwrap();
    println!("{:?}", now.temperature);
    Ok(())
}