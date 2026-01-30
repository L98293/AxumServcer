mod dto;
mod entity;
mod service;
mod handler;

use axum::{
    Router,
};
use sea_orm::{Database, DbConn};
use std::env;
use axum::routing::{get, post, put};
use handler::todo_handler::create_handler;
use crate::handler::todo_handler::{read_all_handler, read_handler, update_handler};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let db_url = env::var("DB_URL")
        .unwrap();
    
    let db: DbConn = Database::connect(&db_url)
        .await
        .unwrap();
    
    let app = Router::new()
        .route("/create", post(create_handler))
        .route("/read/{id}", get(read_handler))
        .route("/read", get(read_all_handler))
        .route("/update/{id}", put(update_handler))
        .with_state(db);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}