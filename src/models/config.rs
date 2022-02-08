use anyhow;

pub struct Config {
    pub base_dir: String,
    pub post_dir: String,
    pub output_dir: String,
    pub layout_dir: String,
}

impl Config {
    pub fn load(_: &str) -> anyhow::Result<Config> {
        Ok(Config {
            base_dir: "./site/".to_string(),
            post_dir: "content/".to_string(),
            output_dir: "public/".to_string(),
            layout_dir: "layouts/".to_string(),
        })
    }
}