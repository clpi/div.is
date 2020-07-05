use warp::filters::BoxedFilter;
use warp::Filter;
use crate::db::Db;

pub struct Routes {
    db: Db,
}

impl Routes {

    pub async fn new(db: Db) -> Self {
        Routes { db }
    }

    pub async fn hello() -> BoxedFilter<()> {
        warp::path("hello")
            .boxed()
    }

    // NOTE: Access to users records if user has cookie / etc.
    pub async fn record_by_id() -> BoxedFilter<(String,)> {
        warp::path!("record" / i32)
            .map(|uid: i32| "hw".to_string())
            .boxed()
    }

    pub async fn user_record() -> BoxedFilter<(String,)> {
        warp::path!("user" / "rec" / String)
            .map(|rec_name: String| rec_name)
            .boxed()
    }

    pub async fn index() -> BoxedFilter<(String,)> {
        warp::path!("index")
            .map(|| "Hello".to_string())
            .boxed()
    }         

    pub async fn sum() -> BoxedFilter<(String,)> {
         warp::path!("sum" / u32 / u32)
            .map(|a, b| format!("{} + {} = {}", a, b, a+b))
            .boxed()
    }

    //pub async fn get_user(self) -> BoxedFilter<(String,)> {
        //warp::get()
            //.and(self.db.clone())
            //.and(warp::path("user"))
            //.and(warp::path::param::<String>())
            //.and_then(handlers::get_user_by_username)
            //.boxed()
    //}

    //pub async fn login() -> BoxedFilter<(String,)> {
        //warp::post()
            //.and(wdb.clone())
            //.and(warp::path("login"))
            //.and(warp::body::json())
            //.and_then(handlers::login)
            //.boxed()
    //}

    //pub async fn register(self) -> BoxedFilter<(String,)> {
        //warp::post()
            //.and(self.db)
            //.and(warp::path("register"))
            //.and(warp::body::json())
            //.and_then(handlers::register)
            //.boxed()
    //}
}
