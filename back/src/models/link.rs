use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    now_ts,
};


// TODO modularize implementation with traits so redundant logic is reduced

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
    ) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO UserRecordLinks
            (uid, rid, created_at) VALUES ($1, $2, $3);")
            .bind(user_id)
            .bind(record_id)
            .bind(now_ts())
            .execute(&db.pool).await?;
        Ok(())
    }
}
