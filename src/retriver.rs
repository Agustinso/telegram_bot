use reqwest::{header::{USER_AGENT, HeaderMap, HOST, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, ORIGIN, CONNECTION, REFERER, HeaderName, AUTHORIZATION}, Client};


enum Weather {
    NUBLADO,
}
enum Wind {
    CALMA
}

struct WeatherData {
    date: String,
    humidity: f32,
    temperature: f32,
    visibility: f32,
    weather: Weather,
    wind_id: u32,

}


pub async fn retrive() -> Result<(), Box<dyn std::error::Error>> {
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

    //dbg!(&token);
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

    println!("{:#?}", now_resp);
    Ok(())
}
