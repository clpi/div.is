mod db;
mod routes;

use std::net::SocketAddr;
use warp::http::StatusCode;
use warp::{Filter, self};
use db::models::*; //TODO: Merge models and schema files
use db::Db;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    
    let db = setup_db().await?;
    let cors = warp::cors().allow_any_origin();

    let (host, port) = get_host();

    ex_user().insert(db).await?; // -> IT WORKS!

    let index = warp::path!("index")
        .map(|| "Hello");
    let sum = warp::path!("sum" / u32 / u32)
        .map(|a, b| format!("{} + {} = {}", a, b, a+b));
    let get_user_by_username = warp::get()
        .and(warp::path::param())
        .map(move |u: String| format!("hey {}", u));

    let api = index.or(sum)
        .with(warp::log("routes"))
        .with(cors);

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}

pub async fn get_username(db: Db, username: String) -> String {
    User::from_username(db, username).await.unwrap().to_string()
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
