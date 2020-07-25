use super::*;

pub fn routes(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("record").and(shared_with_user(db)
        .or(get(db)))
        .or(create(db))
}

// this should go in user
pub fn shared_with_user(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let get_records_shared_with = warp::get()
        .and(using(db.to_owned()))
        .and(warp::body::json())
        .and(warp::path!("shared"))
        .and_then(handlers::get_records_shared_with);
    get_records_shared_with 
}

pub fn get(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let get_record = warp::get()
        .and(using(db.to_owned()))
        .and(warp::path!(i32))
        .and_then(handlers::get_record_by_id);
    get_record
}

pub fn create(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post().and(using(db.to_owned()))
        .and(warp::path!("new"))
        .and(warp::body::json())
        .and_then(handlers::add_record)
}
