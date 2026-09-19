#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use axum::{
    Router,
    routing::{delete, get, post},
};

mod handlers;
mod models;
mod persistance;

use handlers::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to create the Postgres connection pool");

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
