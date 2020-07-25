use super::*;

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
