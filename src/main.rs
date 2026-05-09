use axum::{routing::get,Router};
mod models;
mod handlers;
mod db;

use handlers::*;

#[tokio::main]
async fn main() {
    // Connecting DB
    let pool = db::init_db().await;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async {"Hello"}))
        .route("/tasks", get(get_all_tasks).post(create_task))
        // .route("/tasks/:id", get().put().delete())
        .with_state(pool)
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();   
}