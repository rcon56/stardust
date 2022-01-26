use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::{Map, Value as Json};
use handlebars::to_json;
use anyhow;

use super::render::{RenderContext, Renderable};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub url_path: String,
    pub tpl_path: String,
    pub content: String,
}

impl Renderable for Page {
    fn render_to_write(&self, ctx: &RenderContext) -> anyhow::Result<()> {
        
        let mut data = Map::new();
        data.insert("site".to_string(), to_json(ctx.site));
        data.insert("page".to_string(), to_json(self));

        let mut output_file = File::create(ctx.site.base_dir + &self.url_path)?;

        ctx.tpl_render.render_to_write(&self.tpl_path, &data, output_file)?;

        Ok(())
    }
}