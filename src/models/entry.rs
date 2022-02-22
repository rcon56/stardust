use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub title: String,
    pub date: String,
    pub count: Option<usize>,
    pub digest: Option<String>,
}