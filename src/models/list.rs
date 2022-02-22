use serde::{Serialize, Deserialize};

use super::page::Block;
use super::entry::Entry;

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
    pub entries: Vec<Entry>,
}

impl Block for List {
    fn kind(&self) -> &str {
        "list"
    }
}