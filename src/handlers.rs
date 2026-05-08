use axum::{Json, debug_handler};

use crate::models::Task;

#[debug_handler]
pub async fn get_all_tasks() -> Json<Task>{
    Json::from(Task{
        id: ,
        title: ,
        done: false,
        created_at: 
    })
}

pub async fn create_task(){
    
}