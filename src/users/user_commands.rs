use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum UserCommand {
    CreateUser(UserCommandPayload),
    DeleteUser { id: i32 },
}

#[derive(Debug, Deserialize)]
pub struct UserCommandPayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String
}