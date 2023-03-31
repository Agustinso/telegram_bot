use crate::providers;

enum WeatherType {
    NUBLADO,

}
enum WindType {
    CALMA,

}

struct Wind {
    variant: WindType,
    angle: f32,
    speed: f32
}

struct WeatherData {
    date: String,
    humidity: f32,
    pressure: f32,
    temperature: f32,
    visibility: f32,
    weather: WeatherType,
    wind: Wind
}


pub async fn retrive() -> Result<(), Box<dyn std::error::Error>> {
    providers::SMN::now().await?;
    Ok(())
}
