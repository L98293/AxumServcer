use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateToDoListRequest {
    
    pub title: String,
    pub content: String
}