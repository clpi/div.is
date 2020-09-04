pub mod auth;
pub mod user;
pub mod record;
pub mod db;

use warp::{Filter, http::Method};
use crate::db::Db;
use super::{AppData, handlers};

pub fn routes(data: AppData) -> impl Filter
        <Extract=(impl warp::Reply,), 
        Error=warp::Rejection> + Clone 
{

    //let hello = warp::get().and(warp::any()).map(|| { "hello" });
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

pub fn using<T: Clone + Send>(data: T) 
    -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data.clone())
}

// TODO figure out why this doesn't work in user routes
//      (because of warp::path:;end lack?)
pub fn all_users(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(using(db.to_owned()))
        .and(warp::path!("all"))
        .and_then(handlers::get_all_users)
}


// might be totally trash functions down here, maybe delete
//pub fn context<T: Clone + Send>(data: T) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    //warp::any().and(using(data))
        //.and(warp::path("context"))
        //.and(warp::path::end())
        ////.and_then()
//}



//pub fn check_auth(data: &AppData, filter: impl Filter) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    //warp::post()
        //.and(using(data.clone()))
        //.and(warp::path!("check"))
        //.and(warp::path::end())
        //.and(warp::body::json())
        //.and(warp::addr::remote())
//}
