#[macro_use]
extern crate rocket;

pub mod controllers;
pub mod library;
pub mod models;
pub mod output;
pub mod routes;
pub mod schema;
pub mod services;
pub mod utils;

use crate::library::base_lib_key::REDIS_URL;
use crate::{routes::get_routes, utils::rate_limiter::global_rate_limiter::GlobalRateLimiter};
use dotenvy::dotenv;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;

// Note: The #[launch] macro is removed from here.
// It will be placed in the new main.rs.
// The function is made public so it can be called from main.rs and our tests.
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port: u16 = port.parse().expect("PORT harus berupa angka");
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let redis_client =
        redis::Client::open(REDIS_URL.to_string()).expect("failed to create Redis client");

    let environment = env::var("ENV_ENVIRONMENT").unwrap_or_else(|_| "PRODUCTION".to_string());

    let allowed_origins = if environment == "DEVELOPMENT" {
        AllowedOrigins::all()
    } else {
        AllowedOrigins::some_regex(&[r"^https://.*\.yumana\.my\.id$"])
    };

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error konfigurasi CORS");

    rocket::custom(
        rocket::Config::figment()
            .merge(("port", port))
            .merge(("address", address)),
    )
    .mount("/", get_routes())
    .manage(redis_client)
    .attach(utils::db::attach_db())
    .attach(cors)
    .attach(GlobalRateLimiter)
}
