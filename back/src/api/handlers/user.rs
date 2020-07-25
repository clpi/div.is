use super::*;

pub async fn get_user_by_username (
    db: Db, username: String
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::from_username(&db, username).await {
        Ok(user) => Ok(user.to_string()),
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
    println!("Adding record... ");
    println!("{}", serde_json::to_string(&record).unwrap());
    match record.insert(&db).await {
        Ok(record) => {
            Ok(serde_json::to_string(&record).unwrap())
        },    
        Err(_) => Err(warp::reject()),
    }
}
