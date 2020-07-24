use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    Time, Status, Permission, Model,
    link::FieldEntryLink,
};

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct EntryType {
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

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct EntryEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub rid: i32,
    pub etid: i32,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

impl EntryType {
    pub async fn new() {}
    
    pub async fn with_field(&self, db: &Db, field_id: i32) -> sqlx::Result<()> {
        FieldEntryLink::create(&db, field_id, self.id.unwrap()).await?;
        Ok(())
    }
}

impl EntryEntry {

}
