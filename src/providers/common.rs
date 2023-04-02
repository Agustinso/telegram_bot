#[derive(serde::Deserialize, Debug)]
pub enum WeatherType {
    DESPEJADO,
    APENASNUBLADO,
    LIJERAMENTENUBLADO,
    PARCIALMENTENUBLADO,
    MAYORMENTENUBLADO,
    COMPLETAMENTENUBLADO,
    LLUVIATORMENTA,
    LLUVIOSO,
    PARCIALMENTELLUVIOSO,
    LLUVIATORMENTAELECTRICA,
    LLUVIANIEVE,
    NIEVE,
    LLUVIACOPIOSA,
    NEVADACOPIOSA,
    FUERTESLLUVIASTORMENTAELECTRICA,
}

#[derive(serde::Deserialize, Debug)]
pub struct Wind {
    pub(crate) desc: String,
    pub(crate) angle: f32,
    pub(crate) speed: f32
}

#[derive(serde::Deserialize, Debug)]
pub struct WeatherData {
    pub(crate) date: String,
    pub(crate) humidity: f32,
    pub(crate) pressure: f32,
    pub(crate) temperature: f32,
    pub(crate) weather: WeatherType,
    pub(crate) wind: Wind
}
