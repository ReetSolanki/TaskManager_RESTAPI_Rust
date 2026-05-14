use axum::{Json, debug_handler, extract::State};
use axum::extract::{Path};
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

pub async fn get_one_task_byid(
    State(pool): State<SqlitePool>,
    Path(task_id): Path<i64>
) -> Json<Task> {

    let task = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, done, created_at
        FROM tasks
        WHERE id = ?
        "#
    )
    .bind(task_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    return Json(task);
}

pub async fn delete_task_byid(
    State(pool): State<SqlitePool>,
    Path(task_id): Path<i64>
) -> String {

    sqlx::query(
        r#"
        DELETE FROM tasks
        WHERE id = ?
        "#
    )
    .bind(task_id)
    .execute(&pool)
    .await
    .unwrap()
    ;


    return "Task deleted successfully".to_string();
}

pub async fn update_details_byid(
    State(pool): State<SqlitePool>,
    Path(task_id): Path<i64>,
    Json(task): Json<Task>
) -> Json<Task> {

    sqlx::query(
        r#"
        UPDATE tasks
        SET title = ?, done = ?
        WHERE id = ?
        "#
    )
    .bind(&task.title)
    .bind(task.done)    
    .bind(task_id)
    .execute(&pool)
    .await
    .unwrap()
    ;
    
    let updated_task = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, done, created_at
        FROM tasks
        WHERE id = ?
        "#
    )
    .bind(task_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    return Json(updated_task);
}