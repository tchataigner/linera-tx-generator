use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use linera_service::cli_wrappers::local_net::PathProvider;
use linera_service::cli_wrappers::{ClientWrapper, Network};
use log::info;
use std::fmt::Display;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Local mode with `storage-address` and `path` arguments
    Local {
        /// Case to execute against the network
        #[arg(short, long)]
        case: Case,

        /// Path to local storage
        #[arg(short, long)]
        path: String,

        /// Optional port for the node service
        #[arg(short, long)]
        node_service_port: Option<u16>,
    },

    /// Remote mode with `url` argument
    Remote {
        /// URL for remote access
        #[arg(long)]
        url: String,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Case {
    /// Simple counter case
    Counter,
    /// Simple blob case with small and large blobs
    Blob,
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Case::Counter => write!(f, "counter"),
            Case::Blob => write!(f, "blob"),
        }
    }
}

mod blob;
mod counter;

const DEFAULT_PORT_NODE_SERVICE: u16 = 8080;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::init();

    match cli.mode {
        Mode::Local {
            case,
            path,
            node_service_port,
        } => {
            info!("Running in local mode with case: {}", case);

            let path_provider = PathProvider::new(&Some(path))?;
            let external = Network::Grpc;
            let client = ClientWrapper::new(path_provider, external, None, 0);
            let chain = client
                .load_wallet()?
                .default_chain()
                .expect("No default chain");
            let node_service_port = node_service_port.unwrap_or(DEFAULT_PORT_NODE_SERVICE);

            match case {
                Case::Counter => {
                    counter::run_counter_case(&chain, &client, node_service_port).await
                }
                Case::Blob => blob::run_blob_case(&chain, &client).await,
            }
        }
        Mode::Remote { url } => {
            println!("Running in remote mode with:");
            println!("URL: {}", url);

            //TODO handle remote net

            Ok(())
        }
    }
}
