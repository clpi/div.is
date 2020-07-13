#[macro_use]
extern crate serde;
mod db;
mod api;

use warp::{Filter, self};
use db::models::*; //TODO: Merge models and schema files
use self::api::handlers;
use std::collections::HashMap;
use warp::http::Method;
use db::Db;

// TODO: Created BoxedFilter routes in api/routes/*.rs modules
// TODO: Implement CRUD for basic records
// TODO learn about borowing + references - i.e. is referencing Strings stupid??

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    
    let cors = warp::cors()
        .allow_credentials(true)
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", 
            "Referer", "Origin", "Access-Control-Request-Method", 
            "Access-Control-Request-Headers", "Access-Control-Allow-Origin", 
            "content-type", "credentials",
        ])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ]);

    let (host, port) = get_host();
    let app_data = api::AppData {
        jwt_secret: api::auth::get_jwt_secret().await.unwrap(),
        secret_key: api::auth::get_secret_key().await.unwrap(),
        db: setup_db().await?,
    };

    let index = warp::path!("index")
        .map(|| "Hello");
    let sum = warp::path!("sum" / u32 / u32)
        .map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // NOTE GET /api/user/<username>
    let get_user = warp::get()
        .and(with_db(app_data.db.clone()))
        .and(warp::path!("user" / String))
        .and_then(handlers::get_user_by_username);

    let get_all_users = warp::get()
        .and(with_db(app_data.db.clone()))
        .and(warp::path!("user" / "all"))
        .and_then(handlers::get_all_users);

    let get_record = warp::get()
        .and(with_db(app_data.db.clone()))
        .and(warp::path!("user" / String / "record" / String))
        .map(|db: Db, u: String, r: String| {
            format!("{}, {}", u, r)
        });

    // NOTE POST /api/login
    // TODO Fix login handler
    let login = warp::post()
        .and(with_data(app_data.clone()))
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(handlers::login);

    // NOTE POST /api/register
    let register = warp::post()
        .and(with_data(app_data.clone()))
        .and(warp::path("register"))
        .and(warp::body::json())
        .and_then(handlers::register);

    let check_cookie = warp::get()
        .and(with_data(app_data.clone()))
        .and(warp::path!("userstatus"))
        .and(warp::cookie::optional("Authorization"))
        .and_then(handlers::check_cookie)
        .with(warp::cors().allow_credentials(true)
            .allow_any_origin()
            .allow_methods(&[Method::GET])
            .allow_header("Access-Control-Allow-Origin"));

    // NOTE DELETE /user/<username>
    let delete_user = warp::post()
        .and(with_db(app_data.db.clone()))
        .and(warp::path!("user" / String))
        .and_then(handlers::delete_user_by_username);

    // NOTE UPDATE /user/<username>
    let update_user = warp::put()
        .and(with_db(app_data.db.clone()))
        .and(warp::path!("user" / String))
        .and_then(handlers::update_user_by_username);

    // NOTE: /api/user/
    let user_actions = get_user
        .or(delete_user)
        .or(get_all_users)
        .or(update_user)
        .or(check_cookie)
        .or(register)
        .or(login);

    let routes = index.or(sum).or(user_actions);

    // TODO register appdata with all routes, exclude from non needed ones
    let api = warp::path("api")
        .and(routes) 
        .with(cors);

    warp::serve(api).run(([127, 0, 0, 1], 3001)).await;
    Ok(())
}


pub async fn setup_db() -> sqlx::Result<db::Db> {
    let db_url = dotenv::var("DB_URL").expect("DB_URL not set");
    Ok( db::Db::new(&db_url).await? )
}

pub fn get_host() -> (String, String) {
    let host = dotenv::var("DEV_HOST").expect("DEV_HOST not set");
    let port = dotenv::var("DEV_PORT").expect("DEV_PORT not set");
    (host, port) 
}


pub fn ex_user() -> User {
    db::models::User {
        id: None, email: String::from("chris@pecu.cc"),
        username: String::from("chrisp"),
        password: String::from("hashword"),
        created_at: sqlx::types::chrono::Utc::now().timestamp() as i32,
    }
}

fn with_data(data: api::AppData) 
    -> impl Filter<Extract = (api::AppData,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || data.clone())
}

fn with_db(db: Db)
    -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// NOTE: Remember, now is not the time to overengineer things
