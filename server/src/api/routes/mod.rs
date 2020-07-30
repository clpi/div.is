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
        .or(user::routes(&data.db.clone()))
        .or(record::routes(&data.db.clone()))
        .or(auth::routes(&data.clone()))
        .or(db::routes(&data.db.clone()))
        .or(all_users(&data.db.clone()))
        .or(record::get_by_uid(&data.db.clone())) //temp for testing
}

// TODO figure out why this doesn't work in user routes
//      (because of warp::path:;end lack?)
pub fn all_users(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(using(db.to_owned()))
        .and(warp::path!("all"))
        .and_then(handlers::get_all_users)
}
