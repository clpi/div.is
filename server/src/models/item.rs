use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use std::collections::HashMap;
use super::{
    Time, Status, Permission, Model, Priority,
    field::Field, record::Record,
    link::{ItemFieldLink, RecordItemLink},
};
use std::rc::Rc;

#[derive(Default, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
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
            status: Status::active(),
            private: Permission::private(),
            created_at: Time::now(),
        }
    }

    pub fn build(uid: i32, name: String) -> ItemBuilder {
        ItemBuilder::new(Self::new(uid, name)) 
    }

    pub async fn from_id(db: &Db, iid: i32) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Sqlite, Item>("
            SELECT * FROM Items where id = ?;")
            .bind(iid)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }


    pub async fn insert(self, db: &Db) -> sqlx::Result<Self> {
        let res = sqlx::query("
            INSERT INTO Items 
            (uid, name, status, private, created_at)
            VALUES ($1, $2, $3, $4, $5);")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.private)
            .bind(&self.created_at)
            .execute(&db.pool).await?;
        Ok(self)
    }

    pub async fn delete(db: &Db, id: i32) -> sqlx::Result<()> {
        let res = sqlx::query("
            DROP FROM Items WHERE id=?;")
            .bind(id)
            .execute(&db.pool).await?;
        Ok(())
    }

    pub async fn by_user(db: &Db, uid: i32) -> sqlx::Result<Vec<Item>> {
        let res = sqlx::query_as::<Sqlite, Item>("
            SELECT FROM Items WHERE uid=?;")
            .bind(uid)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn by_user_and_name(db: &Db, uid: i32, name: String) -> sqlx::Result<Vec<Item>> {
        let res = sqlx::query_as::<Sqlite, Item>("
            SELECT FROM Items WHERE uid=$1 AND name=$2;")
            .bind(uid)
            .bind(name)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn by_name(db: &Db, name: &str) -> sqlx::Result<Vec<Item>> {
        let res = sqlx::query_as::<Sqlite, Item>("
            SELECT FROM Items WHERE name=?;")
            .bind(name)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    /// Method to create new (field-less) item and attach it
    /// to a number of records given their IDs
    /// should be in items?
    pub async fn insert_with_records(
        db: &Db,
        uid: i32,
        name: String,
        recs: Option<HashMap<i32, String>>, //Priority::?
    ) -> sqlx::Result<Item> {
        let item = Item::new(uid, name)
            .insert(&db).await?;
        match recs {
            Some(recs) => {
                for recid in recs.keys() {
                    let rec = Record::from_id(&db, recid.to_owned()).await?;
                    let pri = recs.get(recid).unwrap();
                    rec.create_link_to_item(
                        db, item.id.unwrap(), pri.to_owned()).await?;
                }
            },
            None => {}
        }
        Ok(item)
    }

    pub async fn with_field(self, 
        db: &Db, 
        field_id: i32, 
        priority: Option<String>
    ) -> sqlx::Result<Self> {
        match priority {
            Some(pri) => {
                ItemFieldLink::create(
                    &db, self.id.unwrap(), 
                    field_id, pri)
                    .await?;
            },
            None => {
                ItemFieldLink::create(
                    &db, self.id.unwrap(), 
                    field_id, Priority::lowest())
                    .await?;
            }}
        Ok(self)
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
pub struct ItemItemRelationship {
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
