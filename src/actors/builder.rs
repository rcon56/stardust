use anyhow;

use crate::actors::render::BASE_TEMPLATE;

use super::render::{RenderContext, Renderable};
use super::page::{Page, PageData};
use super::config::Config;

pub struct Builder {
    input_dir: String,
    output_dir: String,
}

impl Builder {

    pub fn from_config(config: &Config) -> Builder {
        Builder {
            input_dir: config.layout_dir.to_string(),
            output_dir: config.output_dir.to_string(),
        }
    }

    // pub fn new(idir: &str, odir: &str) -> Builder {
    //     Builder {
    //         input_dir: idir.to_string(),
    //         output_dir: odir.to_string(),
    //     }
    // }

    pub fn build(&self, ctx: &RenderContext) -> anyhow::Result<()> {

        let main_page = Page {
            file_dir: format!("{}{}", &ctx.site.base_dir, &self.output_dir),
            url_path: "/index.html".to_string(),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                is_home: true,
            },
        };

        println!("? main page: {:?}", main_page);

        main_page.render_to_write(ctx)?;

        println!("Build index ok.");

        Ok(())
    }
}