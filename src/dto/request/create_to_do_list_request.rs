use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateToDoListRequest {

    pub title: String,
    pub content: String
}