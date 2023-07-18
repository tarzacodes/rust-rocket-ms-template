#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate eventbus;

pub mod users;
pub mod infra;
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(infra::db::stage())
    .mount("/users/", users::user_controller::user_routes())
}
