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

// TODO get async trait and make this an async trait, THEN
//      add get_by_id, del_by_id, update_by_id
pub trait Model: Sized + Serialize { 
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub trait ModelLink { } 

//TODO rename private field to permission field
//TODO figure out how to best use enums // best practices for them
//TODO think this through -- all enums
pub enum Time {
    Now,
    Tomorrow,
    DayAfterTomorrow,
    WeekLater,
    MonthLater,
}

impl Time {
    pub fn now() -> i32 { Utc::now().timestamp() as i32 }
}

pub enum Status {
    Archived,    
    Deleted,
    Active,
}

impl Status {
    pub fn active() -> String { "active".to_string() }
    pub fn archived() -> String { "archived".to_string() }
}

// TODO think this through
pub enum Permission {
    Public,
    Private,
    Selective,
}

impl Permission {
    pub fn private() -> bool { true }
}

pub enum Priority {
    Lowest,
    Highest,
}
impl Priority {
    pub fn lowest() -> String { "lowest".to_string() }
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
