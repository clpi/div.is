pub mod models;
pub mod users;
pub mod records;


pub use models::*;
pub use users::*;


use sqlx::SqlitePool;
use sqlx::*;

#[derive(Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {

    pub async fn new(url: &str) -> sqlx::Result<Self> {
        if !std::path::Path::new(&url.to_string()).exists() {
            // run sqlite3 mem.db "" to create
        }
        let pool = sqlx::SqlitePool::new(&url).await?;
        sqlx::query_file!("schema/schema.sql")
            .execute(&pool).await?;
        println!("Successfully created DB pool.");
        Ok( Self { pool } )
    }

    pub async fn init(self) -> sqlx::Result<Self> {
        sqlx::query_file!("schema/schema.sql")
            .execute(&self.pool).await?;
        Ok( self )
   }
}
