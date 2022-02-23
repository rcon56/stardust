use std::fs;
use std::option::Option;
use serde::{Serialize, Deserialize};
use serde_json::Map;
use handlebars::to_json;
use anyhow;

use super::paginator::Paginator;
use super::render::{RenderContext, Renderable};

#[deprecated]
pub trait Block {
    fn kind(&self) -> &str;
}

pub struct PageArg<'a> {
    pub title: &'a str,
    pub url: &'a str,
    pub ekind: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub summary: String,
    pub author: String,
    pub block: String,      // block name
    pub kind: String,       // partial template name
    pub has_menu: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData<T: Serialize> {
    pub file_dir: String,
    pub url_path: String,
    pub tpl_name: String,
    pub page: Page,
    pub block: Option<T>,
    pub paginator: Option<Paginator>,
}

impl<T> Renderable for PageData<T> where T: Serialize {
    fn render_to_write(&self, ctx: &RenderContext) -> anyhow::Result<()> {
        
        let mut data = Map::new();
        data.insert("site".to_string(), to_json(&ctx.site));
        data.insert("page".to_string(), to_json(&self.page));
        if let Some(it) = &self.block {
            data.insert(self.page.block.clone(), to_json(it));
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