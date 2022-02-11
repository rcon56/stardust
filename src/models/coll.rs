use serde::{Serialize, Deserialize};

use super::item::Item;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollEntry {
    pub title: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coll {
    pub title: String,
    pub kind: String,
    pub entries: Vec<CollEntry>,
}

impl Item for Coll {
    fn render_key(&self) -> &str {
        "coll"
    }
}


