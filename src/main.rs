use axum::{
    Router,
    routing::{delete, get, post},
};

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/question", post(create_question))
        .route("/question", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answer", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
