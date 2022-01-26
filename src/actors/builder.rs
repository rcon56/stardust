use anyhow;

use super::render::{RenderContext, Renderable};
use super::page::Page;
use super::config::Config;

const BASE_HTML: &str = "_default/base.html";

pub struct Builder {
    input_dir: String,
    output_dir: String,
}

impl Builder {

    pub fn from_config(config: &Config) -> Builder {
        Builder {
            input_dir: config.layout_dir,
            output_dir: config.output_dir,
        }
    }

    pub fn new(idir: &str, odir: &str) -> Builder {
        Builder {
            input_dir: idir.to_string(),
            output_dir: odir.to_string(),
        }
    }

    pub fn build(&self, ctx: &RenderContext) -> anyhow::Result<()> {

        let main_page = Page {
            url_path: "/index.html".to_string(),
            tpl_path: self.input_dir + &BASE_HTML,
            content: "".to_string(),
        };

        main_page.render_to_write(ctx)?;

        Ok(())
    }
}