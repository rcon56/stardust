use std::fs;
use std::option::Option;
use serde::{Serialize, Deserialize};
use serde_json::Map;
use handlebars::to_json;
use anyhow;

use super::paginator::Paginator;
use super::render::{RenderContext, Renderable};

pub trait Block {
    fn kind(&self) -> &str;
}


pub struct PageArg<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub ekind: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    pub content: String,
    pub summary: String,
    pub author: String,
    pub kind: String,
    pub has_menu: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T: Serialize> {
    pub file_dir: String,
    pub url_path: String,
    pub tpl_name: String,
    pub data: PageData,
    pub block: Option<T>,
    pub paginator: Option<Paginator>,
}

impl<T> Renderable for Page<T> where T: Serialize {
    fn render_to_write(&self, ctx: &RenderContext) -> anyhow::Result<()> {
        
        let mut data = Map::new();
        data.insert("site".to_string(), to_json(&ctx.site));
        data.insert("page".to_string(), to_json(&self.data));
        if let Some(it) = &self.block {
            data.insert(self.data.kind.clone(), to_json(it));
        }
        if let Some(pg) = &self.paginator {
            data.insert("paginator".to_string(), to_json(pg));
        }

        // println!("data: {:?}", data);
        let file_path = format!("{}{}", &self.file_dir, &self.url_path);
        println!("dir: {:?}", &file_path);

        fs::create_dir_all(&file_path)?;
        let output_file = fs::File::create(format!("{}/index.html", &file_path))?;

        ctx.tpl_render.render_to_write(&self.tpl_name, &data, output_file)?;

        Ok(())
    }
}