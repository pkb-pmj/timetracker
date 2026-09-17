use std::{path::Path, sync::Mutex};

use sqlx::{
    migrate,
    migrate::{MigrateDatabase, Migrator},
    query, Sqlite, SqlitePool,
};
use uuid::{ContextV7, Timestamp, Uuid};

static MIGRATOR: Migrator = migrate!("./migrations");

pub struct UuidGenerator(Mutex<ContextV7>);

impl UuidGenerator {
    pub fn new() -> Self {
        Self(Mutex::new(ContextV7::new().with_additional_precision()))
    }

    pub fn generate(&self) -> Uuid {
        Uuid::new_v7(Timestamp::now(&self.0))
    }
}

pub async fn get_pool(database_path: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
    std::fs::create_dir_all(database_path.as_ref().parent().expect("invalid path"))
        .expect("failed to create database directory");

    let database_url = format!("sqlite://{}", database_path.as_ref().display());

    println!("{}", database_url);

    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        Sqlite::create_database(&database_url).await?;
    }

    let db = SqlitePool::connect(&database_url).await?;

    test(&db).await?;

    // TODO: only reset database if migrations fail?
    MIGRATOR.undo(&db, 0).await?;
    MIGRATOR.run(&db).await?;

    Ok(db)
}

pub async fn test(db: &SqlitePool) -> Result<i64, sqlx::Error> {
    Ok(query!("SELECT (1) as id").fetch_one(db).await?.id)
}
