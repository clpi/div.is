//use warp::Filter;
//use crate::api::{using, handlers};
//use crate::api::AppData;
//use crate::db::Db;
//use warp::filters::BoxedFilter;


//pub fn routes(data: AppData) -> impl Filter {
    //all(data.db.clone())
//}

//fn all(db: Db) -> impl Filter {
    //warp::get()
        //.and(warp::path("all"))
        //.and(using(db))
        //.and_then(handlers::get_all_users)
//}

//pub fn routes(data: AppData) -> BoxedFilter<()> {
   //warp::path("users")
       //.and(id::routes(data))
       //.and(warp::path::end())
       //.boxed()
//}

//mod id {
    //use super::*;

    //pub fn routes(data: Appdata) -> BoxedFilter<()> {
        //warp::path("id")
            //.and(get(data.db))
            //.and(del(data.db))
            //.and(warp::path::end())
            //.boxed()

    //}

    //fn get(db: Db) -> BoxedFilter<(String,)> {
        //warp::get()
            //.and(warp::path!(String))
            //.and(using(db))
            //.and_then(handlers::get_user_by_id)
            //.boxed()
    //}
    //fn del(db: Db) -> BoxedFilter<()> {
        //warp::get()
            //.and(warp::path("dfs"))
            //.and(using(db))
            //.and_then(handlers::get_all_users)
            //.boxed()
    //}
//}

//mod record {

//}

//pub fn get_by_id(db: Db) -> BoxedFilter<(String,)> {
//}

