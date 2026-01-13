use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToDoList {
    id: u64,
    title: String,
    content: String
}

impl ToDoList {

    // 생성자
    pub fn new(id: u64, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content
        }
    }

    // Setter
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    // Getter
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}