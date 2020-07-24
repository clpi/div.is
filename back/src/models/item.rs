use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    now_ts, Model,
    field::Field,
    link::{ItemFieldLink, RecordItemLink},
};

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub pid: Option<i32>, // parent item id
    pub name: String,
    pub status: i32,
    pub private: bool,
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

impl Item {
    pub async fn new() {}

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
