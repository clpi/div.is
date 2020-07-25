pub mod pg;
pub mod redis;

use sqlx::SqlitePool;
use sqlx::*;

#[derive(Clone)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {

    pub async fn new(url: &str) -> sqlx::Result<Self> {
        if !std::path::Path::new(&url.to_string()).exists() {
            // run sqlite3 mem.db "" to create
        }
        let pool = sqlx::SqlitePool::new(&url).await?;
        sqlx::query_file!("sql/schema.sql")
            .execute(&pool).await?;
        println!("Successfully created DB pool.");
        Ok( Self { pool } )
    }

    pub async fn init(self) -> sqlx::Result<Self> {
        sqlx::query_file!("sql/schema.sql")
            .execute(&self.pool).await?;
        Ok( self )
   }

    pub async fn clear(self) -> sqlx::Result<()> {
        sqlx::query(";").execute(&self.pool).await?;
        Ok(())
    }

    //pub async fn clear(self) -> sqlx::Result<()> {
        //sqlx::query_file!("sql/clear.sql")
            //.execute(&self.pool).await?;
        //Ok( () )
    //}

    pub async fn clear_table(self, table: &str) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM $1;")
            .bind(&table)
            .execute(&self.pool).await?;
        Ok(())
    }
}
