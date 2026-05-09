use axum::{Json, debug_handler, extract::State};
use sqlx::{SqlitePool};

use crate::models::{CreateTaskInput, Task};

#[debug_handler]
pub async fn get_all_tasks(
    State(pool): State<SqlitePool>
) -> Json<Vec<Task>> {
    
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, done, created_at
        FROM tasks
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(tasks)
}

pub async fn create_task(
    State(pool): State<SqlitePool>,
    Json(user_input): Json<CreateTaskInput>,
) -> String {

    sqlx::query(
        r#"
        INSERT INTO tasks (title, done, created_at)
        VALUES (?, ?, datetime('now'))
        "#
    )
    .bind(user_input.title)
    .bind(false)
    .execute(&pool)
    .await
    .unwrap();

    "Task created".to_string()
}