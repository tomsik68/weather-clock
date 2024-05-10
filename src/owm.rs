use serde::Deserialize;
use serde::Serialize;

pub async fn fetch_current_weather(url: &str) -> anyhow::Result<Root> {
    Ok(reqwest::get(url).await?.json().await?)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    #[serde(rename = "timezone_offset")]
    pub timezone_offset: i64,
    pub current: Current,
    pub minutely: Vec<Minutely>,
    pub hourly: Vec<Hourly>,
    pub daily: Vec<Daily>,
    #[serde(default)]
    pub alerts: Vec<Alert>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "dew_point")]
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: i64,
    pub visibility: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_deg")]
    pub wind_deg: i64,
    pub weather: Vec<Weather>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Minutely {
    pub dt: i64,
    pub precipitation: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hourly {
    pub dt: i64,
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "dew_point")]
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: i64,
    pub visibility: i64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_deg")]
    pub wind_deg: i64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    pub weather: Vec<Weather>,
    pub pop: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daily {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub moonrise: i64,
    pub moonset: i64,
    #[serde(rename = "moon_phase")]
    pub moon_phase: f64,
    pub summary: String,
    pub temp: Temp,
    #[serde(rename = "feels_like")]
    pub feels_like: FeelsLike,
    pub pressure: i64,
    pub humidity: i64,
    #[serde(rename = "dew_point")]
    pub dew_point: f64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
    #[serde(rename = "wind_deg")]
    pub wind_deg: i64,
    #[serde(rename = "wind_gust")]
    pub wind_gust: f64,
    pub weather: Vec<Weather>,
    pub clouds: i64,
    pub pop: f64,
    pub rain: Option<f64>,
    pub uvi: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temp {
    pub day: f64,
    pub min: f64,
    pub max: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeelsLike {
    pub day: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    #[serde(rename = "sender_name")]
    pub sender_name: String,
    pub event: String,
    pub start: i64,
    pub end: i64,
    pub description: String,
    pub tags: Vec<String>,
}
