use async_std::println;
use core::panic;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlit, SqlitePool};
use std::result::Result;

const db_url: String = String::from("sqlite:://sqlite.db");

pub async fn create_schema() -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitPool::connect(&db_url).await?;

    let qry = "";

    let result = sqlx::query(&qry).execute(&pool).await;
    pool.close().await;
    return result;
}

async fn create_db() {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();

        match create_schema(&db_url).await {
            Ok(_) => println!("Database created successfully!"),
            Err(e) => panic!("{}", e),
        };
    };
}

pub async fn create_entry() {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        pub async fn retrieve_all_entries(
            String: &db_url,
        ) -> Result<SqliteQueryResult, sqlx::Error> {
            return;
        }

        let qry = "SELECT * FROM data";

        let instances = SqlitePool::connect(&db_url).await.unwrap();

        let result = sqlx::query(&qry).bind("data").execute(&intances).await;
        instances.close().await;

        result;
    }
}
