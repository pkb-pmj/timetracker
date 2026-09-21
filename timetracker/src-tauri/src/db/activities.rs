use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tauri::State;
use uuid::Uuid;

use crate::db::{Database, DatabaseResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub start: i64,
    pub end: Option<i64>,
    pub name: String,
}

#[tauri::command]
pub async fn start_activity(
    db: State<'_, Database>,
    start: i64,
    name: &str,
) -> DatabaseResult<Uuid> {
    Ok(query!(
        r#"
            INSERT INTO activities (id, start, end, name)
            VALUES (?, ?, NULL, ?)
            RETURNING id AS "id: Uuid"
        "#,
        db.new_uuid(),
        start,
        name,
    )
    .fetch_one(db.pool())
    .await?
    .id)
}

#[tauri::command]
pub async fn update_activity_start(
    db: State<'_, Database>,
    id: Uuid,
    start: i64,
) -> DatabaseResult<()> {
    query!(
        r#"
            UPDATE activities
            SET start = ?
            WHERE id = ?
            RETURNING id
        "#,
        start,
        id,
    )
    .fetch_one(db.pool())
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn update_activity_end(
    db: State<'_, Database>,
    id: Uuid,
    end: Option<i64>,
) -> DatabaseResult<()> {
    query!(
        r#"
            UPDATE activities
            SET end = ?
            WHERE id = ?
            RETURNING id
        "#,
        end,
        id,
    )
    .fetch_one(db.pool())
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn update_activity_name(
    db: State<'_, Database>,
    id: Uuid,
    name: &str,
) -> DatabaseResult<()> {
    query!(
        r#"
            UPDATE activities
            SET name = ?
            WHERE id = ?
            RETURNING id
        "#,
        name,
        id,
    )
    .fetch_one(db.pool())
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn get_activity(db: State<'_, Database>, id: Uuid) -> DatabaseResult<Activity> {
    Ok(query_as!(
        Activity,
        r#"
            SELECT id AS "id: Uuid", start, end, name
            FROM activities
            WHERE id = ?
        "#,
        id
    )
    .fetch_one(db.pool())
    .await?)
}

#[tauri::command]
pub async fn get_all_activities(db: State<'_, Database>) -> DatabaseResult<Vec<Activity>> {
    Ok(query_as!(
        Activity,
        r#"
            SELECT id AS "id: Uuid", start, end, name
            FROM activities
        "#,
    )
    .fetch_all(db.pool())
    .await?)
}
