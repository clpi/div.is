use warp::Filter;
use warp::http::Response;
use warp::http::StatusCode;
use crate::db::models::*;
use crate::db::Db;
use crate::api::UserLogin;
use warp::http::header::{HeaderMap, HeaderValue};


pub async fn test(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("hi {}", name);
    Ok(warp::reply::html(reply))
}

pub async fn get_user_by_username (
    db: Db, username: String
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(db, username).await {
        Ok(user) => Ok(user.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn create_user_record (
    db: Db, uid: i32, name: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Record::new(uid, name).insert_db(db).await {
        Ok(ok) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn get_user_by_id(
    db: Db, id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_id(db, id).await {
        Ok(user) => Ok(user.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn login (
    db: Db, req_user: UserLogin,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(db, req_user.username).await {
        Ok(db_user) => {
            if db_user.password == req_user.password {
                let mut headers = HeaderMap::new();
                headers.insert("loginCookie", HeaderValue::from_static("coooookie"));
                Ok(StatusCode::OK.to_string())
            } else {
                println!("LOGIN: Incorrect password for {}", db_user.username);
                Err(warp::reject())
            }
        },
        Err(_e) => {
            println!("LOGIN: No user with username found");
            Err(warp::reject::not_found())
        },
    }
}

pub async fn register (
    db: Db, req_user: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match req_user.insert_into(db).await {
        Ok(_o) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Err(warp::reject()),
    }
}

pub async fn delete_user_by_username(
    db: Db, username: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::delete_by_username(db, username).await {
        Ok(_o) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}
