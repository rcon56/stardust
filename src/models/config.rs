use std::fs;

use anyhow;
use toml::Value;

pub struct Config {
    pub post_dir: String,
    pub output_dir: String,
    pub layout_dir: String,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Config> {
        let cfg_str = fs::read_to_string(path)?;
        let cfg_toml = cfg_str.parse::<Value>()?;
        Ok(Config {
            post_dir: cfg_toml["build"]["dir"]["post"].as_str().unwrap_or("./site/content").to_string(),
            output_dir: cfg_toml["build"]["dir"]["output"].as_str().unwrap_or("./site/content").to_string(),
            layout_dir: cfg_toml["build"]["dir"]["layout"].as_str().unwrap_or("./site/content").to_string(),
        })
        // Ok(Config {
        //     post_dir: "./site/content/".to_string(),
        //     output_dir: "./site/public/".to_string(),
        //     layout_dir: "./site/layouts/".to_string(),
        // })
    }
}