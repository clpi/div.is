use warp::Filter;
use warp::http::Response;
use warp::http::StatusCode;
use crate::db::models::*;
use crate::db::Db;
use chrono::Utc;
use super::{using, AppData,
    auth::{hash_pwd, verify_pwd, encode_jwt, decode_jwt, Claims}};

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
    pub privel: i32,
    pub created: i32,
}

impl SessionData {
    pub fn new(user: &User, token: &String, privelege: i32) -> Self {
        Self {
            uid: user.id.unwrap(), 
            token: token.to_owned(),
            username: user.username.clone(), 
            email: user.email.clone(),
            privel: privelege,
            created: Utc::now().timestamp() as i32,
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
    match Record::new(uid, name).insert(&db).await {
        Ok(ok) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn get_user_by_id(
    db: Db, id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_id(&db, id).await {
        Ok(user) => Ok(user.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn get_all_users(
    db: Db
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("fetching users...");
    match User::fetch_all(&db).await {
        Ok(users) => {
            Ok(serde_json::to_string(&users)
                .unwrap_or("No users".to_string())
            )
        },
        Err(_e) => {
            Err(warp::reject())

        }    
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
                let jwt = encode_jwt(&data.jwt_secret, &db_user).unwrap();
                let resp = SessionData::new(&db_user, &jwt, 0);
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

pub fn refresh(data: AppData) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::post().and(warp::path("auth"))
        .and(warp::path("check"))
        .and(using(data))
        .and(warp::cookie::optional("Authorization"))
        .and_then(check_cookie)
}

pub async fn login_header (
    data: AppData, req_user: UserLogin,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(&data.db, req_user.username).await {
        Ok(db_user) => {
            let (db_pwd, req_pwd) = (&db_user.password, &req_user.password);
            if verify_pwd(&data.secret_key, &req_pwd, &db_pwd).await {
                println!("LOGIN: Logged in {}", db_user.username);
                let jwt = encode_jwt(&data.jwt_secret, &db_user).unwrap();
                let resp = SessionData::new(&db_user, &jwt, 0);
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
                        serde_json::to_string(&claims).unwrap(),
                        "Authorization", "false",
                    )
                )
                },
                Err(_) => {
                    println!("Cookie is bad");
                    Err(warp::reject())
                }
            }
        },
        None => {
            println!("No cookie");
            Err(warp::reject())
        }
    }
}

// TODO actually implement
// TODO make fn for match cookie => some token => match decode jwt
pub async fn logout(data: AppData, cookie: Option<String>)
    -> Result<impl warp::Reply, warp::Rejection> {
    match cookie {
        Some(token) => {
            match decode_jwt(&data.jwt_secret, &token) {
                Ok(_) => {
                    Ok(warp::reply::with_header(
                        StatusCode::OK.to_string(),
                        warp::http::header::SET_COOKIE,
                        format!("Authorization={}", "")
                    )
                )},
                Err(_) => Err(warp::reject()),
            }
        },
        None => Err(warp::reject()),
    }
}

pub async fn delete_user_by_username(
    db: Db, username: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::delete_by_username(&db, username).await {
        Ok(_o) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

// TODO: Implement
pub async fn update_user_by_username(
    db: Db, username: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::delete_by_username(&db, username).await {
        Ok(_o) => Ok(StatusCode::OK.to_string()),
        Err(_e) => Ok(StatusCode::BAD_REQUEST.to_string()),
    }
}

pub async fn get_record_by_id(
    db: Db, id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Record::from_id(&db, id).await {
        Ok(record) => Ok(record.to_string()),
        Err(_) => Err(warp::reject()),
    }
}

pub async fn get_user_records(
    db: Db, user: User
) -> Result<impl warp::Reply, warp::Rejection> {
    match user.get_records(&db).await {
        Ok(records) => Ok(serde_json::to_string(&records).unwrap()),
        Err(_) => Err(warp::reject())
    }
}

pub async fn add_record(
    db: Db, record: Record,
) -> Result<impl warp::Reply, warp::Rejection> {
    match &record.insert(&db).await {
        Ok(rec) => {
            let (uid, rid) = (rec.uid, rec.id.unwrap());
            match UserRecordLink::create(&db, uid, rid).await {
                Ok(_) => Ok(rec.to_string()),
                Err(_) => Err(warp::reject()),
            }
        },    
        Err(_) => Err(warp::reject()),
    }
}

pub async fn clear_database(
    db: Db
) -> Result<impl warp::Reply, warp::Rejection> {
    match db.clear().await {
        Ok(_) => Ok(StatusCode::OK.to_string()),
        Err(_) => Err(warp::reject()),
    }
}

pub async fn clear_table(
    db: Db, table: String
) -> Result<impl warp::Reply, warp::Rejection> {
    match db.clear_table(table.as_str()).await {
        Ok(_) => Ok(StatusCode::OK.to_string()),
        Err(_) => Err(warp::reject()),
    }
}

