use super::*;
use crate::api::auth::{hash_pwd, verify_pwd, encode_jwt, decode_jwt, Claims};
use chrono::Utc;
use crate::models::user::UserInfo;


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
                let headers = warp::http::HeaderMap::new();
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
    match &req_user.insert(data.db).await {
        Ok(user) => {
            println!("Registered {}", user.username);
            Ok(warp::reply::with_header(
                StatusCode::OK.to_string(),
                warp::http::header::SET_COOKIE,
                format!("SESS={}", user.username)))
        },
        Err(_e) => {
            println!("Could not insert user into DB");
            Err(warp::reject())
        }    
    }
}

// TODO implement oauth 2 instead of jwt
pub async fn refresh(data: AppData, cookie: Option<String>) 
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
