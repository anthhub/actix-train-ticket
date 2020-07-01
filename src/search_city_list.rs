#![allow(unused_variables)]
use super::city_list::{CityList, CityResult};
use actix_web::{web, HttpResponse, Result};
use pinyin::ToPinyin;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCityResult {
    pub result: Vec<SearchResult>,
    pub searchKey: String,
}

impl SearchCityResult {
    pub fn new(searchKey: String, result: Vec<SearchResult>) -> SearchCityResult {
        SearchCityResult { result, searchKey }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub key: String,
    pub display: String,
}

impl SearchResult {
    pub fn new(key: String, display: String) -> SearchResult {
        SearchResult { key, display }
    }
}

impl Clone for SearchResult {
    fn clone(&self) -> SearchResult {
        let SearchResult { key, display } = self;
        SearchResult {
            key: key.clone(),
            display: display.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct Info {
    pub key: String,
}

#[get("/api/search")]
pub async fn get_search_city_list(info: web::Query<Info>) -> Result<HttpResponse> {
    // let f = File::open("static/cities.json").unwrap();
    // let v: CityResult = serde_json::from_reader(f).unwrap();

    // 先搜索词转拼音
    let key: &str = &info.key.trim();
    let search_key = key.to_string();
    println!("search_key->{}", search_key);

    let key_chars_vec = key.chars().map(|char| {
        // 一个字符
        let str: &str = &(char.to_string());
        let chars_vec: Vec<&str> = str
            .to_pinyin()
            .map(|item| {
                if let Some(pinyin) = item {
                    pinyin.plain()
                } else {
                    str
                }
            })
            .collect();

        let mut s = "".to_string();
        for pat in chars_vec {
            s.push_str(pat);
        }

        // 先搜索词转拼音

        return s;
    });

    let mut pinyin = "".to_string();
    for pat in key_chars_vec {
        pinyin.push_str(&pat);
    }
    println!("pinyin->{}", pinyin);

    // 取出首字母
    let first_char = pinyin.get(0..1);
    let first_char = match first_char {
        Some(expr) => expr.to_uppercase(),
        None => "".to_string(),
    };
    let key: &str = &pinyin;
    // 再取出cities查找
    let f = File::open("static/cities.json").unwrap();
    let v: CityResult = serde_json::from_reader(f).unwrap();
    let citis_matched_first_char = v.cityList.iter().find(|item| item.title == first_char);

    let result_citis_matched: Vec<SearchResult> = match citis_matched_first_char {
        Some(citis_matched_first_char) => citis_matched_first_char
            .citys
            .iter()
            .filter(|item| {
                let name_pinyin: &str = &item.name;
                let name_py = name_pinyin.to_pinyin();

                let mut name = "".to_string();
                for pat in name_py {
                    if let Some(pinyin) = pat {
                        name.push_str(pinyin.plain());
                    } else {
                        name.push_str(" ");
                    }
                }
                key.contains(&name) || name.contains(&key)
            })
            .map(|item| SearchResult::new(item.name.clone(), item.name.clone()))
            .collect(),
        None => vec![],
    };

    let result_citis_hot: Vec<SearchResult> = v
        .hotCities
        .iter()
        .map(|item| SearchResult::new(item.name.clone(), item.name.clone()))
        .collect();

    let result = [result_citis_matched, result_citis_hot].concat();

    let search_city_result = SearchCityResult::new(search_key, result);

    Ok(HttpResponse::Ok().json(search_city_result))
}
