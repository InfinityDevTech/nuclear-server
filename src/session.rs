use std::{
    collections::HashMap,
    sync::Mutex,
};

use lazy_static::lazy_static;
use reqwest::StatusCode;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, PartialOrd)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, EnumIter)]
pub enum Region {
    NorthAmerica,
    SouthAmerica,
    Asia,
    Europe,
    Africa,
    Australia,
    Antarctica,
}

pub fn region_from_string(region: String) -> Region {
    let string = region.to_lowercase();

    match string.as_str() {
        "northamerica" => Region::NorthAmerica,
        "southamerica" => Region::SouthAmerica,
        "asia" => Region::Asia,
        "europe" => Region::Europe,
        "africa" => Region::Africa,
        "australia" => Region::Australia,
        "antarctica" => Region::Antarctica,
        _ => Region::NorthAmerica,
    }
}

pub fn string_from_region(region: Region) -> String {
    match region {
        Region::NorthAmerica => "NorthAmerica".to_string(),
        Region::SouthAmerica => "SouthAmerica".to_string(),
        Region::Asia => "Asia".to_string(),
        Region::Europe => "Europe".to_string(),
        Region::Africa => "Africa".to_string(),
        Region::Australia => "Australia".to_string(),
        Region::Antarctica => "Antarctica".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub session_id: String,
    pub region: Region,
    pub host_steam_id: String,
    pub players: Vec<String>,
    pub difficulty: GameDifficulty,
    pub started: bool,
    pub extraction_enabled: bool,
    pub friendly_fire: bool,
}

lazy_static! {
    pub static ref SESSIONS: Mutex<HashMap<String, Session>> = Mutex::new(HashMap::new());
    pub static ref SESSIONS_BY_REGION: Mutex<HashMap<Region, Vec<String>>> =
        Mutex::new(HashMap::new());
}

pub fn setup_hashmap() {
    let mut regions = SESSIONS_BY_REGION.lock().unwrap();

    for region in Region::iter() {
        regions.insert(region.clone(), vec![]);
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegionListRes {
    pub regions: Vec<String>,
}

#[get("/sessions/regions")]
pub async fn list_regions() -> Json<RegionListRes> {
    let regions = SESSIONS_BY_REGION.lock().unwrap();
    let mut regions_list = Vec::new();

    for region in regions.keys() {
        regions_list.push(string_from_region(region.clone()));
    }

    Json(RegionListRes {
        regions: regions_list,
    })
}

#[derive(Serialize, Deserialize)]
pub struct SessionListRes {
    pub region: String,
    pub sessions: Vec<Session>,
}

#[get("/sessions/list/<region>")]
pub async fn get_sessions_by_region(region: &str) -> Json<SessionListRes> {
    let sessions = SESSIONS.lock().unwrap();
    let sessions_by_region = SESSIONS_BY_REGION.lock().unwrap();
    let region = region_from_string(region.to_string());

    let session_ids = sessions_by_region.get(&region).unwrap();
    let mut sessions_by_region = Vec::new();

    for session_id in session_ids {
        let session = sessions.get(session_id).unwrap();
        sessions_by_region.push(session.clone());
    }

    let res = SessionListRes {
        region: string_from_region(region),
        sessions: sessions_by_region,
    };

    Json(res)
}

#[derive(Serialize, Deserialize)]
pub struct SessionCreateArgs {

}

#[post("/sessions/create", data = "<body>")]
pub async fn create_session(body: Json<SessionCreateArgs>) -> Status {
    Status::Ok
}