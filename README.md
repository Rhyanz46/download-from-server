# 📥 Download from Server

<div align="center">

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)]()

*A simple and secure CLI tool for downloading files from remote servers using SSH authentication*

</div>

## ✨ Features

- 🔐 **Secure SSH Authentication** - Uses your existing SSH keys for secure connections
- ⚡ **Fast Downloads** - Efficient file transfer with progress feedback
- 📝 **Interactive Configuration** - Step-by-step server setup with connection testing
- 🎯 **Smart Download Locations** - Choose where to save files with intuitive prompts
- 🗂️ **Multiple Server Management** - Store and manage multiple server configurations
- 🎨 **Beautiful Terminal UI** - Colored output and user-friendly interface
- 💾 **Persistent Configuration** - Server settings saved securely in JSON format

## 🚀 Quick Start

### Installation

#### Method 1: Easy Install Script (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/downloader.git
cd downloader

# Run the installer
./install.sh
```

The installer will:
- Check system requirements and install Rust if needed
- Build the binary from source
- Ask for installation type:
  - **User-only** (recommended) - Installs to `~/.local/bin`
  - **System-wide** - Installs to `/usr/local/bin` (requires sudo)
- Create a short alias `dfs` for convenience
- Add to PATH if needed

#### Method 2: Manual Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/downloader.git
cd downloader

# Build the project
cargo build --release

# Install system-wide (optional)
cargo install --path .
```

#### Uninstallation

To remove the tool completely:

```bash
# From the project directory
./uninstall.sh
```

This will remove the binary, symlink, and optionally configuration files.

### Basic Usage

After installation, you can use either the full command or the short alias:

1. **Add a server configuration:**
   ```bash
   download-from-server add
   # or using the alias:
   dfs add
   ```

2. **Download a file:**
   ```bash
   download-from-server download myserver /path/to/remote/file.txt
   # or using the alias:
   dfs download myserver /path/to/remote/file.txt
   ```

3. **List all servers:**
   ```bash
   download-from-server list
   # or using the alias:
   dfs list
   ```

## 📖 Detailed Usage

### 🔧 Adding a Server

The interactive add command guides you through server configuration:

```bash
download-from-server add
```

**Configuration Steps:**
1. **Server Alias** - A memorable name for your server
2. **Hostname/IP** - Server address (e.g., `192.168.1.100` or `example.com`)
3. **SSH Username** - Your SSH username (defaults to current user)
4. **SSH Port** - Port number (defaults to 22)
5. **Private Key Path** - Path to your SSH private key (defaults to `~/.ssh/id_rsa`)
6. **Connection Test** - Automatic verification of SSH connection
7. **Confirmation** - Review and save your configuration

### 📥 Downloading Files

#### Basic Download
```bash
download-from-server download myserver /home/user/documents/report.pdf
```

#### Download with Custom Location
```bash
# Download to Desktop
download-from-server download myserver /home/user/data.csv -d ~/Desktop

# Download to specific directory
download-from-server download myserver /home/user/app.tar.gz -d /path/to/downloads

# Download with custom filename
download-from-server download myserver /home/user/backup.zip -d ~/my-backup.zip
```

#### Interactive Download Location
When no destination is specified, you'll see:

```
Download Location Options:
1. Current directory (default)
2. Desktop
3. Downloads
4. Custom path

Choose download location (1-4) [1]:
```

### 📋 Managing Servers

#### List All Configured Servers
```bash
download-from-server list
```

Output:
```
Configured servers:

  Name: production
    Host: deploy@prod.example.com:22
    SSH Key: /home/user/.ssh/id_rsa
    Added: 2025-12-04T10:00:00Z

  Name: staging
    Host: user@staging.example.com:22
    SSH Key: /home/user/.ssh/staging_key
    Added: 2025-12-04T09:30:00Z
```

#### Remove a Server
```bash
download-from-server remove myserver
```

You'll be asked to confirm before removal.

## ⚙️ Installation Details

### Install Script Features

The `install.sh` script provides a user-friendly installation experience:

- **System Detection** - Automatically detects macOS/Linux and CPU architecture
- **Rust Management** - Installs Rust if not present
- **Installation Types**:
  - **User-only** (`~/.local/bin`): No sudo required, safer choice
  - **System-wide** (`/usr/local/bin`): Available to all users, requires sudo
- **PATH Configuration** - Automatically adds installation directory to PATH
- **Symlink Creation** - Creates `dfs` alias for quick access
- **Verification** - Confirms installation was successful

### Command Line Options

```bash
# Interactive installation
./install.sh

# Non-interactive options
./install.sh --user    # Install for current user only
./install.sh --system  # Install system-wide (requires sudo)
```

## 🔧 Configuration

Server configurations are stored in `~/.downloader-from-server/config.json`:

```json
{
  "servers": {
    "production": {
      "hostname": "prod.example.com",
      "username": "deploy",
      "ssh_key_path": "/home/user/.ssh/id_rsa",
      "port": 22,
      "created_at": "2025-12-04T10:00:00Z"
    },
    "staging": {
      "hostname": "staging.example.com",
      "username": "user",
      "ssh_key_path": "/home/user/.ssh/staging_key",
      "port": 2222,
      "created_at": "2025-12-04T09:30:00Z"
    }
  }
}
```

## 🛡️ Security

- **SSH Key Authentication** - Uses your existing SSH keys for secure connections
- **No Password Storage** - Never stores passwords in configuration files
- **Local Key Usage** - SSH keys remain on your local machine
- **Connection Verification** - Tests connections before saving configurations

## 🔧 Requirements

- **SSH Access** - Passwordless SSH access to your target servers
- **SSH Keys** - Private key file (e.g., `~/.ssh/id_rsa`)
- **Rust Toolchain** - Rust 1.70 or newer (for building from source)

### Setting up SSH Keys

If you don't have SSH keys set up:

```bash
# Generate new SSH key pair
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"

# Copy public key to remote server
ssh-copy-id user@your-server.com
```

## 🔍 Troubleshooting

### Common Issues

1. **Connection Failed**
   - Check if SSH key is correctly configured
   - Verify server hostname and port
   - Ensure passwordless SSH access works

2. **File Not Found**
   - Verify the remote file path is correct
   - Check file permissions on the remote server

3. **Permission Denied**
   - Ensure your SSH key has proper permissions (600)
   - Check remote server user permissions

### Debug Mode

For detailed error information, you can run with debug logging:

```bash
RUST_LOG=debug download-from-server download myserver /path/to/file
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/downloader.git
cd downloader

# Install dependencies
cargo build

# Run tests
cargo test

# Run in development mode
cargo run -- download myserver /path/to/file
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [ssh2](https://github.com/alexcrichton/ssh2-rs) for SSH functionality
- UI powered by [dialoguer](https://github.com/mitsuhiko/dialoguer)
- CLI parsing by [clap](https://clap.rs/)

## 📞 Support

If you encounter any issues or have questions:

- 🐛 [Report a bug](https://github.com/yourusername/downloader/issues)
- 💡 [Request a feature](https://github.com/yourusername/downloader/issues/new?template=feature_request.md)
- 📧 Email: your.email@example.com

---

<div align="center">

Made with ❤️ by [Your Name]

[⭐ Star this repo](https://github.com/yourusername/downloader) • [🐛 Report issues](https://github.com/yourusername/downloader/issues)

</div># download-from-server
