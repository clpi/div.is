pub mod entries;
pub mod routes;
pub mod users;
pub mod auth;
pub mod handlers;

use serde_derive::*;
use crate::db::Db;

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

//#[derive(Serialize, Deserialize)]
//pub struct AppData {
    //pub jwt_secret: String,
    //pub secret_key: argonautica::input::SecretKey,
//}

//impl Into<User> for UserLogin {
    //fn into(self) -> User {
        //User {
            //id: None, email: self.email,
            //username: self.username, 
            //password: self.password,
            //created_at: Utc::now().timestamp() as i32,
        //}
    //}
//}
