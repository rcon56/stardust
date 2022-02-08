use hotwatch::Hotwatch;	
use std::{thread, time::Duration};
use anyhow;

use super::render::RenderContext;
use super::builder::Builder;
use super::config::Config;

pub struct Watcher {
    hotwatch: Hotwatch,
    dir: String,
}

impl Watcher {

    pub fn from_config(config: &Config) -> Watcher {
        Watcher {
            hotwatch: Hotwatch::new().expect("hotwatch failed to initialize!"),
            dir: config.post_dir.clone(),
        }
    }
    
    // pub fn new(dir: &str) -> Watcher {
    //     Watcher {
    //         hotwatch: Hotwatch::new().expect("hotwatch failed to initialize!"),
    //         dir: dir.to_string()
    //     }
    // }

    // pub fn watch(&self, on_change: &dyn Fn() -> ()) -> anyhow::Result<()> {
    //     self.hotwatch.watch(&self.dir, |_| -> Result<(), hotwatch::Error> {
    //         on_change();
    //     })?;
    //     Ok(())
    // }

    // pub fn watch(&mut self) -> anyhow::Result<()> {
    //     self.hotwatch.watch(&self.dir, |_| { println!("Rebuilding..."); })?;
    //     Ok(())
    // }

    pub fn watching(&mut self, ctx: &RenderContext, config: &Config) -> anyhow::Result<()> {
//        self.watch().expect("failed to watch");

        let builder = Builder::from_config(config);
        builder.build(ctx)?;

        let abs_post_dir = format!("{}{}", &ctx.site.base_dir, &self.dir);

        self.hotwatch.watch(&abs_post_dir, |_| { 
            println!("Rebuilding..."); 
        })?;

        loop {
            thread::sleep(Duration::from_secs(1));
        }

        Ok(())
    }
}