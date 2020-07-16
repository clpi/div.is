use warp::filters::BoxedFilter;
use warp::Filter;
use crate::db::Db;
use super::handlers;
use super::AppData;
use super::using;

pub mod record;
pub mod auth;

// TODO Figure out how to do this

pub fn routes(data: AppData) -> BoxedFilter<(impl warp::Reply,)> {
    user::routes(data.db.clone())
        .or(test())
        .boxed()
}

pub mod user {

    use super::*;

    pub fn routes(db: Db) -> BoxedFilter<(impl warp::Reply,)> {
        username(db.clone())
            .or(id(db.clone()))
            .boxed()
    }

    pub fn username(db: Db) -> BoxedFilter<(impl warp::Reply,)> {
        using(db).and(warp::path!("username" / String))
            .and_then(handlers::get_user_by_username)
            .boxed()
    }

    pub fn id(db: Db) -> BoxedFilter<(impl warp::Reply,)> {
        using(db)
            .and(warp::path!("id" / String))
            .and_then(handlers::get_user_by_username)
            .boxed()
    }

}


    // NOTE POST /api/login
    // TODO Fix login handler

    //// NOTE POST /api/register
    //let register = warp::post()
        //.and(with_data(app_data.clone()))
        //.and(warp::path("register"))
        //.and(warp::body::json())
        //.and_then(handlers::register);

    //let check_cookie = warp::get()
        //.and(with_data(app_data.clone()))
        //.and(warp::path!("userstatus"))
        //.and(warp::cookie::optional("Authorization"))
        //.and_then(handlers::check_cookie)
        //.with(warp::cors().allow_credentials(true)
            //.allow_any_origin()
            //.allow_methods(&[Method::GET])
            //.allow_header("Access-Control-Allow-Origin"));

    //// NOTE DELETE /user/<username>
    //let delete_user = warp::post()
        //.and(with_db(app_data.db.clone()))
        //.and(warp::path!("user" / String))
        //.and_then(handlers::delete_user_by_username);

    //// NOTE UPDATE /user/<username>
    //let update_user = warp::put()
        //.and(with_db(app_data.db.clone()))
        //.and(warp::path!("user" / String))
        //.and_then(handlers::update_user_by_username);

    //let clear_db = warp::delete()
        //.and(with_db(app_data.db.clone()))
        //.and(warp::path!("db" / "clear"))
        //.and_then(handlers::clear_database);

    //let clear_db_table = warp::delete()
        //.and(with_db(app_data.db.clone()))
        //.and(warp::path!("db" / "clear" / String))
        //.and_then(handlers::clear_table);

pub fn test() -> BoxedFilter<(String,)> {
    warp::path!("test" / String / String )
        .map(|s1: String, s2: String| {
            format!("What's up {} {}", s1, s2)
        })
        .boxed()
}
