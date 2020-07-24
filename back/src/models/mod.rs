pub mod entry;
pub mod user;
pub mod field;
pub mod item;
pub mod group;
pub mod record;
pub mod action;
pub mod link;

use chrono::Utc;
use serde::Serialize;

pub trait Model: Sized + Serialize { 
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub trait ModelLink { } 
pub fn now_ts() -> i32 {
    Utc::now().timestamp() as i32
}

//pub struct DbQuery<T: Model> {
    //pub db: Db,
    //pub model: T,
//}

//impl DbQuery<T: Model> {

    //pub async fn new<T: Model>(db: Db, model: T) -> DbQuery { 
        //DbQuery { db, model } 
    //}

    //pub async fn with_id<T: Model>(&self, id: i32) -> sqlx::Result<T> {
        //let res: Self = sqlx::query_as::<Sqlite, T>(
            //"SELECT * FROM ? WHERE id=?;")
            //.bind(self.model.to_string())
            //.bind(id)
            //.fetch_one(&self.db.pool).await?;
        //Ok(res)

    //}
    
//}
