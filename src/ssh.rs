use ssh2::Session;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use anyhow::Result;
use crate::error::DownloaderError;

pub struct SshClient {
    session: Session,
}

impl SshClient {
    pub fn connect(hostname: &str, port: u16, username: &str, ssh_key_path: &str) -> Result<Self> {
        // Check if private key exists
        if !Path::new(ssh_key_path).exists() {
            return Err(DownloaderError::SshKeyNotFound(ssh_key_path.to_string()).into());
        }

        // Connect to the SSH server
        let tcp = TcpStream::connect(format!("{}:{}", hostname, port))
            .map_err(|e| DownloaderError::ConnectionFailed(
                format!("Failed to connect to {}:{} - {}", hostname, port, e)
            ))?;

        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| DownloaderError::ConnectionFailed(
                format!("SSH handshake failed: {}", e)
            ))?;

        // Try to authenticate with the private key
        session.userauth_pubkey_file(username, None, Path::new(ssh_key_path), None)
            .map_err(|e| DownloaderError::ConnectionFailed(
                format!("SSH authentication failed: {}", e)
            ))?;

        Ok(SshClient { session })
    }

    pub fn download_file(&self, remote_path: &str, local_path: &str) -> Result<()> {
        let (mut remote_file, _) = self.session.scp_recv(Path::new(remote_path))
            .map_err(|e| {
                if e.message().contains("File not found") {
                    DownloaderError::RemoteFileNotFound(remote_path.to_string())
                } else {
                    DownloaderError::SshError(e)
                }
            })?;

        let mut local_file = File::create(local_path)
            .map_err(|e| DownloaderError::IoError(e))?;

        let mut buffer = [0; 8192];
        loop {
            let bytes_read = remote_file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            local_file.write_all(&buffer[..bytes_read])?;
        }

        Ok(())
    }

    pub fn check_file_exists(&self, path: &str) -> Result<bool> {
        let sftp = self.session.sftp()
            .map_err(DownloaderError::SshError)?;

        match sftp.stat(Path::new(path)) {
            Ok(_) => Ok(true),
            Err(e) => {
                // Check if the error is file not found
                if e.message().contains("No such file") || e.message().contains("does not exist") {
                    Ok(false)
                } else {
                    Err(DownloaderError::SshError(e).into())
                }
            }
        }
    }
}

pub fn extract_hostname_from_public_key(public_key_path: &str) -> Result<String> {
    let mut file = File::open(public_key_path)
        .map_err(|_| DownloaderError::SshKeyNotFound(public_key_path.to_string()))?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Parse SSH public key format: "algorithm key-data comment"
    let parts: Vec<&str> = content.trim().split_whitespace().collect();

    if parts.len() < 3 {
        return Err(DownloaderError::HostnameExtractionFailed.into());
    }

    let comment = parts[2];

    // Extract hostname from comment (format: user@hostname)
    if let Some(at_pos) = comment.rfind('@') {
        let hostname = comment[at_pos + 1..].to_string();
        if !hostname.is_empty() {
            return Ok(hostname);
        }
    }

    Err(DownloaderError::HostnameExtractionFailed.into())
}

pub fn get_private_key_path(public_key_path: &str) -> Result<String> {
    // Convert public key path to private key path
    if public_key_path.ends_with(".pub") {
        Ok(public_key_path.trim_end_matches(".pub").to_string())
    } else {
        // If it doesn't end with .pub, assume it's already a private key path
        Ok(public_key_path.to_string())
    }
}

pub fn validate_ssh_keys(public_key_path: &str) -> Result<String> {
    // Check if public key exists
    if !Path::new(public_key_path).exists() {
        return Err(DownloaderError::SshKeyNotFound(public_key_path.to_string()).into());
    }

    // Check if private key exists
    let private_key_path = get_private_key_path(public_key_path)?;
    if !Path::new(&private_key_path).exists() {
        return Err(DownloaderError::SshKeyNotFound(private_key_path).into());
    }

    // Try to extract hostname to validate key format
    extract_hostname_from_public_key(public_key_path)?;

    Ok(private_key_path)
}