use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateToDoListRequest {
    
    title: Option<String>,
    content: Option<String>
}