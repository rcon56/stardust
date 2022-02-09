// use std::fs;
// use std::collections::BTreeMap;
use anyhow;

use crate::actors::render::BASE_TEMPLATE;

use super::poster::Poster;
use super::render::{RenderContext, Renderable};
use super::page::{Page, PageData};
use super::post::{Post, Front};
use super::list::{List, Entry};
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

        let posts = self.find_posts();

        for post in posts.iter() {
            self.make_post(post).render_to_write(ctx)?;
        }

        self.make_list("posts", &posts).render_to_write(ctx)?;

        println!("Build posts ok.");

        Ok(())
    }

    fn find_posts(&self) -> Vec<Post> {

        Poster::read_md_in_dir(&self.post_dir)
            .iter()
            .map(|(front, body)| -> anyhow::Result<Post> {

                println!("? front: {:?}, body: {:?}", front, body);
                let post_front: Front = serde_yaml::from_str(front)?;
                let parser = pulldown_cmark::Parser::new_ext(&body,  pulldown_cmark::Options::all());  // TODO: options
                let mut content = String::new();
                pulldown_cmark::html::push_html(&mut content, parser);
                
                Ok(Post {
                    url: format!("/posts/{}", title2path(&post_front.title)),
                    date: post_front.date,
                    author: post_front.author.unwrap_or("Unknown".to_string()),
                    title: post_front.title,
                    tags: post_front.tags.unwrap_or(vec!()),
                    content: content,
                })
            })
            .filter_map(|r| r.ok() )
            .collect()

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
                kind: "main".to_string(),
            },
            item: None,
        }
    }


    fn make_post(&self, post: &Post) -> Page<Post> {
        Page {
            file_dir: self.output_dir.to_string(),
            url_path: post.url.to_string(),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                kind: "post".to_string(),
            },
            item: Some(post.clone()),
        }
    }

    fn make_list(&self, list_name: &str, posts: &[Post]) -> Page<List> {
        Page {
            file_dir: self.output_dir.to_string(),
            url_path: format!("/{}", list_name),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                kind: "list".to_string(),
            },
            item: Some(List {
                date: "DATE".to_string(),
                title: list_name.to_string(),
                entries: posts.iter().map(|p| Entry {
                    title: p.title.clone(),
                    date: p.date.clone(),
                    url: p.url.clone(),
                }).collect(),
            })
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