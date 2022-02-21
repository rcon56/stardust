use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Site {
    pub base_url: String,
    pub title: String,
    pub description: String,
}

// #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
// pub struct Site {
//     pub data: SiteData,
// }