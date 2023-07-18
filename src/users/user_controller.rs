use rocket::http::Status;
use rocket::serde::json::Json;

use super::user_commands::{UserCommand, UserCommandPayload};

pub fn user_routes() -> Vec<rocket::Route> {
    routes![
        create,
        hello,
    ]
}

#[post("/create", format = "application/json", data = "<data>")]
pub fn create(data: Json<UserCommandPayload>) -> Status {
    let command = UserCommand::CreateUser(data.0);
    // RegisterUserCommandHandler::new().handle(command);
    Status::Ok
}

#[get("/hello")]
pub fn hello() -> Status {
    println!("HELLO!");
    Status::Ok
}
