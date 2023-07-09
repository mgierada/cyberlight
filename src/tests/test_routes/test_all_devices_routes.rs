#[cfg(test)]
pub mod tests {
    use crate::rocket;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_get_all_devices_handler() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/devices").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more assertions to validate the response body
    }

    #[test]
    fn test_get_status_for_all_devices() {
        let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
        let response = client.get("/status").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Add more assertions to validate the response body
    }
    //
    // #[test]
    // fn test_get_status_for_device() {
    //     let client = Client::untracked(crate::rocket()).expect("valid rocket instance");
    //     let response = client.get("/status/device/model").dispatch();
    //     assert_eq!(response.status(), Status::Ok);
    //     // Add more assertions to validate the response body
    // }
}
