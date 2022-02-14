use serde::{Serialize, Deserialize};

use super::page::Block;

const KIND: &str = "list";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEntry {
    pub title: String,
    pub date: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub title: String,
    pub kind: String,
    pub entries: Vec<ListEntry>,
}

impl Block for List {
    fn kind(&self) -> &str {
        "list"
    }
}