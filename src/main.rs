use std::env;

use dotenv::dotenv;
use db::{DBCollections, Perks, UserData, COLLECTIONS, DATABASE_NAME, DB};
use log::info;
use mongodb::bson::doc;
use rocket::{http::Status, serde::json::Json};
use rocket_db_pools::{mongodb::Collection, Connection, Database};

#[macro_use] extern crate rocket;
pub mod logging;
pub mod db;

#[post("/login/<id>")]
async fn post_register(id: &str, db: Connection<DB>) -> Result<Json<UserData>, Status> {
    // Get the steam user data, this is to check if the user exists
    // and get the users username and stuffs.
    let collection: Collection<UserData> = db.database(DATABASE_NAME).collection(&COLLECTIONS[DBCollections::UserStatCollection]);

    let potential_user = collection.find_one(
        doc! {
            "steam_id": id
        },
        None
    ).await;

    if let Ok(Some(potential_user)) = potential_user {
        return Ok(Json(potential_user));
    }

    if let Ok(res) = reqwest::get(format!("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}", env::var("STEAM_API_KEY").unwrap(), id)).await {
        let data = res.json::<serde_json::Value>().await;

        if data.is_err() {
            return Err(Status::NoContent);
        }
        let data = data.unwrap();

        if data["response"]["players"].is_array() && data["response"]["players"].as_array().unwrap().is_empty() {
            return Err(Status::NoContent);
        }

        let user_data = UserData {
            steam_id: data["response"]["players"][0]["steamid"].as_str().unwrap().to_string(),
            last_known_username: data["response"]["players"][0]["personaname"].as_str().unwrap().to_string(),
            points: 0,
            perks: Perks::default()
        };

        let result = collection.insert_one(user_data.clone(), None).await;

        if let Ok(_result) = result {
            Ok(Json(user_data))
        } else {
            Err(Status::InternalServerError)
        }
    } else {
        Err(Status::NoContent)
    }
}

#[get("/data/<id>")]
async fn get_data(id: &str, db: Connection<DB>) -> Result<Json<UserData>, Status> {
    let collection: Collection<UserData> = db.database(DATABASE_NAME).collection(&COLLECTIONS[DBCollections::UserStatCollection]);
    let result = collection.find_one(
        doc! {
            "steam_id": id
        },
        None
    ).await;

    if let Ok(result) = result {
        if result.is_none() {
            return Err(Status::NoContent);
        }

        Ok(Json(result.unwrap()))
    } else {
        Err(Status::InternalServerError)
    }
}

#[post("/data/<id>/update_points/<points>")]
async fn update_points(id: &str, points: i32, db: Connection<DB>) -> Status {
    let collection: Collection<UserData> = db.database(DATABASE_NAME).collection(&COLLECTIONS[DBCollections::UserStatCollection]);

    let user = collection.find_one(
        doc! {
            "steam_id": id
        },
        None
    ).await;

    if let Ok(Some(mut user)) = user {
        user.points += points as i64;

        if user.points < 0 {
            user.points = 0;
        }

        let result = collection.update_one(
            doc! {
                "steam_id": id
            },
            doc! {
                "$set": {
                    "points": user.points
                }
            },
            None
        ).await;

        if let Ok(_result) = result {
            Status::Ok
        } else {
            Status::InternalServerError
        }
    } else {
        Status::NoContent
    }
}

#[get("/")]
async fn index() -> &'static str {
    "Nuclear Nightmare Server?"
}

#[launch]
async fn rocket() -> _ {
    println!("Checking .env...");
    dotenv().ok();

    println!("Initializing logger...");
    logging::init_logger();
    logging::clean_old();

    let figment = rocket::Config::figment()
        .merge(("databases.nuclear_db", rocket_db_pools::Config {
            url: env::var("MONGO_DB_URL").unwrap(),
            min_connections: None,
            max_connections: 1024,
            connect_timeout: 3,
            idle_timeout: None,
            extensions: None,
        }));

        info!("Starting Rocket server...");
    rocket::custom(figment).attach(DB::init()).mount("/", routes![index, get_data, update_points, post_register])
    //rocket::build().attach(DB::init()).mount("/", routes![index, get_points, post_register])
}
