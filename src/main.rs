mod dto;
mod entity;
mod service;
mod handler;

use axum::{
    Router,
};
use sea_orm::{Database, DbConn};
use std::env;
use axum::handler::Handler;
use axum::routing::{get, post};
use handler::todo_handler::create_handler;
use crate::dto::request::create_to_do_list_request::CreateToDoListRequest;
use crate::handler::todo_handler::read_handler;

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
        .with_state(db);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}