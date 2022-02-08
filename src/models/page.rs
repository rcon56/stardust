use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value as Json};
use handlebars::to_json;
use anyhow;

use super::render::{RenderContext, Renderable};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    pub content: String,
    pub summary: String,
    pub author: String,
    pub has_menu: bool,
    pub is_home: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub file_dir: String,
    pub url_path: String,
    pub tpl_name: String,
    pub data: PageData,
}

impl Renderable for Page {
    fn render_to_write(&self, ctx: &RenderContext) -> anyhow::Result<()> {
        
        let mut data = Map::new();
        data.insert("site".to_string(), to_json(&ctx.site.data));
        data.insert("page".to_string(), to_json(&self.data));

        let output_file = File::create(format!("{}{}", &self.file_dir, &self.url_path))?;

        ctx.tpl_render.render_to_write(&self.tpl_name, &data, output_file)?;

        Ok(())
    }
}