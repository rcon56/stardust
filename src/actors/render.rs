use anyhow;
use handlebars::Handlebars;
// use pulldown_cmark::Parser;

use super::site::Site;

#[derive(Debug, Clone)]
pub struct RenderContext<'a> {
    pub site: &'a Site,
    pub tpl_render: &'a Handlebars<'a>,
//     pubmd_render: &'a Parser,
}

pub trait Renderable {
//     fn render(&self, ctx: &RenderContext) -> anyhow::Result<String>;
    fn render_to_write(&self, ctx: &RenderContext) -> anyhow::Result<()>;
}