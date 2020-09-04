use super::*;

pub fn routes(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("auth").and(
        login(data.clone())
            .or(register(&data))
            .or(refresh(&data))
            .or(logout(&data))
            .or(header_refresh(data.clone()))
    )
}

pub fn login(data: AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let login = warp::post()
        .and(using(data.clone()))
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(handlers::login)
        .with(warp::log::log("auth"));
    login
}

pub fn register(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let register = warp::post()
        .and(using(data.clone()))
        .and(warp::path("register"))
        .and(warp::body::json())
        .and_then(handlers::register)
        .with(warp::log::log("auth"));
    register
}
pub fn refresh(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let refresh = warp::get()
        .and(using(data.clone()))
        .and(warp::path!("refresh"))
        .and(warp::cookie::optional("Authorization"))
        .and_then(handlers::refresh)
        .with(warp::log::log("auth"))
        .with(warp::cors().allow_credentials(true)
            .allow_any_origin()
            .allow_methods(&[Method::GET])
            .allow_header("Access-Control-Allow-Origin"));
    refresh
}

pub fn logout(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let logout = warp::post()
        .and(using(data.clone()))
        .and(warp::path!("logout"))
        .and(warp::cookie::optional("Authorization"))
        .and_then(handlers::logout)
        .with(warp::log::log("auth"))
        .with(warp::cors().allow_credentials(true)
            .allow_any_origin()
            .allow_methods(&[Method::POST])
            .allow_header("Access-Control-Allow-Origin"));
    logout
}

pub fn header_refresh(data: AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(using(data))
        .and(warp::path("check_header"))
        .and(warp::header::optional::<String>("Authorization"))
        .and_then(handlers::auth::refresh)
}

// use as a base filter to protect calls
//pub fn has_token(data: &AppData) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    //warp::header("Authorization")
        //.or(warp::cookie("Authorization")).unify()
        //.map(Some).or(warp::any().map(|| None)).unify()
        //.and_then(|token: Option<String>| async {
            //match token {
                //Some(token) => Ok(token),
                //None => Ok(String::from("")),
            //}
        //})
//}

