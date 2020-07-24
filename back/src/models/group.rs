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
