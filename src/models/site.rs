use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Site {
    pub base_url: String,
    pub title: String,
    pub description: String,
}

impl Site {
    pub fn new_base(base_url: String) -> Site {
        Site {
            base_url,
            title: "Stardust Ocean".to_string(),
            description: "Unbreakable Ruby!".to_string(),
        }
    }

    pub fn new() -> Site {
        Site {
            base_url: "".to_string(),
            title: "Stardust Ocean".to_string(),
            description: "Unbreakable Ruby!".to_string(),
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
// pub struct Site {
//     pub data: SiteData,
// }