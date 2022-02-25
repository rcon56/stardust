use hotwatch::Hotwatch;	
use hotwatch::notify::DebouncedEvent;
use std::{thread, time::Duration};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use anyhow;

use super::config::Config;

pub struct Watcher {
    hotwatch: Hotwatch,
    watch_dir: String,
}

impl Watcher {

    pub fn from_config(config: &Config) -> Watcher {
        Watcher {
            hotwatch: Hotwatch::new().expect("hotwatch failed to initialize!"),
            watch_dir: config.post_dir.clone(),
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

    pub fn watch(&mut self, file_changed: Arc<AtomicBool>) -> anyhow::Result<()> {

        self.hotwatch.watch(&self.watch_dir, move |ev| { 
            match ev {
                DebouncedEvent::Create(path) |
                DebouncedEvent::Write(path)  |
                DebouncedEvent::Remove(path)  => file_changed.store(true, Ordering::Relaxed),
                DebouncedEvent::Rename(path0, path1) => file_changed.store(true, Ordering::Relaxed),
                _ => {},
            }            
        })?;

        Ok(())
    }
}