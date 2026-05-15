use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,  // false on creation
    pub created_at: String // ISO timestamp
}

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct CreateTaskInput {
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTaskInput {
    pub title: String,
    pub done: bool,
}