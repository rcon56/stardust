use clap::Parser;
use anyhow::{Result, bail};

mod server;

#[derive(Clone, Parser, Debug)]
#[clap(author = "rcon", version = "0.0.1", about = "Handshake with Stardust", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, PartialEq, Eq, Parser)]
enum Command {
    Serve {
        #[clap(short = 'h', long, value_name = "host", default_value = "localhost")]
        host: String,

        #[clap(short = 'p', long, value_name = "port", default_value = "8080")]
        port: u16,

        /// Disable rebuilding on change
        #[clap(long)]
        no_watch: bool,
    },
}

impl Args {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Serve {
                host, 
                port, 
                no_watch,
            } => { 
                println!("serve..."); 
                server::Server::new(host.clone(), *port, *no_watch).serve();
                Ok(())
            },
            _ => bail!("Unsupported command"),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    args.run()?;
    Ok(())
}