use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::DbConn;
use crate::dto::request::create_to_do_list_request::CreateToDoListRequest;
use crate::dto::request::update_to_do_list_request::UpdateToDoListRequest;
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

pub async fn read_all_handler(
    State(db): State<DbConn>
) -> Result<Json<Vec<todo::Model>>, StatusCode> {

    ToDoService::read_all_todo_list(&db)
        .await
        .map(Json)
        .map_err(|e| StatusCode::NOT_FOUND)
}

pub async fn update_handler(
    State(db): State<DbConn>,
    Path(id): Path<u64>,
    Json(request): Json<UpdateToDoListRequest>
) -> Result<Json<todo::Model>, StatusCode> {

    ToDoService::update_todo_list(&db, id, request)
        .await
        .map(Json)
        .map_err(|e| StatusCode::BAD_REQUEST)
}