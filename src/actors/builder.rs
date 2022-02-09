// use std::fs;
// use std::collections::BTreeMap;
use anyhow;

use crate::actors::render::BASE_TEMPLATE;

use super::poster::Poster;
use super::render::{RenderContext, Renderable};
use super::page::{Page, PageData};
use super::post::Post;
use super::config::Config;

pub struct Builder {
    post_dir: String,
    output_dir: String,
}

impl Builder {

    pub fn from_config(config: &Config) -> Builder {
        Builder {
            post_dir: format!("{}{}", &config.base_dir, &config.post_dir),
            output_dir: format!("{}{}", &config.base_dir, &config.output_dir),
        }
    }

    // pub fn new(idir: &str, odir: &str) -> Builder {
    //     Builder {
    //         input_dir: idir.to_string(),
    //         output_dir: odir.to_string(),
    //     }
    // }

    pub fn build(&self, ctx: &RenderContext) -> anyhow::Result<()> {

//        println!("? main page: {:?}", main_page);

        self.make_main().render_to_write(ctx)?;
        println!("Build main page ok.");

        self.find_posts()
            .into_iter()
            .try_for_each(|p| self.make_post(p).render_to_write(ctx) )?;

        println!("Build posts ok.");

        Ok(())
    }

    fn find_posts(&self) -> Vec<Post> {

        Poster::read_md_in_dir(&self.post_dir)
            .iter()
            .map(|(front, body)| -> anyhow::Result<Post> {
                println!("? front: {:?}, body: {:?}", front, body);
                let mut post: Post = serde_yaml::from_str(front)?;
                let parser = pulldown_cmark::Parser::new_ext(&body,  pulldown_cmark::Options::all());  // TODO: options
                let mut content = String::new();
                pulldown_cmark::html::push_html(&mut content, parser);
                post.content = Some(content);
                println!("post: {:?}", post);
                Ok(post)
            })
            .filter_map(|r| r.ok() )
            .collect()

        // let mdv: Vec<String> = walkdir::WalkDir::new(&self.input_dir)
        //     .into_iter()
        //     .filter_map(|e| e.ok())
        //     .filter(|e| e.path().display().to_string().ends_with(".md"))
        //     .map(|e| e.path().display().to_string())
        //     .map(|f| fs::read_to_string(&f).unwrap_or("".to_string()) )
        //     .map(|md| {
        //         let parser = pulldown_cmark::Parser::new_ext(&md,  pulldown_cmark::Options::all());
        //         let mut body = String::new();
        //         pulldown_cmark::html::push_html(&mut body, parser);
        //         body
        //     })
        //     .collect();

        // vec![Post {
        //     date: "Feb 8 2022".to_string(),
        //     author: "lds56".to_string(),
        //     title: "test".to_string(),
        //     content: "<h1>Hellow world</h1><p>This is so cool!!</p>".to_string(),
        //     tags: vec!["foo".to_string()],
        // }]
    }

    fn make_main(&self) -> Page<Post> {
        Page {
            file_dir: self.output_dir.to_string(),
            url_path: "/".to_string(),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                is_home: true,
            },
            item: None,
        }
    }


    fn make_post(&self, post: Post) -> Page<Post> {
        Page {
            file_dir: self.output_dir.to_string(),
            url_path: format!("/posts/{}", title2path(&post.title)),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                is_home: false,
            },
            item: Some(post),
        }
    }
}


fn title2path(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c == ' ' {'-'} else {c})
        .filter(|c| c == &'-' || c.is_alphanumeric())
        .collect()
}