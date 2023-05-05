use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub city: City,
    pub cod: String,
    pub message: f64,
    pub cnt: i64,
    pub list: Vec<List>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    pub id: i64,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub population: i64,
    pub timezone: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: Temp,
    #[serde(rename = "feels_like")]
    pub feels_like: FeelsLike,
    pub pressure: i64,
    pub humidity: i64,
    pub weather: Vec<Weather>,
    pub speed: f64,
    pub deg: i64,
    pub gust: f64,
    pub clouds: i64,
    pub pop: f64,
    pub rain: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temp {
    pub day: f64,
    pub min: f64,
    pub max: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeelsLike {
    pub day: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}
