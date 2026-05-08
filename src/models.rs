#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,  // false on creation
    pub created_at: String // ISO timestamp
}

pub struct CreateTaskInput {
    title: String
}