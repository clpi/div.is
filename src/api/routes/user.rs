use super::*;

//TODO decide whether to implement in one big routes fn
//     or in separate function = separate route?
//TODO figure out if db/data passing should be ref
//     or if that's dangerous and should use cloning?
//TODO create a macro for these routes / find other way
//     to reduce redundancy?
//TODO further modularize into get/post/etc?
pub fn routes(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("user").and(get_by_username(db)
            .or(get_by_id(db))
            .or(all(db))
            .or(update(db))
            .or(delete(db))
            .or(add_record(db))
            .or(get_records(db)))
}

pub fn get_by_username(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(using(db.to_owned()))
        .and(warp::path!(String))
        .and_then(handlers::get_user_by_username)
}

pub fn get_by_id(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(using(db.to_owned()))
        .and(warp::path!("id" / i32))
        .and_then(handlers::user::get_user_by_id)
}

pub fn all(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(using(db.to_owned()))
        .and(warp::path!("all"))
        .and_then(handlers::user::get_all_users)
}

pub fn get_records(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get().and(using(db.to_owned()))
        .and(warp::path!("record"))
        .and(warp::body::json())
        .and_then(handlers::user::get_user_records)
}

// TODO update by id
pub fn update(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::put().and(using(db.to_owned()))
        .and(warp::path!(String))
        .and_then(handlers::user::update_user_by_username)
}

// TODO update by id
pub fn delete(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::delete().and(using(db.to_owned()))
        .and(warp::path!(String))
        .and_then(handlers::user::delete_user_by_username)
}

// TODO make this user/:id/record to and add id, sep. route in records
pub fn add_record(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post().and(using(db.to_owned()))
        .and(warp::path!("record"))
        .and(warp::body::json())
        .and_then(handlers::user::add_record)
}
