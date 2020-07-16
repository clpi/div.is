use serde_derive::*;
use sqlx::FromRow;
use super::Db;
use sqlx::{sqlite::*, Sqlite, types::chrono::{DateTime, Utc}};
use crate::api::auth::hash_pwd;

//TODO Consider adding custom types for forieng key references, using
//TODO Severely refactor code to reduce redundancy (query builder?)
//TODO Consider removing redundancy between EntryType/Item
//TODO Separate implementations into own files
//TODO error handling for with_x functions for models w/o ids -- remove unwrap()
//sqlx::Type and transparent
//NOTE: Add (?) last login, pwd hash
#[derive(Default, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

// NOTE: Add (?) preferences
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub img_path: String,
    pub gender: String,
    pub birth_date: i32,
    pub location: String,
    pub experience: i32,
    pub privelge_level: i32,
    pub created_at: i32,
    #[serde(default = "now_ts")]
    pub updated_at: i32,
}
// TODO: Eventually make records collaborable, not associated with single user
// TODO: make collaborative records have anonymous option
//#[derive(Serialize, Deserialize)]
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32, //user id
    pub name: String,
    pub status: i32,
    pub private: bool,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub pid: Option<i32>, // parent item id
    pub name: String,
    pub status: i32,
    pub private: bool,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

// NOTE: typ can mean cardinally valued (ie checkboxes), stateful, textbox, etc.
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Field {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub typ: String,
    pub value: String,
    pub private: bool, 
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RecordItemLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub rid: i32,
    pub iid: i32,
    pub status: i32,
    pub priority: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ItemFieldLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub iid: i32,
    pub fid: i32,
    pub status: i32,
    pub priority: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct FieldEntryLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub fid: i32,
    pub etid: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct EntryType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: i32,
    pub private: bool,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct EntryEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub rid: i32,
    pub etid: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct FieldEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub eeid: i32,
    pub fid: i32, 
    pub content: String,
} //created at contained in EntryEntryid,

// a logic implementation to connect actions in web app
// to entries, rules, fields, etc.
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Rule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub rid: Option<i32>, // record ID
    pub aid: Option<i32>, //action id
    pub name: String,
    pub priority: Option<i32>,
    pub status: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub target: String,
    pub action: String,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

// NOTE: Define as enum for in-code options TODO allow for tiered conditions
// TODO Create table for actions performed on the basis of conditions
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Condition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub pos: i32, //position in rule
    pub and_or: bool, //whether this is preceded by "and". 0 if preceded by "or"
    pub ruleid: Option<i32>, //can be associated only with a single record
    pub iid1: Option<i32>, //can be associated only with a single item if wished
    pub fid1: i32, //must be associated with a field, whether singular or global
    pub iid2: Option<i32>, 
    pub fid2: i32,
    pub cond: i32, // < <= = >= > etc
    pub status: bool,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub private: bool, 
    pub status: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserGroupLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub gid: i32,
    pub role: String,
    pub status: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserRecordLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub rid: i32,
    pub privelege: i32,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

// a label is a Field
// a task is an item
// a goal is an item

pub trait Model: Sized {  }
pub trait ModelLink { } 

impl User {

    pub async fn new(email: String, uname: String, pwd: String) -> Self { 
        User {
            id: None,
            email: String::from(email),
            username: String::from(uname),
            password: String::from(pwd),
            created_at: now_ts(),
        } 
    }

    pub async fn hash_pass(mut self, key: String) -> Result<Self, std::io::Error> {
        self.password = hash_pwd(&key, &self.password.clone()).await;
        Ok(self)
    }

    //pub async fn from_db(db: Db) -> DbQuery { User::default(), DbQuery::from(db) }

    pub async fn build() -> Self {
        User {
            id: None, created_at: now_ts(),
            ..User::default()
        }
    }

