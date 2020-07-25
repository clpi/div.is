use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    Time, Status, Permission, Model,
    field::Field,
    link::{ItemFieldLink, RecordItemLink},
};
use std::rc::Rc;

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub pid: Option<i32>, // parent item id
    pub name: String,
    #[serde(default = "Status::active")]
    pub status: String,
    #[serde(default = "Permission::private")]
    pub private: bool,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

impl Item {

    pub fn new(uid: i32, name: String) -> Self {
        Self {
            id: None,
            uid, name,
            pid: None,
            status: Status::active(),
            private: Permission::private(),
            created_at: Time::now(),
        }
    }

    pub fn create(uid: i32, name: String) -> ItemBuilder {
        ItemBuilder::new(Self::new(uid, name)) 
    }


    pub async fn insert(self, db: Db) -> sqlx::Result<()> {
        let res = sqlx::query("
            INSERT INTO Items 
            (uid, pid, name, status, private, created_at)
            VALUES ($1, $2, $3, $4, $5, $6);")
            .bind(&self.uid)
            .bind(&self.pid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.private)
            .bind(&self.created_at)
            .execute(&db.pool).await?;
        Ok(())
    }

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

impl Model for Item {}

pub struct ItemBuilder {
    item: Rc<Item>,
    fields: Option<Vec<Rc<Item>>>,
}

impl ItemBuilder {
    pub fn new(item: Item) -> Self {
        Self {
            item: Rc::from(item),
            fields: None,
        }
    }
    
    pub fn with_field(self, field: Field) -> () {
        
    }

    //TODO implement
    pub fn build() -> Item {
        Item::default()
    }
}

// $07/25/20$  not impl in sql
//
pub enum Relationship {
    Parent(i32, i32), //not sure if i want "following?"
    Child(i32, i32),
    Synchronize(i32, i32),
    NoRelationship,
}
impl Relationship {
    pub fn none() -> String { "none".to_string() }
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RecordRelationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub iid1: i32,
    pub iid2: i32,
    #[serde(default = "Relationship::none")]
    pub rel: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
    #[serde(default = "Time::now")]
    pub updated_at: i32,
}
