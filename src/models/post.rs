use serde::{Serialize, Deserialize};

use super::item::Item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub date: String,
    pub author: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Item for Post {
    fn render_key(&self) -> &str {
        "post"
    }
}

// <footer class="post-tags">
// {{#each post.tags}}
// <!-- "tags/"  -->
//     <a href="{{ abs-url . }}">{{ . }}</a>
// {{/each}}

// </footer>