    pub async fn insert_into(self, db: Db) -> sqlx::Result<Self> {
        sqlx::query("INSERT INTO Users 
            (email, username, password, created_at) 
            VALUES ($1, $2, $3, $4);")  
            .bind(&self.email)
            .bind(&self.username)
            .bind(&self.password)
            .bind(now_ts())
            .execute(&db.pool).await?;
        Ok(self)
    }

    pub async fn with_user_info(self, info: UserInfo) -> () {  }

    pub async fn delete_by_id(db: &Db, id:i32) -> sqlx::Result<u64> {
        sqlx::query("DELETE FROM Users where id=?")
            .bind(id)
            .execute(&db.pool).await
    }

    pub async fn delete_by_username(db: &Db, username: String) 
    -> sqlx::Result<u64> {
        sqlx::query("DELETE FROM Users where username=?")
            .bind(username)
            .execute(&db.pool).await
    }

    pub async fn from_id(db: &Db, id: i32 )
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

    pub async fn from_username(db: &Db, username: String) 
        -> sqlx::Result<Self>
    {
        sqlx::query_as::<Sqlite, Self>
            ("SELECT * FROM Users WHERE username=?;")
            .bind(String::from(username))
            .fetch_one(&db.pool).await
    }

    pub async fn from_email(db: &Db, username: &str) 
        -> sqlx::Result<Self> 
    {
        sqlx::query_as::<Sqlite, Self>
            ("SELECT * FROM Users WHERE email=?;")
            .bind(String::from(username))
            .fetch_one(&db.pool).await
    }

    pub async fn from_db<T: Into<String>>(db: &Db, param: &str, value: T)
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

    pub async fn fetch_all(db: &Db)
        -> sqlx::Result<Vec<User>> 
    {
        let res: Vec<User> = sqlx::query_as::<Sqlite, Self>
            ("SELECT * FROM Users;")
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn create_record(&self, name: &str) -> Record {
        Record {
            id: None,
            uid: self.id.unwrap(),
            name: name.to_string(),
            created_at: now_ts(),
            status: 1, private: true
        }
    }

    pub async fn with_record(&self, db: &Db, record_id: i32, privelege: i32) 
        -> sqlx::Result<()> {
        UserRecordLink::create(&db, self.id.unwrap(), record_id, privelege).await?;
        Ok(())
    }

    pub async fn get_records(&self, db: &Db) -> sqlx::Result<Vec<Record>> {
        let res: Vec<Record> = sqlx::query_as::<Sqlite, Record>(
            "SELECT * FROM Records WHERE uid=?;")
            .bind(self.id)
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    //pub async fn query<T: Model>(&self) -> &Self { self }

    pub async fn create_entry(&self, record_name: String, entry_name: &str) -> EntryType {
        Record::new(self.id.unwrap(), record_name)
            .create_entry_type(self.id.unwrap(), entry_name.to_string()).await
    }

    pub async fn get_all_data(&self) {  }
}

// NOTE: pass user ID into all user header requests, so it can be passed ot Record::new()
impl Record {

    pub fn new(uid: i32, name: String) -> Record {
        Self { 
            id: None, uid, name, 
            status: 1,
            private: true,
            created_at: now_ts(),
        }
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO Records 
        (uid, name, status, private, created_at) 
        VALUES ($1, $2, $3, $4, $5);")  
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.private)
            .bind(&self.created_at)
            .execute(&db.pool).await?;
        Ok(())
    }

    pub async fn get_items(self, db: &Db) -> sqlx::Result<Vec<Item>> {
        let items: Vec<Item> = sqlx::query_as::<Sqlite, Item>("
            SELECT * FROM Items INNER JOIN RecordItemLinks 
            ON RecordItemLinks.iid=Items.id WHERE RecordItemLinks.iid=?;")
            .bind(&self.id)
            .fetch_all(&db.pool).await?;
        Ok(items)
    }

    pub async fn with_name(mut self, name: String) -> Record {
        self.name = name; self
    }

    pub async fn create_entry_type(&self, uid: i32, name: String) -> EntryType {
        EntryType {
            id: None,
            uid: uid,
            name: name,
            private: true,
            status: 1,
            created_at: now_ts(),
        }
   }

    pub async fn with_item(&self, 
        db: &Db, 
        item_id: i32, 
        priority: Option<i32>
    ) -> sqlx::Result<()> {
        RecordItemLink::create(&db, self.id.unwrap(), item_id, priority).await?;
        Ok(())
    }

}

//TODO allow for requests like User::with_id(id).delete() or User::with_username(

impl Item {
    pub async fn new() {}

    pub async fn with_field(&self, 
        db: &Db, 
        field_id: i32, 
        priority: Option<i32>
    ) -> sqlx::Result<()> {
        ItemFieldLink::create(&db, self.id.unwrap(), field_id, priority).await?;
        Ok(())
    }

    pub async fn get_fields(self, db: &Db) -> sqlx::Result<Vec<Field>> {
        let items: Vec<Field> = sqlx::query_as::<Sqlite, Field>("
            SELECT * FROM Fields INNER JOIN ItemFieldLinks 
            ON ItemFieldLinks.fid=Fields.id WHERE ItemFieldLinks.fid=?")
            .bind(&self.id)
            .fetch_all(&db.pool).await?;
        Ok(items)
    }
}

impl RecordItemLink {
    pub async fn create(
        db: &Db,
        record_id: i32,
        item_id: i32,
        priority: Option<i32>
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO RecordItemLinks
        (rid, iid, priority, created_at) 
        VALUES ($1, $2, $3, $4);")
            .bind(record_id)
            .bind(item_id)
            .bind(priority)
            .bind(now_ts())
            .execute(&db.pool).await?;
        Ok(())
    }
}

impl ItemFieldLink {
    pub async fn create(
        db: &Db,
        item_id: i32,
        field_id: i32,
        priority: Option<i32>
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO ItemFieldLinks
        (iid, fid, priority, created_at) 
        VALUES ($1, $2, $3, $4);")
            .bind(item_id)
            .bind(field_id)
            .bind(priority)
            .bind(now_ts())
            .execute(&db.pool).await?;
        Ok(())
    }
}

impl FieldEntryLink {
    pub async fn create(
        db: &Db,
        field_id: i32,
        entry_type_id: i32,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO FieldEntryLinks
        (fid, etid, created_at) VALUES ($1, $2, $3);")
            .bind(field_id)
            .bind(entry_type_id)
            .bind(now_ts())
            .execute(&db.pool).await?;
        Ok(())
    }
}

impl UserGroupLink {
    pub async fn create(
        db: &Db,
        user_id: i32,
        group_id: i32,
        role: String,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO UserGroupLinks
        (uid, gid, role, created_at) VALUES ($1, $2, $3, $4);")
            .bind(user_id)
            .bind(group_id)
            .bind(role)
            .bind(now_ts())
            .execute(&db.pool).await?;
        Ok(())
    }
}

impl UserRecordLink {
    pub async fn create(
        db: &Db,
        user_id: i32,
        record_id: i32,
        privelege: i32,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO UserRecordLinks
            (uid, rid, privelege, created_at) VALUES ($1, $2, $3, $4);")
            .bind(user_id)
            .bind(record_id)
            .bind(privelege)
            .bind(now_ts())
            .execute(&db.pool).await?;
        Ok(())
    }
}

impl EntryType {
    pub async fn new() {}
    
    pub async fn with_field(&self, db: &Db, field_id: i32) -> sqlx::Result<()> {
        FieldEntryLink::create(&db, field_id, self.id.unwrap()).await?;
        Ok(())
    }
}


impl Field {
    pub async fn new() {}
}

impl Group {
    pub async fn new() {}

    pub async fn with_user(&self, db: &Db, user_id: i32, role: String) 
        -> sqlx::Result<()> {
        UserGroupLink::create(&db, user_id, self.id.unwrap(), role).await?;
        Ok(())
    }
}




impl Into<String> for User {
    fn into(self) -> String { String::from("User") }
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

pub enum Conditions {}
impl Model for User {}
impl Model for Record {}
impl Model for EntryType {}
impl Model for Item {}
impl Model for Field {}
impl Model for Condition {}
impl Model for Rule {}
impl Model for Group {} 

impl ModelLink for RecordItemLink { }
impl ModelLink for ItemFieldLink { }
impl ModelLink for FieldEntryLink { }
impl ModelLink for UserGroupLink { }

pub enum FieldTypes {
    Text,
    TextBox,
    EnumSingle,
    EnumMultiple,
    Boolean,
    Range,
}

pub enum GroupRole {

}

pub enum RecordPrivelege {
    
}

pub fn now_ts() -> i32 {
    Utc::now().timestamp() as i32
}
/*
 *
 * Vague ideas:
 * Factoid objects: can interplay with rules, conditions, etc. to provide a portrait over time
 * and provide insights into other entries
*/
// NOTE Right now, entries must contain items in order to contain their fields -- they can choose
// fields individually from items, but cannot make a ifeld not related to an item
