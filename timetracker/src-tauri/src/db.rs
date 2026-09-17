use std::path::Path;

use sqlx::{migrate::MigrateDatabase, query, Sqlite, SqlitePool};

pub async fn get_pool(database_path: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
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

    Ok(db)
}

pub async fn test(db: &SqlitePool) -> Result<i64, sqlx::Error> {
    Ok(query!("SELECT (1) as id").fetch_one(db).await?.id)
}
