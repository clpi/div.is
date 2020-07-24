#[macro_use]
extern crate serde;
extern crate argonautica;
mod db;
mod api;
mod models;

use warp::{Filter, self};
use warp::http::Method;
use api::routes;

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

    let (host, port) = hosts();
    let db = db().await?;

    let app_data = api::AppData {
        jwt_secret: api::auth::get_jwt_secret().await.unwrap(),
        secret_key: api::auth::get_secret_key().await.unwrap(),
        db: db,
    };

    let routes = warp::path("api").and(routes::routes(app_data)).with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 3001)).await;
    Ok(())
}

pub async fn db() -> sqlx::Result<db::Db> {
    //let db_type = dotenv::var("DB_TYPE").expect("SQLite");
    let db_url = dotenv::var("DB_URL").expect("DB_URL not set");
    Ok( db::Db::new(&db_url).await? )
}

pub fn hosts() -> (String, String) {
    let host = dotenv::var("DEV_HOST").expect("DEV_HOST not set");
    let port = dotenv::var("DEV_PORT").expect("DEV_PORT not set");
    (host, port) 
}

