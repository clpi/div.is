use super::*;

pub async fn get_by_id(
    db: Db, id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Record::from_id(&db, id).await {
        Ok(record) => Ok(serde_json::to_string(&record).unwrap()),
        Err(_) => Err(warp::reject()),
    }
}

pub async fn get_by_uid(
    db: Db, uid: i32 
) -> Result<impl warp::Reply, warp::Rejection> {
    match Record::from_uid(&db, uid).await {
        Ok(records) => {
            println!("{}", serde_json::to_string(&records).unwrap());
            Ok(serde_json::to_string(&records).unwrap())
        },
        Err(_) => Err(warp::reject())
    }
}

pub async fn get_shared_with(
    db: Db, user: User
) -> Result<impl warp::Reply, warp::Rejection> {
    match Record::associated_with_user(&db, &user).await {
        Ok(records) => Ok(serde_json::to_string(&records).unwrap()),
        Err(_) => Err(warp::reject())
    }
}

pub async fn create(
    db: Db, record: Record,
) -> Result<impl warp::Reply, warp::Rejection> {
    match record.insert(&db).await {
        Ok(record) => {
            Ok(serde_json::to_string(&record).unwrap())
        },    
        Err(_) => Err(warp::reject()),
    }
}

pub async fn add_item(
    db: Db, record: Record,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok("todo".to_string())
}

pub async fn add_new_item(
    db: Db, record: Record,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok("todo".to_string())
}

