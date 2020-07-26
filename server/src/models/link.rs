use sqlx::{Postgres, FromRow, postgres::*};
use crate::db::Db;
use super::*;
use super::{
    Time, Status, Priority, Permission, Model,
};

pub enum Links {
    RecordItem,
    ItemField,
    UserGroup,
    FieldEntry,
    UserRecord,
}

pub trait ModelLink: Sized { }

// TODO modularize implementation with traits so redundant logic is reduced

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RecordItemLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub rid: i32,
    pub iid: i32,
    #[serde(default = "Status::active")]
    pub status: String,
    #[serde(default = "Priority::lowest")]
    pub priority: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ItemFieldLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub iid: i32,
    pub fid: i32,
    #[serde(default = "Status::active")]
    pub status: String,
    #[serde(default = "Priority::lowest")]
    pub priority: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct FieldEntryLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub fid: i32,
    pub etid: i32,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

// TODO enumerate roles add default
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserGroupLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub gid: i32,
    pub role: String,
    #[serde(default = "Status::active")]
    pub status: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

// TODO enumerate priveleges? or make role merge with usertype?
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UserRecordLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub rid: i32,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

impl RecordItemLink {
    pub async fn create(
        db: &Db,
        record_id: i32,
        item_id: i32,
        priority: String,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO RecordItemLinks
        (rid, iid, priority, created_at) 
        VALUES ($1, $2, $3, $4);")
            .bind(record_id)
            .bind(item_id)
            .bind(priority)
            .bind(Time::now())
            .execute(&db.pool).await?;
        Ok(())
    }

    pub async fn get_members(self, db: &Db) -> sqlx::Result<(Record, Item)> {
        let rec = Record::from_id(db, self.rid).await?;
        let itm = Item::from_id(db, self.rid).await?;
        Ok((rec, itm))
    }
}

impl ItemFieldLink {
    pub async fn create(
        db: &Db,
        item_id: i32,
        field_id: i32,
        priority: String,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO ItemFieldLinks
        (iid, fid, priority, created_at) 
        VALUES ($1, $2, $3, $4);")
            .bind(item_id)
            .bind(field_id)
            .bind(priority)
            .bind(Time::now())
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
            .bind(Time::now())
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
            .bind(Time::now())
            .execute(&db.pool).await?;
        Ok(())
    }
}

impl UserRecordLink {
    pub async fn create(
        db: &Db,
        user_id: i32,
        record_id: i32,
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO UserRecordLinks
            (uid, rid, created_at) VALUES ($1, $2, $3);")
            .bind(user_id)
            .bind(record_id)
            .bind(Time::now())
            .execute(&db.pool).await?;
        Ok(())
    }
}
