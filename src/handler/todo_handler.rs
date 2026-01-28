use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::DbConn;
use crate::dto::request::create_to_do_list_request::CreateToDoListRequest;
use crate::entity::todo;
use crate::service::todo_service::ToDoService;

pub async fn create_handler(
    State(db): State<DbConn>,
    Json(request): Json<CreateToDoListRequest>
) -> Result<Json<todo::Model>, StatusCode> {

    ToDoService::create_todo_list(&db, request)
        .await
        .map(Json)
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn read_handler(
    State(db): State<DbConn>,
    Path(id): Path<u64>
) -> Result<Json<todo::Model>, StatusCode> {

    ToDoService::read_todo_list(&db, id)
        .await
        .map(Json)
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)
}