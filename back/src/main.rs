mod db;
mod api;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use warp::{Filter, self};
use db::models::*; //TODO: Merge models and schema files
use db::Db;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    
    let db = setup_db().await?;
    
    let wdb = warp::any().map(move || db.clone());
    let cors = warp::cors().allow_any_origin();

    let (host, port) = get_host();
    //ex_user().insert(wdb).await?; // -> IT WORKS!

    let index = warp::path!("index")
        .map(|| "Hello");
    let sum = warp::path!("sum" / u32 / u32)
        .map(|a, b| format!("{} + {} = {}", a, b, a+b));
    let get_user = warp::get()
        .and(wdb.clone())
        .and(warp::path("user"))
        .and(warp::path::param())
        .and_then(get_user_by_username);

    let routes = index.or(sum).or(get_user);

    let api = warp::path("api")
        .and(routes)
        .with(cors);

    warp::serve(api).run(([127, 0, 0, 1], 3001)).await;
    Ok(())
}

pub async fn get_user_by_username(
    db: Db, username: String
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(db, username).await {
        Ok(user) => Ok(user.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn get_user_by_id(
    db: Db, id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_id(db, id).await {
        Ok(user) => Ok(user.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn login (
    db: Db, req_user: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(db, req_user.username).await {
        Ok(db_user) => {
            if db_user.password == req_user.password {
                Ok(StatusCode::OK.to_string())
            } else {
                Ok(StatusCode::UNAUTHORIZED.to_string())
            }
        },
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn register (
    db: Db, user: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match user.insert_into(db).await {
        Ok(_o) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn delete_user_by_username(
    db: Db, user: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match user.delete_from(db).await {
        Ok(_o) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
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

// NOTE: Remember, now is not the time to overengineer things
