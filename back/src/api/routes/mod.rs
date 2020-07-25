pub mod auth;
pub mod user;
pub mod record;
pub mod db;

use warp::{Filter, http::Method};
use crate::db::Db;
use super::{AppData, using, handlers};

pub fn routes(data: AppData) -> impl Filter
        <Extract=(impl warp::Reply,), 
        Error=warp::Rejection> + Clone 
{

    let index = warp::path("index").and(warp::fs::file("../../static/index.html")).map(|index| warp::reply::html("<h1>hello</h1>"));
    let s_dir = warp::path("static").and(warp::fs::dir("../../static"));

    index.or(s_dir)
        .or(user::routes(&data.db))
        .or(record::routes(&data.db))
        .or(auth::routes(&data))
        .or(db::routes(&data.db))
}
