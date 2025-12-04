use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloaderError {
    #[error("Server '{0}' not found in configuration")]
    ServerNotFound(String),

    #[error("SSH key file not found: {0}")]
    SshKeyNotFound(String),

    #[error("Failed to connect to server: {0}")]
    ConnectionFailed(String),

    #[error("File not found on remote server: {0}")]
    RemoteFileNotFound(String),

    #[error("Permission denied accessing: {0}")]
    PermissionDenied(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid SSH key format: {0}")]
    InvalidSshKeyFormat(String),

    #[error("Could not extract hostname from SSH key comment")]
    HostnameExtractionFailed,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("SSH error: {0}")]
    SshError(#[from] ssh2::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Home directory not found")]
    HomeDirectoryNotFound,
}