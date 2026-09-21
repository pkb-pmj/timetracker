pub mod activities;

use std::{
    path::Path,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Serialize, Serializer};
use sqlx::{
    migrate,
    migrate::{MigrateDatabase, Migrator},
    query, Sqlite, SqlitePool,
};
use uuid::{ContextV7, Timestamp, Uuid};

static MIGRATOR: Migrator = migrate!("./migrations");

pub struct Database {
    pool: SqlitePool,
    uuid_generator: UuidGenerator,
}

pub struct UuidGenerator(Mutex<ContextV7>);

impl UuidGenerator {
    pub fn new() -> Self {
        Self(Mutex::new(ContextV7::new()))
    }

    pub fn generate(&self) -> Uuid {
        Uuid::new_v7(Timestamp::now(&self.0))
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .cast_signed()
}

impl Database {
    pub async fn new(database_path: impl AsRef<Path>) -> Result<Self, sqlx::Error> {
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

        let pool = SqlitePool::connect(&database_url).await?;

        // TODO: only reset database if migrations fail?
        MIGRATOR.undo(&pool, 0).await?;
        MIGRATOR.run(&pool).await?;

        let uuid_generator = UuidGenerator::new();

        Ok(Self {
            pool,
            uuid_generator,
        })
    }

    fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    fn new_uuid(&self) -> Uuid {
        self.uuid_generator.generate()
    }

    pub async fn test(&self) -> Result<i64, sqlx::Error> {
        Ok(query!("SELECT (1) as id").fetch_one(&self.pool).await?.id)
    }

    pub async fn create_event(&self, name: &str) -> Result<Uuid, sqlx::Error> {
        Ok(query!(
            r#"
                INSERT INTO events (id, time, name)
                VALUES (?, ?, ?)
                RETURNING id AS "id: Uuid"
            "#,
            self.new_uuid(),
            now(),
            name,
        )
        .fetch_one(&self.pool)
        .await?
        .id)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("{0}")]
    Sqlx(#[from] sqlx::Error),
}

impl Serialize for DatabaseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;
