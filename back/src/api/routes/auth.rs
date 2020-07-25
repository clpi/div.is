use super::*;

pub fn routes(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("auth").and(
        login(&data)
            .or(register(&data))
            .or(refresh(&data))
            .or(logout(&data))
    )
}

pub fn login(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let login = warp::post()
        .and(using(data.to_owned()))
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(handlers::login);
    login
}

pub fn register(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let register = warp::post()
        .and(using(data.to_owned()))
        .and(warp::path("register"))
        .and(warp::body::json())
        .and_then(handlers::register);
    register
}
pub fn refresh(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let refresh = warp::get()
        .and(using(data.to_owned()))
        .and(warp::path!("refresh"))
        .and(warp::cookie::optional("Authorization"))
        .and_then(handlers::refresh)
        .with(warp::cors().allow_credentials(true)
            .allow_any_origin()
            .allow_methods(&[Method::GET])
            .allow_header("Access-Control-Allow-Origin"));
    refresh
}

pub fn logout(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let logout = warp::post()
        .and(using(data.to_owned()))
        .and(warp::path!("logout"))
        .and(warp::cookie::optional("Authorization"))
        .and_then(handlers::logout)
        .with(warp::cors().allow_credentials(true)
            .allow_any_origin()
            .allow_methods(&[Method::POST])
            .allow_header("Access-Control-Allow-Origin"));
    logout
}




