pub mod models;

pub use models::*;
use sqlx::SqlitePool;
use sqlx::*;

pub struct Db {
    pool: SqlitePool,
}

impl Db {

    pub async fn new(url: &str) -> sqlx::Result<Self> {
        Ok( Self { 
            pool: sqlx::SqlitePool::new(&url).await?,
        } )
    }

    pub async fn init(self) -> sqlx::Result<Self> {
        sqlx::query_file!("schema/schema.sql")
            .execute(&self.pool).await?;
        Ok( self )
    }
}
