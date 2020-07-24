use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    now_ts, Model,
    entry::EntryType,
    user::User, item::Item,
    link::{UserRecordLink, RecordItemLink},
};

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: i32,
    pub private: bool,
    #[serde(default = "now_ts")]
    pub created_at: i32,
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

    pub async fn from_id(db: &Db, id: i32) -> sqlx::Result<Self> {
        let record = sqlx::query_as::<Sqlite, Self>("SELECT * FROM Records WHERE id=?;")  
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok(record)
    }

    pub async fn insert(self, db: &Db) 
    -> sqlx::Result<Self> {
        sqlx::query("INSERT INTO Records 
        (uid, name, status, private, created_at) VALUES ($1, $2, $3, $4, $5);")  
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.private)
            .bind(now_ts())
            .execute(&db.pool).await?;
        UserRecordLink::create(db, self.uid, self.id.unwrap()).await?;
        Ok(self)
    }

    pub async fn get_items(self, db: &Db) -> sqlx::Result<Vec<Item>> {
        let items: Vec<Item> = sqlx::query_as::<Sqlite, Item>("
            SELECT * FROM Items INNER JOIN RecordItemLinks 
            ON RecordItemLinks.iid=Items.id WHERE RecordItemLinks.iid=?;")
            .bind(&self.id)
            .fetch_all(&db.pool).await?;
        Ok(items)
    }

    pub async fn add_user(self, db: &Db, uid: i32) -> sqlx::Result<Self> {
        UserRecordLink::create(db, uid, self.id.unwrap()).await?;
        Ok(self)
    }

    pub async fn get_users(self, db: &Db) -> sqlx::Result<Vec<User>> {
        let users: Vec<User> = sqlx::query_as::<Sqlite, User>("
            SELECT * FROM Users INNER JOIN UserRecordLinks
            ON UserRecordLinks.uid=Users.id WHERE UserRecordLinks.rid=?;")
            .bind(&self.id)
            .fetch_all(&db.pool).await?;
        Ok(users)
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

impl Model for Record {}
