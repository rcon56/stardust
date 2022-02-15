use serde::{Serialize, Deserialize};

use super::page::Block;

const KIND: &str = "post";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Front {
    pub date: String,
    pub author: Option<String>,
    pub title: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub date: String,
    pub author: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub content: String,
    pub url: String,
}

impl Block for Post {
    fn kind(&self) -> &str {
        KIND
    }
}


