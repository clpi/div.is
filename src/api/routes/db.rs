use super::*;

// TODO add api credentials to base route filter
pub fn routes(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("db").and(clear_all(db)
            .or(clear_table(db)))
}

pub fn clear_all(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::delete().and(using(db.to_owned()))
        .and(warp::path!("clear"))
        .and_then(handlers::clear_database)
} 

pub fn clear_table(db: &Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::delete().and(using(db.to_owned()))
        .and(warp::path!("clear" / String))
        .and_then(handlers::clear_table)
}

pub fn create_table(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::post().and(using(db))
        .and(warp::path!("table"))
        .and_then(handlers::clear_database)
} 
