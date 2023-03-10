use dotenv::dotenv;
fn main() {
    dotenv().ok();
    let goove_api_token = std::env::var("GOVEE_API_KEY").expect("GOVEE_API_KEY must be set.");
    println!("Hello, world!");
    println!("GOVEE_API_KEY: {}", goove_api_token);
}
