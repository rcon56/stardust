pub struct Config {
    pub host: String,
    pub port: u8,
    pub base_dir: String,
    pub post_dir: String,
    pub output_dir: String,
    pub layout_dir: String,
    pub no_watch: bool,
}

impl Config {
    pub fn load(path: &str) -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 1080,
            base_dir: "./site/".to_string(),
            post_dir: "content/".to_string(),
            output_dir: "public/".to_string(),
            layout_dir: "layout/".to_string(),
            no_watch: false,
        }
    }
}