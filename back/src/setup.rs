use dotenv::var;

pub fn setup() {

    let db = Db::new(&var("DATABASE_URL").unwrap()).await;

    let host = dotenv::var("DEV_HOST").expect("HOST not set");
    let port = dotenv::var("DEV_PORT").expect("PORT not set");
    
}
