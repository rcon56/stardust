use std::{net::SocketAddr, str::FromStr, convert::Infallible};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use anyhow;
use axum::{http::StatusCode, service, Router};
use tower_http::services::ServeDir;

use super::render::RenderContext;
use super::watcher::Watcher;
use super::builder::Builder;
use super::config::Config;
use super::site::{Site};

const LOCALHOST_ALIAS: &str = "localhost";
const LOCALHOST: &str = "127.0.0.1";

pub struct Server {
    net_addr: SocketAddr,
    file_addr: ServeDir,
    no_watch: bool,
}

impl Server {

    pub fn new(host: &str, port: &u16, no_watch: &bool, odir: &str) -> Server {

        let net_addr = format!("{}:{}", 
            if host == LOCALHOST_ALIAS {LOCALHOST} else {host}, port);

        Server {
            net_addr: SocketAddr::from_str(&net_addr).expect("Invalid net addr"),
            file_addr: ServeDir::new(odir),
            no_watch: *no_watch,
        }
    }

    pub async fn serve(&self, config: Config) -> anyhow::Result<()> {

        let site = Site::new_base(self.net_addr.to_string());
        let no_watch = self.no_watch;        
        println!("? site: {:?}", site);
    
        tokio::task::spawn_blocking(move || {
    
            println!("watching...");
            let ctx = RenderContext::new(&site, &config);
            let builder = Builder::from_config(&config);
            builder.build(&ctx).expect("Failed to build");

            let fc = AtomicBool::new(false).into();

            if !no_watch {
                let mut watcher = Watcher::from_config(&config);
                match watcher.watch(Arc::clone(&fc)) {
                    Ok(_)  => println!("Watch ok!"),
                    Err(e) => println!("Watch error: {}", e),
                }

                println!("monitoring...");
                loop {
                    if fc.load(Ordering::Relaxed) {
                        println!("Rebuilding...");                     
                        if let Err(e) = builder.build(&ctx) {
                            println!("Rebuilding err - {}", e.to_string());
                        }
                        fc.store(false, Ordering::Relaxed);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        });
    
    
        println!("launching...");
        let app = Router::new().nest(
            "/",
            service::get(self.file_addr.to_owned()).handle_error(|error: std::io::Error| {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        );

        println!("serving site on {}", self.net_addr);
        axum::Server::bind(&self.net_addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}