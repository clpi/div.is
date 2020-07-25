use sqlx::{sqlite::*, Sqlite, FromRow};use crate::api::auth::hash_pwd;
use crate::db::Db;
use super::{
    Model, Time, Status, Permission,
    record::Record,
    entry::EntryType,
    link::{UserGroupLink, UserRecordLink},
};

// TODO make password Option<String> so no need for UserSession
// or passwordless user structs
#[derive(Default, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
}


pub enum UserType {
    Regular,
    Administrator,
}

impl UserType {
    pub fn regular() -> String { String::from("regular") }
}

impl User {

    pub async fn new(email: String, uname: String, pwd: String) -> Self { 
        User {
            id: None,
            email: String::from(email),
            username: String::from(uname),
            password: String::from(pwd),
            created_at: Time::now(),
        } 
    }

    pub async fn hash_pass(mut self, key: String) -> Result<Self, std::io::Error> {
        self.password = hash_pwd(&key, &self.password.clone()).await;
        Ok(self)
    }

    //pub async fn from_db(db: Db) -> DbQuery { User::default(), DbQuery::from(db) }

    pub async fn build() -> Self {
        User {
            id: None, created_at: Time::now(),
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
            .bind(Time::now())
            .execute(&db.pool).await?;
        match &self.id.clone() {
            Some(id) => UserInfo::insert_new(db, id.to_owned()).await?,
            None => {
                let new_id = User::from_username
                    (&db, self.username.clone())
                    .await?.id.unwrap();
                UserInfo::insert_new(db, new_id).await?;
            }
        }
        Ok(self)
    }

    pub async fn with_user_info(self, info: UserInfo) -> () {  }

    pub async fn delete_by_id(db: &Db, id:i32) -> sqlx::Result<u64> {
        let res = sqlx::query("DELETE FROM Users where id=?")
            .bind(id)
            .execute(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_info(self, db: &Db) -> sqlx::Result<UserInfo> {
        let info = sqlx::query_as::<Sqlite, UserInfo>("
            SELECT * FROM UserInfo INNER JOIN Users on UserInfo.uid
            =Users.id WHERE UserInfo.uid=?
            ")
                .bind(self.id)
                .fetch_one(&db.pool).await?;
        Ok(info)
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
            status: Status::active(),
            permission: Permission::private(),
            created_at: Time::now(),
        }
    }

    pub async fn with_record(&self, db: &Db, record_id: i32) 
        -> sqlx::Result<()> {
        UserRecordLink::create(&db, self.id.unwrap(), record_id).await?;
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
// TODO actually implement -- without db query
//impl From<UserSession> for User {
    //fn from(user: UserSession) -> User {}
//}


impl Into<String> for User {
    fn into(self) -> String { String::from("User") }
}

impl Model for User {}

// NOTE: Add (?) preferences
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub experience: i32,
    #[serde(default = "UserType::regular")]
    pub user_type: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
    #[serde(default = "Time::now")]
    pub updated_at: i32,
}

// TODO Implement
impl UserInfo {
    pub async fn insert_new(db: Db, uid: i32) -> sqlx::Result<()> {
    let res = sqlx::query("INSERT INTO UserInfo (uid, created_at, updated_at) VALUES ($1);")
        .bind(uid)
        .bind(Time::now())
        .bind(Time::now())
        .execute(&db.pool).await?;
    Ok(())
    }
}

pub struct UserInfoBuilder {


}

pub struct UserPrefs {

}
