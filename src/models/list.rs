use serde::{Serialize, Deserialize};

use super::item::Item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub title: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub date: String,
    pub title: String,
    pub entries: Vec<Entry>,
}

impl Item for List {
    fn render_key(&self) -> &str {
        "list"
    }
}