use warp::Filter;
use warp::http::Response;
use warp::http::StatusCode;
use crate::db::models::*;
use crate::db::Db;
use crate::api::AppData;
use super::auth::{hash_pwd, verify_pwd, encode_jwt, decode_jwt};

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionData { 
    pub uid: i32,
    pub token: String,
    pub email: String,
    pub username: String,
}

impl SessionData {
    pub fn new(user: &User, token: &String) -> Self {
        Self {
            uid: user.id.unwrap(), 
            token: token.to_owned(),
            username: user.username.clone(), 
            email: user.email.clone()
        }
    }
}

pub async fn test(
    name: String
) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("hi {}", name);
    Ok(warp::reply::html(reply))
}

pub async fn get_user_by_username (
    db: Db, username: String
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(&db, username).await {
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
        Ok(users) => {
            println!("OK! {}", &serde_json::to_string(&users[..2]).unwrap());
            Ok(warp::reply::json(
                &serde_json::to_string(&users)
                .unwrap_or("No users".to_string()))
            )
        },
        Err(_e) => Err(warp::reject()),
    }
}

// TODO implement hashing verification
// TODO first match should be some vs none -> impl option for user::from_username
// NOTE check header is set: $curl -x POST -c header.txt localhost:3001/api/login && cat header.txt
// TODO implement custom response types wrapperes and error handling
pub async fn login (
    data: AppData, req_user: UserLogin,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(&data.db, req_user.username).await {
        Ok(db_user) => {
            let (db_pwd, req_pwd) = (&db_user.password, &req_user.password);
            if verify_pwd(&data.secret_key, &req_pwd, &db_pwd).await {
                println!("LOGIN: Logged in {}", db_user.username);
                let jwt = encode_jwt(&data.jwt_secret, db_user.id.unwrap()).unwrap();
                let resp = SessionData::new(&db_user, &jwt);
                Ok(warp::reply::with_header(
                    serde_json::to_string(&resp).unwrap(),
                    warp::http::header::SET_COOKIE,
                    format!("Authorization={}", jwt)
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
    data: AppData, req_user: User,
) -> Result<impl warp::Reply, warp::Rejection> {
    let req_user = req_user.hash_pass(data.secret_key).await.unwrap();
    println!("Hashed user password");
    match req_user.insert_into(data.db).await {
        Ok(user) => {
            println!("Inserted user into DB");
            Ok(warp::reply::with_header(
                StatusCode::OK.to_string(),
                warp::http::header::SET_COOKIE,
                format!("SESS={}", user.username)
            ))
        },
        Err(_e) => {
            println!("Could not insert user into DB");
            Err(warp::reject())
        }    
    }
}

pub async fn check_cookie(data: AppData, cookie: Option<String>) 
    -> Result<impl warp::Reply, warp::Rejection> {
    println!("Checking cookie...");
    match cookie {
        Some(token) => {
            println!("Cookie: {}", token);
            match decode_jwt(&data.jwt_secret, &token) {
                Ok(claims) => {
                    println!("Cookie is good");
                    Ok(warp::reply::with_header(
                            warp::reply::json(
                                &serde_json::to_string(&claims).unwrap()
                            ), "Authorization", "true"
                        )
                    )
                },
                Err(_) => {
                    println!("Cookie is bad");
                    Ok(warp::reply::with_header(
                        warp::reply::json(&token), "Authorization", "false"
                    ))
                }
            }
        },
        None => {
            println!("No cookie");
            Err(warp::reject())

        }
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
