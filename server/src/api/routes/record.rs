use super::*;

pub fn routes(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("record").and(shared_with_user(db)
        .or(get(db))
        .or(create(db))
        .or(get_by_uid(db)))
}

// this should go in user
pub fn shared_with_user(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(using(db.to_owned()))
        .and(warp::body::json())
        .and(warp::path!("shared"))
        .and_then(handlers::get_shared_with)
}

pub fn get(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(using(db.to_owned()))
        .and(warp::path!(i32))
        .and_then(handlers::get_by_id)
}

// maybe have this route be: /:uid/records or /user/:uid/records?
pub fn get_by_uid(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(using(db.to_owned()))
        .and(warp::path!("uid" / i32))
        .and_then(handlers::get_by_uid)
}

pub fn create(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post().and(using(db.to_owned()))
        .and(warp::path!("new"))
        .and(warp::body::json())
        .and_then(handlers::add_record)
}

pub fn add_item(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post().and(using(db.to_owned()))
        .and(warp::path!("item"))
        .and(warp::body::json())
        .and_then(handlers::record::add_item)

}

pub fn add_new_item(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post().and(using(db.to_owned()))
        .and(warp::path!("item" / "new"))
        .and(warp::body::json())
        .and_then(handlers::record::add_new_item)

}
