use rocket::response::Redirect;

#[get("/")]
pub fn home() -> Redirect {
    Redirect::to("/status")
}
