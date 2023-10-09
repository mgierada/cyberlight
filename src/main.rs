#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use error_handlers::error_handlers::{not_found, server_error, ununauthorized};
use lazy_static::lazy_static;
use routes::all_devices_routes::{
    get_all_devices_handler, get_status_for_all_devices, get_status_for_device,
};
use routes::healthcheck_routes::healthcheck_handler;
use routes::home_routes::home;
use routes::office_routes::{
    office_board_off_on_handler, office_board_on_handler, office_off_handler, office_on_handler,
    office_table_off_handler, office_table_on_handler, office_window_off_handler,
    office_window_on_handler,
};
use routes::standing_routes::{
    standing_left_off_handler, standing_left_on_handler, standing_right_off_handler,
    standing_right_on_handler,
};
use std::env::var;

pub mod constants;
pub mod error_handlers;
pub mod implementations;
pub mod routes;
pub mod services;
pub mod tests;
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
        .register("/", catchers![ununauthorized, not_found, server_error])
        .mount(
            "/standing",
            routes![
                standing_left_on_handler,
                standing_left_off_handler,
                standing_right_on_handler,
                standing_right_off_handler
            ],
        )
        .mount(
            "/office",
            routes![
                office_on_handler,
                office_off_handler,
                office_table_on_handler,
                office_table_off_handler,
                office_window_on_handler,
                office_window_off_handler,
                office_board_on_handler,
                office_board_off_on_handler
            ],
        )
        .mount(
            "/",
            routes![
                healthcheck_handler,
                get_all_devices_handler,
                get_status_for_all_devices,
                get_status_for_device,
                home
            ],
        )
}
