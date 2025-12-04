mod cli;
mod config;
mod ssh;
mod error;

use clap::Parser;
use cli::{Cli, Commands};
use config::{Config, ServerConfig};
use ssh::{SshClient};
use std::path::Path;
use std::io::{self, Write};
use anyhow::Result;
use dialoguer::{Input, Confirm};
use console::style;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle commands
    match cli.command {
        Commands::Add {} => {
            add_server_interactive().await
        },
        Commands::Download { alias, remote_path, destination } => {
            download_file(alias, remote_path, destination).await
        },
        Commands::Remove { alias } => {
            remove_server(alias).await
        },
        Commands::List => {
            list_servers().await
        },
    }
}

async fn add_server_interactive() -> Result<()> {
    loop {
        println!("{}", style("=== Add New Server Configuration ===").bold().cyan());
        println!();

        // Step 1: Get alias
        let alias = Input::<String>::new()
            .with_prompt("Enter server alias")
            .interact()?;

        // Check if alias already exists
        let config = Config::load()?;
        if config.servers.contains_key(&alias) {
            println!("{}", style("Error: Server alias already exists!").red());
            let retry = Confirm::new()
                .with_prompt("Try again?")
                .default(true)
                .interact()?;
            if !retry {
                return Ok(());
            }
            continue;
        }

        // Step 2: Get hostname/IP
        let hostname = Input::<String>::new()
            .with_prompt("Enter hostname or IP address")
            .interact()?;

        // Step 3: Get username (with current user as default)
        let current_user = std::env::var("USER").unwrap_or_else(|_| "root".to_string());
        let username = Input::<String>::new()
            .with_prompt("Enter SSH username")
            .default(current_user)
            .interact()?;

        // Step 4: Get SSH port
        let port_str = Input::<String>::new()
            .with_prompt("Enter SSH port")
            .default("22".to_string())
            .interact()?;
        let port: u16 = port_str.parse()
            .map_err(|_| anyhow::anyhow!("Invalid port number"))?;

        // Step 5: Get private key path
        println!();
        println!("{}", style("SSH Key Configuration").yellow());
        let default_key_path = format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default());
        let private_key_path = Input::<String>::new()
            .with_prompt("Enter private key path")
            .default(default_key_path)
            .interact()?;

        // Validate private key exists
        if !Path::new(&private_key_path).exists() {
            println!("{}", style("Error: Private key file not found!").red());
            let retry = Confirm::new()
                .with_prompt("Try again with different settings?")
                .default(true)
                .interact()?;
            if !retry {
                return Ok(());
            }
            continue;
        }

        // Step 6: Test connection
        println!();
        println!("{}", style("Testing SSH connection...").yellow());

        let connection_result = SshClient::connect(&hostname, port, &username, &private_key_path);

        match connection_result {
            Ok(_) => {
                println!("{}", style("✓ Connection successful!").green());
            }
            Err(e) => {
                println!("{}", style("✗ Connection failed:").red());
                println!("{}", e);

                let retry = Confirm::new()
                    .with_prompt("Would you like to edit the configuration?")
                    .default(true)
                    .interact()?;

                if retry {
                    continue; // Start over with new inputs
                } else {
                    return Ok(());
                }
            }
        }

        // Step 7: Confirm and save
        println!();
        println!("{}", style("Configuration Summary:").bold());
        println!("  Alias: {}", alias);
        println!("  Host: {}@{}:{}", username, hostname, port);
        println!("  Private Key: {}", private_key_path);

        let confirm = Confirm::new()
            .with_prompt("Save this configuration?")
            .default(true)
            .interact()?;

        if confirm {
            let mut config = Config::load()?;
            let server_config = ServerConfig {
                hostname,
                username,
                ssh_key_path: private_key_path,
                port,
                created_at: chrono::Utc::now().to_rfc3339(),
            };

            config.add_server(alias.clone(), server_config)?;
            config.save()?;

            println!();
            println!("{}", style("✓ Server configuration saved successfully!").green());
            return Ok(());
        } else {
            let retry = Confirm::new()
                .with_prompt("Configure another server?")
                .default(false)
                .interact()?;

            if retry {
                continue;
            } else {
                println!("{}", style("Configuration discarded.").yellow());
                return Ok(());
            }
        }
    }
}

