use sqlx::postgres::*;
use std::{path::Path, fs};

#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

//TODO figure out what exactly postgres wants a schema file to look like --
//     or rather, the proper way to be handling this

impl Db {

    pub async fn new(url: &str) -> sqlx::Result<Self> {
        let pool = sqlx::PgPool::new(&url).await?;
        println!("Successfully created DB pool.");
        Ok( Self { pool } )
    }

    /// "Can't insert multiple commands into prepared statement" for
    /// postgres but not for sqlite? Weird, but OK.
    //pub async fn init_schema(self) -> sqlx::Result<Self> {
        //for file in fs::read_dir(Path::new("../../sql/tables/"))? {
            //let file = file?.path().file_name();
            //sqlx::query_file!().execute(&self.pool).await?
        //}
        //Ok(self)
    //}

    pub async fn init_users(self) -> sqlx::Result<()> {
        sqlx::query("
            CREATE TABLE IF NOT EXISTS Users (
                id SERIAL NOT NULL PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL UNIQUE CHECK (char_length(username) < 40),
                password TEXT NOT NULL CHECK (char_length(password) < 40),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);") 
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn clear(self) -> sqlx::Result<()> {
        //sqlx::query_file!("sql/clear.sql")
            //.execute(&self.pool).await?;
        sqlx::query("DELETE FROM Users;")
            .execute(&self.pool).await?;
        Ok( () )
    }

    pub async fn clear_schema(self, schema: &str) -> sqlx::Result<()> {
        sqlx::query("DROP SCHEMA ? CASCADE;")
            .bind(schema)
            .execute(&self.pool).await?;
        Ok( () )
    }

    pub async fn create_schema(self, schema: &str) -> sqlx::Result<()> {
        sqlx::query("CREATE SCHEMA ?;")
            .bind(schema)
            .execute(&self.pool).await?;
        Ok( () )
    }

    pub async fn clear_table(self, table: &str) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM $1;")
            .bind(&table)
            .execute(&self.pool).await?;
        Ok(())
    }
}
