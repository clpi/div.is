use warp::filters::BoxedFilter;
use warp::{Filter, http::Method};
use crate::db::Db;
use super::{AppData, using, handlers};

pub mod record;
pub mod auth;

// TODO Figure out how to do this
pub fn routes(data: AppData) -> impl Filter
        <Extract=(impl warp::Reply,), 
        Error=warp::Rejection> + Clone 
{
    let index = warp::path!("index")
        .map(|| "Hello");

    let sum = warp::path!("sum" / u32 / u32)
        .map(|a, b| format!("{} + {} = {}", a, b, a + b));

    let get_user = warp::get()
        .and(using(data.db.clone()))
        .and(warp::path!("user" / String))
        .and_then(handlers::get_user_by_username);

    let get_user_by_id = warp::get()
        .and(using(data.db.clone()))
        .and(warp::path!("user" / "id" / i32))
        .and_then(handlers::get_user_by_id);

    let get_all_users = warp::get()
        .and(using(data.db.clone()))
        .and(warp::path!("users"))
        .and_then(handlers::get_all_users);

    //let get_record = warp::get()
        //.and(using(data.db.clone()))
        //.and(warp::path!(i32 / "record" / String))
        //.and_then(handlers::get_record);

    let delete_user = warp::post()
        .and(using(data.db.clone()))
        .and(warp::path!("user" / String))
        .and_then(handlers::delete_user_by_username);

    let update_user = warp::put()
        .and(using(data.db.clone()))
        .and(warp::path!("user" / String))
        .and_then(handlers::update_user_by_username);

    // NOTE: AUTH ROUTES ------------->
    let login = warp::post()
        .and(using(data.clone()))
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(handlers::login);

    let register = warp::post()
        .and(using(data.clone()))
        .and(warp::path("register"))
        .and(warp::body::json())
        .and_then(handlers::register);

    let refresh = warp::get()
        .and(using(data.clone()))
        .and(warp::path!("userstatus"))
        .and(warp::cookie::optional("Authorization"))
        .and_then(handlers::check_cookie)
        .with(warp::cors().allow_credentials(true)
            .allow_any_origin()
            .allow_methods(&[Method::GET])
            .allow_header("Access-Control-Allow-Origin"));

    let logout = warp::post()
        .and(using(data.clone()))
        .and(warp::path!("logout"))
        .and(warp::cookie::optional("Authorization"))
        .and_then(handlers::logout)
        .with(warp::cors().allow_credentials(true)
            .allow_any_origin()
            .allow_methods(&[Method::POST])
            .allow_header("Access-Control-Allow-Origin"));

    // NOTE: DB ROUTES ------------------->
    let clear_db = warp::delete()
        .and(using(data.db.clone()))
        .and(warp::path!("db" / "clear"))
        .and_then(handlers::clear_database);

    let clear_db_table = warp::delete()
        .and(using(data.db.clone()))
        .and(warp::path!("db" / "clear" / String))
        .and_then(handlers::clear_table);

    let user_routes = 
        get_user
            .or(delete_user)
            .or(get_all_users)
            .or(get_user_by_id)
            .or(update_user);

    let auth_routes = 
        refresh
            .or(register)
            .or(login)
            .or(logout);

    let db_routes =
        clear_db
            .or(clear_db_table);

    let statc = warp::path("static").and(warp::fs::dir("../../client/public`"));

    index.or(auth_routes).or(user_routes).or(db_routes)
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
