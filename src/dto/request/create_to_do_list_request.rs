use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateToDoListRequest {

    title: String,
    content: String
}