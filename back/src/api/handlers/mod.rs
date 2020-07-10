use warp::Filter;
use warp::http::Response;
use warp::http::StatusCode;
use crate::db::models::*;
use crate::db::Db;
use crate::api::UserLogin;
use warp::http::header::{HeaderMap, HeaderValue};

pub async fn test(
    name: String
) -> Result<impl warp::Reply, warp::Rejection> {
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
    match Record::new(uid, name).insert(db).await {
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

pub async fn get_all_users(
    db: Db
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::fetch_all(db).await {
        Ok(users) => Ok(users.get(0).unwrap().to_string()), //TODO return all in json string
        Err(_e) => Err(warp::reject()),
    }
}

// TODO implement hashing verification
// NOTE check header is set: $curl -x POST -c header.txt localhost:3001/api/login && cat header.txt
pub async fn login (
    db: Db, req_user: UserLogin,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(db, req_user.username).await {
        Ok(db_user) => {
            if db_user.password == req_user.password {
                println!("LOGIN: Logged in {}", db_user.username);
                Ok(warp::reply::with_header(
                    StatusCode::OK.to_string(),
                    warp::http::header::SET_COOKIE,
                    format!("SESS={}", db_user.username)
                ))
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

// TODO implement hashing creation
pub async fn register (
    db: Db, req_user: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    match &req_user.insert_into(db).await {
        Ok(user) => {
            Ok(warp::reply::with_header(
                StatusCode::OK.to_string(),
                warp::http::header::SET_COOKIE,
                format!("SESS={}", user.username)
            ))
        },
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

// TODO: Implement
pub async fn update_user_by_username(
    db: Db, username: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::delete_by_username(db, username).await {
        Ok(_o) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn add_record(
    db: Db, user_id: i32, record_id: i32, privelege: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    // first create record! TODO then insert in db! TODO
    match UserRecordLink::create(db, user_id, record_id, privelege).await {
        Ok(_) => Ok(String::from("Created record")),
        Err(_) => Err(warp::reject()),
    }
}