async fn download_file(alias: String, remote_path: String, destination: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let server_config = config.get_server(&alias)?;

    println!("Connecting to {}@{}...", server_config.username, server_config.hostname);

    let client = SshClient::connect(
        &server_config.hostname,
        server_config.port,
        &server_config.username,
        &server_config.ssh_key_path,
    )?;

    // Check if remote file exists
    if !client.check_file_exists(&remote_path)? {
        return Err(error::DownloaderError::RemoteFileNotFound(remote_path).into());
    }

    // Extract filename from remote path
    let filename = Path::new(&remote_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // Determine destination path
    let destination_path = match &destination {
        Some(dest) => {
            // If destination is provided, check if it's a directory or file
            let dest_path = Path::new(dest);
            if dest_path.is_dir() {
                // It's a directory, append filename
                dest_path.join(&filename).to_string_lossy().to_string()
            } else {
                // It's a full file path
                dest.clone()
            }
        }
        None => {
            // No destination provided, ask user interactively
            println!();
            println!("{}", style("Download Location Options:").bold());
            println!("1. Current directory (default)");
            println!("2. Desktop");
            println!("3. Downloads");
            println!("4. Custom path");

            let choice = Input::<String>::new()
                .with_prompt("Choose download location (1-4)")
                .default("1".to_string())
                .interact()?;

            match choice.trim() {
                "1" => filename.clone(), // Current directory
                "2" => {
                    let desktop_path = format!("{}/Desktop/{}",
                        std::env::var("HOME").unwrap_or_default(), filename);
                    desktop_path
                },
                "3" => {
                    let downloads_path = format!("{}/Downloads/{}",
                        std::env::var("HOME").unwrap_or_default(), filename);
                    downloads_path
                },
                "4" => {
                    let custom_path = Input::<String>::new()
                        .with_prompt("Enter custom path")
                        .interact()?;

                    let path = Path::new(&custom_path);
                    if path.is_dir() {
                        path.join(&filename).to_string_lossy().to_string()
                    } else {
                        custom_path
                    }
                },
                _ => filename.clone(), // Default to current directory
            }
        }
    };

    // Show download info
    println!();
    println!("Remote file: {}", remote_path);
    println!("Local destination: {}", destination_path);

    // Confirm download if not specified via command line
    if destination.is_none() {
        let confirm = Confirm::new()
            .with_prompt("Proceed with download?")
            .default(true)
            .interact()?;

        if !confirm {
            println!("Download cancelled.");
            return Ok(());
        }
    }

    // Download the file
    println!("Downloading...");
    client.download_file(&remote_path, &destination_path)?;

    println!();
    println!("{}", style("✓ Download completed successfully!").green());
    println!("  Saved to: {}", destination_path);

    Ok(())
}

async fn remove_server(alias: String) -> Result<()> {
    let mut config = Config::load()?;

    if !config.servers.contains_key(&alias) {
        return Err(error::DownloaderError::ServerNotFound(alias).into());
    }

    print!("Are you sure you want to remove server '{}'? [y/N]: ", alias);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() != "y" && input.trim().to_lowercase() != "yes" {
        println!("Operation cancelled.");
        return Ok(());
    }

    config.remove_server(&alias)?;
    config.save()?;

    println!("✓ Server '{}' removed successfully!", alias);

    Ok(())
}

async fn list_servers() -> Result<()> {
    let config = Config::load()?;

    if config.servers.is_empty() {
        println!("No servers configured.");
        println!("Use 'download-from-server add server <name> <ssh-key-path>' to add a server.");
        return Ok(());
    }

    println!("Configured servers:");
    println!();

    for (name, server) in config.servers.iter() {
        println!("  Name: {}", name);
        println!("    Host: {}@{}:{}", server.username, server.hostname, server.port);
        println!("    SSH Key: {}", server.ssh_key_path);
        println!("    Added: {}", server.created_at);
        println!();
    }

    Ok(())
}