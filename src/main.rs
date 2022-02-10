use clap::Parser;
use anyhow::{Result, bail};

mod actors;
mod models;
mod utils;

#[derive(Clone, Parser, Debug)]
#[clap(author = "rcon", version = "0.0.1", about = "Handshake with Stardust", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
enum Command {
    Serve {
        #[clap(short = 'H', long, value_name = "host", default_value = "127.0.0.1")]
        host: String,

        #[clap(short = 'p', long, value_name = "port", default_value = "8080")]
        port: u16,

        /// Disable rebuilding on change
        #[clap(long)]
        no_watch: bool,

        #[clap(short = 'c', long, value_name = "config", default_value = "site.toml")]
        config: String,
    },
    Build {
        #[clap(short = 'c', long, value_name = "config", default_value = "site.toml")]
        config: String,
    },
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
                let serve_dir = format!("{}{}", &cfg.base_dir, &cfg.output_dir);
                actors::server::Server::new(host, port, no_watch, &serve_dir).serve(cfg).await
            },
            _ => bail!("Unsupported command"),
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    args.run().await
}