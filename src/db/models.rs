use serde_derive::*;
use sqlx::*;
use sqlx::sqlite::*;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use crate::db::Db;

//TODO Consider adding custom types for forieng key references, using
//sqlx::Type and transparent

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: i32,
}

// TODO: Eventually make records collaborable, not associated with single user
// TODO: make collaborative records have anonymous option
//#[derive(Serialize, Deserialize)]
#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct Record {
    pub id: Option<i32>,
    pub uid: i32, //user id
    pub name: String,
    pub status: i32,
    pub private: bool,
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<i32>,
    pub uid: i32,
    pub pid: Option<i32>, // parent item id
    pub name: String,
    pub status: i32,
    pub private: bool,
    pub created_at: i32,
}

// NOTE: typ can mean cardinally valued (ie checkboxes), stateful, textbox, etc.
#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct Field {
    pub id: Option<i32>,
    pub name: String,
    pub typ: String,
    pub value: String,
    pub private: bool, 
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct RecordItemLink {
    pub id: Option<i32>,
    pub rid: i32,
    pub iid: i32,
    pub status: i32,
    pub priority: i32,
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct ItemFieldLink {
    pub id: Option<i32>,
    pub iid: i32,
    pub fid: i32,
    pub status: i32,
    pub priority: i32,
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct FieldEntryLink {
    pub id: Option<i32>,
    pub fid: i32,
    pub etid: i32,
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct EntryType {
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: i32,
    pub private: bool,
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct EntryEntry {
    pub id: Option<i32>,
    pub uid: i32,
    pub etid: i32,
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct FieldEntry {
    pub id: Option<i32>,
    pub eeid: i32,
    pub fid: i32, 
    pub content: String,
} //created at contained in EntryEntryid,

// a logic implementation to connect actions in web app
// to entries, rules, fields, etc.
#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct Rule {
    pub id: Option<i32>,
    pub rid: Option<i32>, // record ID
    pub aid: Option<i32>, //action id
    pub name: String,
    pub priority: Option<i32>,
    pub status: i32,
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct Action {
    pub id: Option<i32>,
    pub target: String,
    pub action: String,
    pub created_at: i32,
}

// NOTE: Define as enum for in-code options TODO allow for tiered conditions
// TODO Create table for actions performed on the basis of conditions
#[derive(Default, FromRow, Serialize, Deserialize)]
pub struct Condition {
    pub id: Option<i32>,
    pub pos: i32, //position in rule
    pub and_or: bool, //whether this is preceded by "and". 0 if preceded by "or"
    pub ruleid: Option<i32>, //can be associated only with a single record (or multiple, for future)
    pub iid1: Option<i32>, //can be associated only with a single item if wished (or multiple, to impl)
    pub fid1: i32, //must be associated with a field, whether singular or global
    pub iid2: Option<i32>, 
    pub fid2: i32,
    pub cond: i32, // < <= = >= > etc
    pub status: bool,
    pub created_at: i32,
}


// a label is a Field
// a task is an item
// a goal is an item

// Use should be able to define their own loggable items
// NOTE: Propose: Optionally define as loggable?
// NOTE: Propose: Item is generic, different item types below
// NOTE: Example: Fooditem, RuleItem, 
//


pub trait Model: Sized {

    fn to_string(&self);
    fn from_id(&self);
    fn from_db(&self);
    fn to_dB(&self);
}


impl User {

    pub async fn new(email: &str, uname: &str, pwd: &str) -> Self { 
        User {
            id: None,
            email: String::from(email),
            username: String::from(uname),
            password: String::from(pwd),
            created_at: Utc::now().timestamp() as i32,
        } 
    }

    pub async fn build() -> Self {
        User {
            id: None, created_at: Utc::now().timestamp() as i32,
            ..User::default()
        }
    }

    pub async fn insert(self, db: Db) -> sqlx::Result<Self> {
        sqlx::query("INSERT INTO Users 
        (email, username, password, created_at) 
        VALUES ($1, $2, $3, $4);")  
            .bind(&self.email)
            .bind(&self.username)
            .bind(&self.password)
            .bind(Utc::now().timestamp() as i32)
            .execute(&db.pool).await;
        Ok(self)
    }

    pub async fn delete_from(self, db: Db) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM Users 
        (email, username, password, created_at) 
        VALUES ($1, $2, $3, $4);")  
            .bind(&self.email)
            .bind(&self.username)
            .bind(&self.password)
            .bind(Utc::now().timestamp() as i32)
            .execute(&db.pool).await;
        Ok(())
    }

    // pub async fn with_db(pool: &MemDb) -> {    }
    pub async fn from_id(db: Db, id: i32 )
        -> sqlx::Result<Self> 
    {
        let res: Self = sqlx::query_as::<Sqlite, Self>(
            "SELECT * FROM Users WHERE id=?;")
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    pub async fn from_user(user: User) -> Self {
        Self { id: None, ..user }
    }

    pub async fn from_username(db: Db, username: &str) 
        -> sqlx::Result<Self> 
    {
        let res: Self = sqlx::query_as::<Sqlite, Self>(
            "SELECT * FROM Users WHERE username=?;")
            .bind(String::from(username))
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    pub async fn from_email(db: Db, username: &str) 
        -> sqlx::Result<Self> 
    {
        let res: Self = sqlx::query_as::<Sqlite, Self>(
            "SELECT * FROM Users WHERE email=?;")
            .bind(String::from(username))
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    pub async fn from_db<T: Into<String>>(db: Db, param: &str, value: T)
        -> sqlx::Result<Self> 
    {
        let val: String = value.into();
        let res: Self = sqlx::query_as::<Sqlite, Self>(
            "SELECT * FROM Users WHERE ?=?;")
            .bind(String::from(param))
            .bind(String::from(val))
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    pub async fn fetch_all(db: Db)
        -> sqlx::Result<Vec<User>> 
    {
        let res: Vec<User> = sqlx::query_as::<Sqlite, Self>
            ("SELECT * FROM Users")
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn table() -> String { String::from("Users") }

    pub fn to_string(&self) -> String { String::new() }

    pub fn create_record(&self, name: &str) -> Record {
        Record {
            id: None,
            uid: self.id.unwrap(),
            name: name.to_string(),
            created_at: Utc::now().timestamp() as i32,
            status: 1, private: true
        }
    }

    pub async fn get_record(&self, db: Db) -> sqlx::Result<Record> {
        let res: Record = sqlx::query_as::<Sqlite, Record>(
            "SELECT * FROM Records WHERE uid=?;")
            .bind(self.id)
            .fetch_one(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn query<T: Model>(&self) -> &Self { self }

    pub async fn create_entry(&self, record_name: &str, entry_name: &str) -> EntryType {
        Record::from_user(self).await
            .with_name(String::from(record_name)).await
            .create_entry_type(self.id.unwrap(), entry_name.to_string()).await
    }

    pub async fn get_all_data(&self) {  }
}

impl Record {

    pub async fn from_user(user: &User) -> Record {
        let ts = Utc::now().timestamp() as i32;
        let uid = user.id.unwrap();
        Self { id: None, uid, name: String::new(), created_at: ts, private: true, status: 1 }
    }

    pub async fn with_name(mut self, name: String) -> Record {
        self.name = name; self
    }

    pub async fn create_entry_type(&self, uid: i32, name: String) -> EntryType {
        EntryType {
            id: Some(0),
            uid: uid,
            name: name,
            private: true,
            status: 1,
            created_at: Utc::now().timestamp() as i32,
        }
    }

    pub async fn add_item(self, item: Item) -> Self { self }
}

impl EntryType {
    pub async fn new() {}
    
}

impl Item {
    pub async fn new() {}
}

impl Field {
    pub async fn new() {}
}

pub enum Conditions {}
/*
impl Model for User {}
impl Model for Record {}
impl Model for Entry {}
impl Model for Item {}
impl Model for Field {}
impl Model for Condition {}
impl Model for Rule {}
*/

/*
 *
 * Vague ideas:
 * Factoid objects: can interplay with rules, conditions, etc. to provide a portrait over time
 * and provide insights into other entries
*/
// NOTE Right now, entries must contain items in order to contain their fields -- they can choose
// fields individually from items, but cannot make a ifeld not related to an item
