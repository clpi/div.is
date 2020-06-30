pub mod entries;
pub mod routes;
pub mod users;
pub mod auth;
pub mod handlers;

use super::db::models::*;
use super::db::Db;
use warp::filters::BoxedFilter;
use sqlx::types::chrono::{DateTime, Utc};
use serde_derive::*;

pub enum UsersRes {
    Login { username: String, password: String },
    Register { email: String, username: String, password: String },
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

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
