#[macro_use]
extern crate rocket;
use dotenv::dotenv;

use lazy_static::lazy_static;
use routes::healthcheck::healthcheck_handler;
use routes::office_lamp_routes::{office_off_handler, office_on_handler};
use routes::tv_lamp_routes::{tv_off_handler, tv_on_handler};
use std::env::var;

pub mod routes;
pub mod services;

// const GOVEE_API_KEY: &str = match std::env::var("API_KEY") {
//     Ok(val) => val.as_str(),
//     Err(_) => panic!("API_KEY environment variable not set"),
// };
//
//
// set GOVEE_API_KEY as a const read from env variables
// const GOVEE_API_KEY: &String = &std::env::var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.");
// static GOVEE_API_KEY: &str = &std::env::var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.");
// static GOVEE_API_KEY: String = std::env::var("GOVEE_API_KEY")
//     .expect("GOVEE_API_KEY must be set.")
//     .unwrap();
//

lazy_static! {
    pub static ref GOVEE_API_KEY: String =
        var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.");
}

fn get_govee_api_key() -> String {
    dotenv().ok();
    std::env::var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.")
}

fn get_goove_root_url() -> String {
    dotenv().ok();
    std::env::var("GOVEE_ROOT_URL").expect("GOVEE_ROOT_URL must be set.")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/tv", routes![tv_on_handler, tv_off_handler])
        .mount("/office", routes![office_on_handler, office_off_handler])
        .mount("/", routes![healthcheck_handler])
}
