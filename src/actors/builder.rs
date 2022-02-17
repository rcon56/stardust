// use std::fs;
// use std::collections::BTreeMap;
use std::collections::{HashSet, HashMap};
use anyhow;

use crate::models::paginator::Paginator;
use crate::utils;

use super::poster::Poster;
use super::render::{RenderContext, Renderable, BASE_TEMPLATE};
use super::page::{Page, PageData, PageArg};
use super::post::{Post, Front};
use super::list::{List, ListEntry};
use super::config::Config;

const POST_NUM_PER_PAGE: usize = 2;

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

    pub fn build(&self, ctx: &RenderContext) -> anyhow::Result<()> {

//        println!("? main page: {:?}", main_page);

        self.make_main().render_to_write(ctx)?;
        println!("Build main page ok.");

        let mut posts = self.find_posts();
        posts.sort();
        // println!("Posts: {:?}", posts);
        // let tags: HashSet<String> = HashSet::from_iter(
           // posts.iter().flat_map(|p| p.tags.iter()).into_iter().cloned());

        let mut tag2posts = HashMap::new();
        for post in posts.iter() {
            for tag in post.tags.iter() {
                tag2posts.entry(tag)
                    .or_insert(vec![])
                    .push(post);
            }
        }

        let mut cate2posts = HashMap::new();
        for post in posts.iter() {
            cate2posts.entry(&post.category)
                .or_insert(vec![])
                .push(post);
        }

        for post in posts.iter() {
            self.make_post(post).render_to_write(ctx)?;
        }

        /////////////////// tags //////////////////////
        // self.make_post(&Post {
        //     date: "".to_string(),
        //     author: "".to_string(),
        //     title: "tag/all".to_string(),
        //     tags: tags.iter().cloned().collect::<Vec<String>>(),
        //     content: "".to_string(),
        // }).render_to_write(ctx)?;
        ///////////////////////////////////////////////
        self.make_lists_with_paginator(&PageArg {title: "POSTS", url: "/post", ekind: Some("post")}, &posts.iter().collect::<Vec<_>>())
            .iter()
            .try_for_each(|p| p.render_to_write(ctx).ok());

        self.make_coll(&PageArg{title: "TAGS", url: "/tag", ekind: Some("tag")},
            &tag2posts).render_to_write(ctx)?;

        self.make_coll(&PageArg{title: "CATEGORIES", url: "/category", ekind: Some("category")},
            &cate2posts).render_to_write(ctx)?;

        for (tag, posts_in_tag) in tag2posts.iter() {
            self.make_lists_with_paginator(&PageArg{title: tag, url: &format!("/tag/{}", tag), ekind: Some("post")}, posts_in_tag)
                .iter()
                .try_for_each(|p| p.render_to_write(ctx).ok());
        }

        for (tag, posts_in_cate) in cate2posts.iter() {
            self.make_lists_with_paginator(&PageArg{title: tag, url: &format!("/category/{}", tag), ekind: Some("post")}, posts_in_cate)
                .iter()
                .try_for_each(|p| p.render_to_write(ctx).ok());                
        }

        println!("Build posts ok.");

        Ok(())
    }

    fn find_posts(&self) -> Vec<Post> {

        Poster::read_md_in_dir(&self.post_dir)
            .iter()
            .map(|(front, body)| -> anyhow::Result<Post> {

                println!("? front: {:?}, body: {:?}", front, body);
                let post_front: Front = serde_yaml::from_str(front)?;
                let post_url = format!("/post/{}", utils::str2path(&post_front.title));

                let parser = pulldown_cmark::Parser::new_ext(&body,  pulldown_cmark::Options::all());  // TODO: options
                let mut content = String::new();
                pulldown_cmark::html::push_html(&mut content, parser);
                
                Post::write(post_front,  post_url, content)

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
            block: None,
            paginator: None,
        }
    }

    fn make_post(&self, post: &Post) -> Page<Post> {
        Page {
            file_dir: self.output_dir.to_string(),
            url_path: format!("/post/{}", utils::str2path(&post.title)),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                kind: "post".to_string(),
            },
            block: Some(post.clone()),
            paginator: None,   // TODO: add post paginator
        }
    }

    fn make_coll(&self, arg: &PageArg, post_group: &HashMap<&String, Vec<&Post>>) -> Page<List> {
        Page {
            file_dir: self.output_dir.to_string(),
            url_path: arg.url.to_string(),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                kind: "coll".to_string(),
            },
            block: Some(List {
                title: arg.title.to_string(),
                kind: arg.ekind.unwrap_or("").to_string(),
                entries: post_group.iter().map(|(k, p)| 
                    ListEntry {
                        title: k.to_string(),
                        date: "".to_string(),
                        count: p.len(),
                    }).collect()
            }),
            paginator: None,
        }
    }

    fn make_lists_with_paginator(&self, arg: &PageArg, posts: &[&Post]) -> Vec<Page<List>> {
        let pg_sz = (posts.len() as f32 / POST_NUM_PER_PAGE as f32).ceil() as usize;
        posts.chunks(POST_NUM_PER_PAGE)
            .enumerate()
            .map(|(pg_idx, post_chunk)| -> Page<List> {
                let pg = Paginator::new_at(pg_sz, pg_idx as i32, arg.url);
                self.make_list(&PageArg {
                    title: arg.title, 
                    url: &Paginator::gen_url(pg_sz, pg_idx as i32, arg.url).unwrap_or("".to_string()), 
                    ekind: Some("post"),
                }, post_chunk, Some(pg))
            })
            .collect()
    }


    fn make_list(&self, arg: &PageArg, posts: &[&Post], pg: Option<Paginator>) -> Page<List> {
        Page {
            file_dir: self.output_dir.to_string(),
            url_path: arg.url.to_string(),
            tpl_name: BASE_TEMPLATE.to_string(),
            data: PageData {
                content: "".to_string(),
                summary: "This is summary.".to_string(),
                author: "lds56".to_string(),
                has_menu: false,
                kind: "list".to_string(),
            },
            block: Some(List {
                kind: arg.ekind.unwrap_or("").to_string(),
                title: arg.title.to_string(),
                entries: posts.iter().map(|p| ListEntry {
                    title: p.title.clone(),
                    date: p.date_str(),
                    count: 1usize,
                }).collect(),
            }),
            paginator: pg,
        }
    }

    // fn make_filter_list(&self, list_name: &str, posts: &[Post], pred: impl FnMut(&&Post) -> bool) -> Page<List> {
    //     Page {
    //         file_dir: self.output_dir.to_string(),
    //         url_path: format!("/{}", list_name),
    //         tpl_name: BASE_TEMPLATE.to_string(),
    //         data: PageData {
    //             content: "".to_string(),
    //             summary: "This is summary.".to_string(),
    //             author: "lds56".to_string(),
    //             has_menu: false,
    //             kind: "list".to_string(),
    //         },
    //         item: Some(List {
    //             date: "DATE".to_string(),
    //             title: list_name.to_string(),
    //             entries: posts.iter().filter(pred).map(|p| Entry {
    //                 title: p.title.clone(),
    //                 date: p.date.clone(),
    //             }).collect(),
    //         })
    //     }
    // }

}