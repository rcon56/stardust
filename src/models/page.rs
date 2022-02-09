use std::fs;
use std::option::Option;
use serde::{Serialize, Deserialize};
use serde_json::Map;
use handlebars::to_json;
use anyhow;

use super::item::Item;
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
pub struct Page<T: Item + Serialize> {
    pub file_dir: String,
    pub url_path: String,
    pub tpl_name: String,
    pub data: PageData,
    pub item: Option<T>,
}

impl<T> Renderable for Page<T> where T: Item + Serialize {
    fn render_to_write(&self, ctx: &RenderContext) -> anyhow::Result<()> {
        
        let mut data = Map::new();
        data.insert("site".to_string(), to_json(&ctx.site.data));
        data.insert("page".to_string(), to_json(&self.data));
        if let Some(it) = &self.item {
            data.insert(it.render_key().to_string(), to_json(it));
        }

//        println!("data: {:?}", data);
        let file_path = format!("{}{}", &self.file_dir, &self.url_path);
        println!("dir: {:?}", &file_path);

        fs::create_dir_all(&file_path)?;
        let output_file = fs::File::create(format!("{}/index.html", &file_path))?;

        ctx.tpl_render.render_to_write(&self.tpl_name, &data, output_file)?;

        Ok(())
    }
}