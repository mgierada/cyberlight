#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_home_redirects_to_status() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::SeeOther);
        let location = response.headers().get_one("Location");
        assert_eq!(location, Some("/status"));
    }
}
