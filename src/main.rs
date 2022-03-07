use clap::{Parser, Subcommand};
use anyhow::{Result, bail};
use handlebars::RenderContext;

mod actors;
mod models;
mod utils;

use models::{site, config};
use actors::{builder, render};

#[derive(Parser)]
#[clap(author = "lds56", version = "0.0.1", about = "Handshake with Stardust", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Serve static site in working directory
    Serve {
        /// Specify serving host
        #[clap(short = 'H', long, value_name = "host", default_value = "127.0.0.1")]
        host: String,

        /// Specify serving port
        #[clap(short = 'p', long, value_name = "port", default_value = "8080")]
        port: u16,

        /// Disable rebuilding on change
        #[clap(long)]
        no_watch: bool,

        /// Use config file
        #[clap(short = 'c', long, value_name = "config", default_value = "site.toml")]
        config: String,
    },
    /// Build web html based on posts
    Build {
        /// Use config file
        #[clap(short = 'c', long, value_name = "config", default_value = "site.toml")]
        config: String,
    },
    /// Create new blank post in content directory
    New,
}

impl Args {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Command::Serve {
                host, 
                port, 
                no_watch,
                config,
            } => { 
                println!("serving...");             
                let cfg = models::config::Config::load(config)?;
                actors::server::Server::new(host, port, no_watch, &cfg.output_dir).serve(cfg).await
            },
            Command::Build { config} => {
                println!("building...");
                let cfg = config::Config::load(config)?;
                let builder = builder::Builder::from_config(&cfg);
                let site = site::Site::new();
                let ctx = render::RenderContext::new(&site, &cfg);
                builder.build(&ctx)
            }
            _ => bail!("Unsupported command"),
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    args.run().await
}