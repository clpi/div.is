pub mod user;
pub mod auth;
pub mod record;
pub mod db;

use warp::Filter;
use std::collections::HashMap;
use crate::db::Db;
use warp::http::StatusCode;
use crate::api::AppData;
use crate::models::{
    user::User, record::Record, item::Item,
    link::{UserRecordLink},
};
use uuid;

pub use user::*;
pub use record::*;
pub use db::*;
pub use auth::*;




    









