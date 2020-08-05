use super::*;

pub fn routes(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("sub")
}

pub fn index(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let index = warp::ws()
        .and(warp::path!("index"))
        .and(warp::path::end())
        .and(using(data.clone()))
        .and(warp::body::json())
        .and_then(|data, body| async move {
            
        });
    index
}
