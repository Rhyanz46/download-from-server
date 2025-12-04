use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "download-from-server")]
#[command(about = "Download files from VPS servers using SSH keys")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new server configuration
    Add {},
    /// Download a file from a configured server (default command)
    Download {
        /// Server alias from configuration
        alias: String,
        /// Remote file path to download
        remote_path: String,
        /// Local directory to save the file (optional, defaults to current directory)
        #[arg(short = 'd', long)]
        destination: Option<String>,
    },
    /// Remove a server configuration
    Remove {
        /// Server alias to remove
        alias: String,
    },
    /// List all configured servers
    List,
}