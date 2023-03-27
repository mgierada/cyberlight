#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use implementations::access_token::auth_error_catcher;
// use implementations::access_token::internal_server_error;
use lazy_static::lazy_static;
use routes::all_devices_routes::{
    get_all_devices_handler, get_status_for_all_devices, get_status_for_device,
};
use routes::healthcheck_routes::healthcheck_handler;
use routes::office_lamp_routes::{office_off_handler, office_on_handler};
use routes::tv_lamp_routes::{tv_off_handler, tv_on_handler};
use std::env::var;

pub mod implementations;
pub mod routes;
pub mod services;
pub mod wrappers;

lazy_static! {
    pub static ref GOVEE_API_KEY: String =
        var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.");
}
lazy_static! {
    pub static ref GOVEE_ROOT_URL: String =
        var("GOVEE_ROOT_URL").expect("GOVEE_ROOT_URL must be set.");
}
lazy_static! {
    pub static ref ACCESS_TOKEN: String = var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set.");
}

#[launch]
fn rocket() -> _ {
    // read .env file
    dotenv().ok();
    rocket::build()
        // .register(catchers![internal_server_error])
        .register("/", catchers![auth_error_catcher])
        .mount("/tv", routes![tv_on_handler, tv_off_handler])
        .mount("/office", routes![office_on_handler, office_off_handler])
        .mount("/", routes![healthcheck_handler])
        .mount(
            "/",
            routes![
                get_all_devices_handler,
                get_status_for_all_devices,
                get_status_for_device,
            ],
        )
}
