use clap::{Parser, Subcommand};
use crate::config::setup::init_wizard;
use crate::core::sync::{sync, mount, status};
use crate::core::rclone;

mod config {
    pub mod setup;
}
mod core {
    pub mod rclone;
    pub mod sync;
}
mod ui {
    pub mod tui;
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Sync,
    Mount,
    Status,
    Version,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init => init_wizard(),
        Commands::Sync => sync(),
        Commands::Mount => mount(),
        Commands::Status => status(),
        Commands::Version => {
            println!("gsync {}", env!("CARGO_PKG_VERSION"));
            if let Err(e) = rclone::run_rclone(&["--version"]) {
                eprintln!("Failed to get rclone version: {e}");
            }
        }
    }
}
