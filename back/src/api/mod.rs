pub mod routes;
pub mod auth;
pub mod handlers;

use serde_derive::*;
use crate::db::Db;
use warp::Filter;

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppData {
    pub jwt_secret: String,
    pub secret_key: String,
    pub db: Db,
}

pub fn using<T: Clone + Send>(data: T) 
    -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data.clone())
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
