use super::*;

pub fn routes(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("item").and(create(db)
        .or(get(db)))
        .or(create(db))
}

pub fn create(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("item").and(shared_with_user(db)
        .or(get(db)))
        .or(create(db))
}

pub fn get(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("item").and(shared_with_user(db)
        .or(get(db)))
        .or(create(db))
}
