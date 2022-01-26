use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Site {
    pub title: String,
    pub description: String,
    pub base_url: String,
    pub base_dir: String,
}