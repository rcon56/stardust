use anyhow::Result;
use axum::{http::StatusCode, service, Router};
use std::{convert::Infallible, net::SocketAddr, thread, time::Duration, str::FromStr};
use tower_http::services::ServeDir;

const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";

pub struct Server {
    host: String,
    port: u16,
    no_watch: bool,
}

impl Server {
    pub fn new(host: String, port: u16, no_watch: bool) -> Server {
        Server { host, port, no_watch }
    }

    pub async fn serve(&self) -> Result<()> {

        tokio::task::spawn_blocking(move || {

            let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");
            hotwatch
                .watch(CONTENT_DIR, |_| {
                    println!("Rebuilding site");
                    // TODO: rebuilding
                })
                .expect("failed to watch content folder!");
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        });
    
        let app = Router::new().nest(
            "/",
            service::get(ServeDir::new(PUBLIC_DIR)).handle_error(|error: std::io::Error| {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        );
    
        let netaddr = format!("{}:{}", self.host, self.port);
        let addr = SocketAddr::from_str(&netaddr)?;

        println!("serving site on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    
        Ok(())
    }
}