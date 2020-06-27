mod db;

use warp::http::StatusCode;
use warp::Filter;
use db::models::*; //TODO: Merge models and schema files

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    
    let db = setup_db().await?;

    let (host, port) = get_host();

    let usr = db::models::User {
        id: None, email: String::from("chris@pecu.cc"),
        username: String::from("chrisp"),
        password: String::from("hashword"),
        created_at: sqlx::types::chrono::Utc::now().timestamp() as i32,
    };

    //usr.insert(db).await?; -> IT WORKS!

    let index = warp::path!("index")
        .map(|| StatusCode::OK);

    let api = index.with(warp::log("routes"));

    warp::serve(api).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}

pub async fn setup_db() -> sqlx::Result<db::Db> {
    let db_url = dotenv::var("DB_URL").expect("DB URL not set");
    Ok( db::Db::new(&db_url).await?.init().await? )
}

pub fn get_host() -> (String, String) {
    let host = dotenv::var("DEV_HOST").expect("HOST not set");
    let port = dotenv::var("DEV_PORT").expect("PORT not set");
    (host, port) 
}

