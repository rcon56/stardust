use serde::{Serialize, Deserialize};

use super::item::Item;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Front {
    pub date: String,
    pub author: Option<String>,
    pub title: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub date: String,
    pub author: String,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}

impl Item for Post {
    fn render_key(&self) -> &str {
        "post"
    }
}


