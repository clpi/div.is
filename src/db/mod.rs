pub mod sqlite;

// TODO figure out what the hell is going on with postgres (i don't
// think it's in-code setup, likely my enviroment?) or return to
// sqlite until you do
use sqlx::*;
use std::{path::Path, fs};
use crate::models::user::User;

pub use sqlite::Db;

pub async fn setup() -> sqlx::Result<Db> {
    let db_url;
    let env = std::env::var("ENVIRONMENT");
    if env.is_ok() {
        db_url = match env.unwrap().as_str() {
            "PROD" => std::env::var("PROD_DB_URL").expect("PROD_DB_URL not set"),
            _ => std::env::var("DEV_DB_URL").expect("DEV_DB_URL not set"),
        }
    } else {
        dbg!(db_url = dotenv::var("DB_URL").expect("DB_URL not set in .env"));
    }
    println!("{}", db_url);
    Ok( Db::new(&db_url).await? )
}
