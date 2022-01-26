use std::{net::SocketAddr, str::FromStr, convert::Infallible};
use anyhow;
use axum::{http::StatusCode, service, Router};
use tower_http::services::ServeDir;
use handlebars::Handlebars;

const CONTENT_DIR: &str = "site/content";
const PUBLIC_DIR: &str = "site/public";

use super::render::RenderContext;
use super::watcher::Watcher;
use super::config::Config;
use super::site::Site;

const LOCALHOST_ALIAS: &str = "localhost";
const LOCALHOST: &str = "127.0.0.1";

pub struct Server {
    addr: SocketAddr,
    no_watch: bool,
}

impl Server {

    pub fn from_config(config: &Config) -> Server {

        let net_addr = format!("{}:{}", 
            if config.host == LOCALHOST_ALIAS {LOCALHOST} else {&config.host}, config.port);

        Server {
            addr: SocketAddr::from_str(&net_addr).expect("Invalid net addr"),
            no_watch: config.no_watch,
        }
    }

    pub fn new(addr: SocketAddr, no_watch: bool) -> Server {
        Server { addr, no_watch }
    }

    pub async fn serve(&self, config: &Config) -> anyhow::Result<()> {

        let site = Site {
            title: "Stardust Ocean".to_string(),
            description: "Unbreakable Ruby!".to_string(),
            base_url: self.addr.to_string(),
            base_dir: config.base_dir.clone()
        };

        let tpl_render = Handlebars::new();
        // TODO: more registers

        let ctx = RenderContext {
            site: &site,
            tpl_render: &tpl_render,
        };

        if !self.no_watch {
            println!("watching...");
            tokio::task::spawn_blocking(move || {
                let config = Config::load("");
                let mut watcher = Watcher::from_config(&config);
                watcher.watching(&ctx, &config);
            });
        }
    
        println!("launching...");
        let app = Router::new().nest(
            "/",
            service::get(ServeDir::new(PUBLIC_DIR)).handle_error(|error: std::io::Error| {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        );

        println!("serving site on {}", self.addr);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await?;
    
        println!("serve done");

        Ok(())
    }
}