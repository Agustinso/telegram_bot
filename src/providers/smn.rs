use serde::{self, Deserialize};
use reqwest::{header::{USER_AGENT, HeaderMap, HOST, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, ORIGIN, CONNECTION, REFERER, HeaderName, AUTHORIZATION}, Client};

use crate::providers::common::{WeatherData, Wind};
use crate::providers::common::WeatherType;

#[derive(Deserialize, Debug)]
struct SMNWeather{
    pub(crate) description: String,
    pub(crate) id: usize
}

#[derive(Deserialize, Debug)]
struct SMNWind{
    pub(crate) direction: String,
    pub(crate) deg: f32,
    pub(crate) speed: f32,
}

#[derive(Deserialize, Debug)]
struct SMNParser {
    pub(crate) date: String,
    pub(crate) humidity: f32,
    pub(crate) pressure: f32,
    pub(crate) temperature: f32,
    pub(crate) weather: SMNWeather,
    pub(crate) wind: SMNWind,
}


pub async fn now() -> Result<WeatherData, Box<dyn std::error::Error>> {
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
        .header(USER_AGENT, "tupapichulo")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let now: SMNParser = serde_json::from_str(now_resp.as_str()).unwrap();
    
    let weather_type = match now.weather.id {
        3|5   => WeatherType::DESPEJADO,
        13|14 => WeatherType::APENASNUBLADO,
        19|20 => WeatherType::LIJERAMENTENUBLADO,
        25|26 => WeatherType::PARCIALMENTENUBLADO,
        37|38 => WeatherType::MAYORMENTENUBLADO,
        43    => WeatherType::COMPLETAMENTENUBLADO,
        72    => WeatherType::LLUVIATORMENTA,
        73    => WeatherType::LLUVIOSO,
        74|75 => WeatherType::PARCIALMENTELLUVIOSO,
        76|81 => WeatherType::LLUVIATORMENTAELECTRICA,
        77    => WeatherType::LLUVIANIEVE,
        79    => WeatherType::NIEVE,
        83    => WeatherType::LLUVIACOPIOSA,
        85    => WeatherType::NEVADACOPIOSA,
        89    => WeatherType::FUERTESLLUVIASTORMENTAELECTRICA,

        _     => WeatherType::DESPEJADO,
    }; 

    
    let data = WeatherData {
        date: now.date, 
        humidity: now.humidity, 
        pressure: now.pressure, 
        temperature: now.temperature,
        wind: Wind{ desc: now.wind.direction, angle: now.wind.deg, speed: now.wind.speed}, 
        weather: weather_type
    };

    Ok(data)
}