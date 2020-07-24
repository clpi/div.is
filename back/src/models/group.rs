use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    now_ts, Model,
    link::UserGroupLink,
};

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
