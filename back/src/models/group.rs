use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    Time, Permission, Status, Model,
    link::UserGroupLink,
};

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    #[serde(default = "Permission::private")]
    pub private: bool, 
    #[serde(default = "Status::active")]
    pub status: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}

impl Group {
    pub async fn new() {}

    pub async fn with_user(&self, db: &Db, user_id: i32, role: String) 
        -> sqlx::Result<()> {
        UserGroupLink::create(&db, user_id, self.id.unwrap(), role).await?;
        Ok(())
    }
}

pub enum GroupRole {

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
pub struct GroupRelationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub gid1: i32,
    pub gid2: i32,
    #[serde(default = "Relationship::none")]
    pub rel: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
    #[serde(default = "Time::now")]
    pub updated_at: i32,
}
