use enum_map::{Enum, EnumMap, enum_map};
use lazy_static::lazy_static;
use rocket_db_pools::Database;
use serde::{Deserialize, Serialize};

// The default is false, THANK GOD.
#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct Perks {
    pub hardcore_survivor: bool,
    pub doggy_treats: bool,
    pub immune_system: bool,
    pub strong_scent: bool,
    pub infection_detector: bool,
    pub gas_guzzler: bool,
    pub drivers_license: bool,
    pub home_free: bool,
    pub gen_jockey: bool,
    pub oil_conservation: bool,
    pub soft_feet: bool,
    pub monster_hunter: bool,
    pub flare_signal: bool,
    pub acrobatic_instructor: bool,
    pub pain_medication: bool,
    pub military_veteran: bool,
    pub hungry_humans: bool,
    pub vaccinated_scientist: bool,
    pub emergency_fund: bool,
    pub warn_blooded: bool,
    pub speed_demon: bool,
    pub pilot_license: bool,
    pub rechargeable_battery: bool,
    pub nuclear_awareness: bool,
    pub elusive_fighter: bool,
    pub dog_whisper: bool,
    pub hide_n_seek: bool,
    pub fire_resistance: bool,
    pub classified_collector: bool,
    pub self_defense: bool,
    pub cold_blooded: bool,
    pub heated_jacket: bool,
    pub health_regeneration: bool,
    pub camp_counseler: bool,
    pub nightmare_detector: bool,
    pub gps_navigator: bool,
    pub rocket_booster: bool,
    pub winter_mystery: bool,
    pub jetpack_ride: bool,
    pub night_vision: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub steam_id: String,
    pub last_known_username: String,
    pub points: i64,
    pub perks: Perks
}

pub static DATABASE_NAME: &str = "nuclear_db";

#[derive(Debug, Clone, Enum)]
pub enum DBCollections {
    UserStatCollection
}

lazy_static! {
    pub static ref COLLECTIONS: EnumMap<DBCollections, String> = {
        enum_map! {
            DBCollections::UserStatCollection => "user_stats".to_string()
        }
    };
}

#[derive(Database)]
#[database("nuclear_db")]
pub struct DB(rocket_db_pools::mongodb::Client);