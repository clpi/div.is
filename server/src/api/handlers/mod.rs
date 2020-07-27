pub mod user;
pub mod auth;
pub mod record;
pub mod db;
pub mod item;
pub mod field;

use warp::Filter;
use crate::db::Db;
use warp::http::StatusCode;
use crate::api::AppData;
use crate::models::{
    user::User, record::Record, item::Item,
    link::{UserRecordLink},
};

pub use user::*;
pub use record::*;
pub use db::*;
pub use auth::*;

pub fn respond(resp: Result<String, serde_json::Error>) 
    -> warp::http::Response<String> {
    match resp {
        Ok(body) => warp::http::Response::builder()
            .status(warp::http::StatusCode::OK)
            .header("content-type", "applicaiton/json")
            .body(body)
            .expect("Valid response"),
        Err(_) => warp::http::Response::builder()
            .status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(String::from(""))
            .expect("Invalid response")
    }
}


    










