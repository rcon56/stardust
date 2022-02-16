use anyhow;
use handlebars::Handlebars;
// use pulldown_cmark::Parser;

use crate::utils;

use super::site::Site;
use super::config::Config;

pub const BASE_TEMPLATE: &str = "base";
const FOOTER_TEMPLATE: &str = "footer";
const HEAD_TEMPLATE: &str = "head";
const HEADER_TEMPLATE: &str = "header";
const POST_TEMPLATE: &str = "single";
const LIST_TEMPLATE: &str = "list";
const COLL_TEMPLATE: &str = "coll";

#[derive(Debug, Clone)]
pub struct RenderContext<'a> {
    pub site: &'a Site,
    pub tpl_render: Handlebars<'a>,
//     pubmd_render: &'a Parser,
}

pub trait Renderable {
//     fn render(&self, ctx: &RenderContext) -> anyhow::Result<String>;
    fn render_to_write(&self, ctx: &RenderContext) -> anyhow::Result<()>;
}

impl<'a> RenderContext<'a> {

    pub fn new(site: &'a Site, config: &'a Config) -> RenderContext<'a> {

        let mut ctx = RenderContext {
            site,
            tpl_render: Handlebars::new(),
        };

        // TODO: more registers
        ctx.tpl_render
            .register_template_file(BASE_TEMPLATE, format!("{}{}/_default/base.hbs", &config.base_dir, &config.layout_dir))
            .expect("Register base template error");

        ctx.tpl_render
            .register_template_file(POST_TEMPLATE, format!("{}{}/_default/single.hbs", &config.base_dir, &config.layout_dir))
            .expect("Register post template error");

        ctx.tpl_render
            .register_template_file(LIST_TEMPLATE, format!("{}{}/_default/list.hbs", &config.base_dir, &config.layout_dir))
            .expect("Register list template error");

        ctx.tpl_render
            .register_template_file(COLL_TEMPLATE, format!("{}{}/_default/coll.hbs", &config.base_dir, &config.layout_dir))
            .expect("Register coll template error");

        ctx.tpl_render
            .register_template_file(FOOTER_TEMPLATE, format!("{}{}/partials/footer.hbs", &config.base_dir, &config.layout_dir))
            .expect("Register footer template error");

        ctx.tpl_render
            .register_template_file(HEAD_TEMPLATE, format!("{}{}/partials/head.hbs", &config.base_dir, &config.layout_dir))
            .expect("Register head template error");

        ctx.tpl_render
            .register_template_file(HEADER_TEMPLATE, format!("{}{}/partials/header.hbs", &config.base_dir, &config.layout_dir))
            .expect("Register header template error");

        ctx.tpl_render
            .register_helper("abs-url", Box::new(abs_url_helper));

        ctx.tpl_render
            .register_helper("as-path", Box::new(path_helper));


        return ctx;
    }
}

fn abs_url_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    c: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {

    let url = h.params()
        .iter()
        .filter_map(|p| p.value().as_str() )
        .collect::<Vec<&str>>()
        .join("/");

    // println!("param---: {:?}", h.params());
    // println!("url---: {}", url);
    
    // get parameter from helper or throw an error
    // let param = h
    //     .param(0)
    //     .ok_or(handlebars::RenderError::new("Param 0 is required for abs-url helper."))?
    //     .value()
    //     .as_str()
    //     .ok_or(handlebars::RenderError::new("Param 0 is required to be string."))?;

    let abs_prefix = c.data()
        .get("site")
        .ok_or(handlebars::RenderError::new("`site` is required in context"))?
        .get("base_url")
        .ok_or(handlebars::RenderError::new("`site.base_url` is required in context"))?
        .as_str()
        .ok_or(handlebars::RenderError::new("`site.base_url` is required to be string"))?;

    let rendered = format!("http://{}/{}", abs_prefix, url);
    out.write(rendered.as_ref())?;
    Ok(())
}

fn path_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    c: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {

    let param = h
        .param(0)
        .ok_or(handlebars::RenderError::new("Param 0 is required for path helper."))?
        .value()
        .as_str()
        .ok_or(handlebars::RenderError::new("Param 0 is required to be string."))?;

    out.write(&utils::str2path(param))?;
    Ok(())
}
