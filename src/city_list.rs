#![allow(unused_variables)]

use actix_web::{HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
#[derive(Debug, Serialize, Deserialize)]
pub struct CityResult {
    pub hotCities: Vec<HotCities>,
    pub cityList: Vec<CityList>,
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotCities {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityList {
    pub title: String,

    #[serde(default)]
    pub citys: Citys,
}

type Citys = Vec<City>;

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub name: String,
}

#[get("/api/cities")]
pub async fn get_city_list() -> Result<HttpResponse> {
    let f = File::open("static/cities.json").unwrap();
    let v: CityResult = serde_json::from_reader(f).unwrap();
    Ok(HttpResponse::Ok().json(v))
}
