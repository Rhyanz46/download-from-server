#!/bin/bash

# =============================================================================
# Download from Server - Installer Script
# =============================================================================
# This script installs the download-from-server CLI tool on macOS and Linux
# Author: Your Name
# Version: 1.0.0
# =============================================================================

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_NAME="download-from-server"
REPO_URL="https://github.com/yourusername/downloader"
BINARY_NAME="download-from-server"

# Functions
print_header() {
    echo -e "${CYAN}============================================${NC}"
    echo -e "${CYAN}📥 Download from Server Installer${NC}"
    echo -e "${CYAN}============================================${NC}"
    echo
}

print_step() {
    echo -e "${BLUE}➜ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

check_system() {
    print_step "Checking system requirements..."

    # Check OS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macOS"
        if [[ $(uname -m) == "arm64" ]]; then
            ARCH="arm64"
        else
            ARCH="x86_64"
        fi
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="Linux"
        ARCH=$(uname -m)
        if [[ "$ARCH" == "x86_64" ]]; then
            ARCH="x86_64"
        elif [[ "$ARCH" == "aarch64" ]]; then
            ARCH="arm64"
        else
            print_error "Unsupported architecture: $ARCH"
            exit 1
        fi
    else
        print_error "Unsupported operating system: $OSTYPE"
        exit 1
    fi

    print_success "Detected $OS ($ARCH)"

    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        print_warning "Rust is not installed"
        echo -e "${YELLOW}Would you like to install Rust? (y/N)${NC}"
        read -r install_rust
        if [[ $install_rust =~ ^[Yy]$ ]]; then
            print_step "Installing Rust..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "$HOME/.cargo/env"
            print_success "Rust installed successfully"
        else
            print_error "Rust is required to build this application"
            exit 1
        fi
    else
        rust_version=$(rustc --version)
        print_success "Rust found: $rust_version"
    fi
}

build_binary() {
    print_step "Building $SCRIPT_NAME..."

    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]]; then
        print_error "Cargo.toml not found. Please run this script from the project root directory."
        exit 1
    fi

    # Build the project
    cargo build --release

    if [[ $? -eq 0 ]]; then
        print_success "Binary built successfully"
    else
        print_error "Failed to build binary"
        exit 1
    fi
}

get_install_type() {
    echo -e "${CYAN}Installation Options:${NC}"
    echo "1) Install for current user only (recommended)"
    echo "2) Install for all users (requires sudo)"
    echo

    while true; do
        read -p "Choose installation type [1-2]: " choice
        case $choice in
            1)
                INSTALL_TYPE="user"
                INSTALL_DIR="$HOME/.local/bin"
                break
                ;;
            2)
                INSTALL_TYPE="system"
                INSTALL_DIR="/usr/local/bin"
                break
                ;;
            *)
                print_warning "Please enter 1 or 2"
                ;;
        esac
    done
}

create_directories() {
    if [[ "$INSTALL_TYPE" == "user" ]]; then
        mkdir -p "$HOME/.local/bin"

        # Add to PATH if not already there
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc" 2>/dev/null || true
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc" 2>/dev/null || true
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.profile" 2>/dev/null || true
            print_warning "Added ~/.local/bin to PATH. You may need to restart your shell or run:"
            echo -e "${YELLOW}  export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
        fi
    else
        # System installation - check if we have sudo
        if ! command -v sudo &> /dev/null; then
            print_error "sudo is required for system-wide installation"
            exit 1
        fi
        sudo mkdir -p "$INSTALL_DIR"
    fi
}

install_binary() {
    print_step "Installing binary to $INSTALL_DIR..."

    BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"

    if [[ "$INSTALL_TYPE" == "system" ]]; then
        sudo cp "target/release/$BINARY_NAME" "$BINARY_PATH"
        sudo chmod +x "$BINARY_PATH"
    else
        cp "target/release/$BINARY_NAME" "$BINARY_PATH"
        chmod +x "$BINARY_PATH"
    fi

    print_success "Binary installed to $BINARY_PATH"
}

create_symlink() {
    # Create a shorter command alias
    SYMLINK_NAME="dfs"
    SYMLINK_PATH="$INSTALL_DIR/$SYMLINK_NAME"

    if [[ "$INSTALL_TYPE" == "system" ]]; then
        sudo ln -sf "$BINARY_PATH" "$SYMLINK_PATH" 2>/dev/null || true
    else
        ln -sf "$BINARY_PATH" "$SYMLINK_PATH" 2>/dev/null || true
    fi

    if [[ -L "$SYMLINK_PATH" ]]; then
        print_success "Created symlink: $SYMLINK_NAME -> $BINARY_NAME"
    fi
}

verify_installation() {
    print_step "Verifying installation..."

    # Refresh shell
    if [[ "$INSTALL_TYPE" == "user" ]]; then
        export PATH="$HOME/.local/bin:$PATH"
    fi

    # Check if command is available
    if command -v "$BINARY_NAME" &> /dev/null; then
        VERSION=$($BINARY_NAME --version 2>/dev/null | head -1)
        print_success "Installation verified! $VERSION"
        echo
        echo -e "${GREEN}You can now use the following commands:${NC}"
        echo -e "  ${CYAN}$BINARY_NAME${NC} - Full command"
        if [[ -L "$SYMLINK_PATH" ]]; then
            echo -e "  ${CYAN}$SYMLINK_NAME${NC} - Short alias"
        fi
        echo
    else
        print_warning "Binary not found in PATH. You may need to:"
        if [[ "$INSTALL_TYPE" == "user" ]]; then
            echo -e "  ${YELLOW}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
        else
            echo -e "  ${YELLOW}Restart your shell${NC}"
        fi
    fi
}

show_post_install() {
    echo
    echo -e "${CYAN}============================================${NC}"
    echo -e "${CYAN}🎉 Installation Complete!${NC}"
    echo -e "${CYAN}============================================${NC}"
    echo
    echo -e "${GREEN}Quick Start:${NC}"
    echo "1. Add your first server:"
    echo -e "   ${YELLOW}$BINARY_NAME add${NC}"
    echo
    echo "2. Download a file:"
    echo -e "   ${YELLOW}$BINARY_NAME download <server-alias> <remote-path>${NC}"
    echo
    echo "3. List configured servers:"
    echo -e "   ${YELLOW}$BINARY_NAME list${NC}"
    echo
    echo -e "${CYAN}For more help:${NC}"
    echo -e "   ${YELLOW}$BINARY_NAME --help${NC}"
    echo
    echo -e "${CYAN}Documentation:${NC}"
    echo -e "   ${YELLOW}$REPO_URL${NC}"
    echo
    echo -e "${PURPLE}⭐ Enjoy using $SCRIPT_NAME!${NC}"
}

cleanup_on_exit() {
    if [[ $? -ne 0 ]]; then
        print_error "Installation failed"
        exit 1
    fi
}

# Set up error handling
trap cleanup_on_exit EXIT

# Main installation flow
main() {
    print_header

    # Ask for installation type first
    get_install_type

    check_system
    build_binary
    create_directories
    install_binary
    create_symlink
    verify_installation
    show_post_install
}

# Check for help flag
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    echo "$SCRIPT_NAME Installer Script"
    echo
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  --user         Install for current user only"
    echo "  --system       Install for all users (requires sudo)"
    echo
    echo "This script installs $SCRIPT_NAME on macOS and Linux systems."
    echo "It will build the binary from source and install it to the appropriate directory."
    exit 0
fi

# Check for command line flags
if [[ "$1" == "--user" ]]; then
    INSTALL_TYPE="user"
    INSTALL_DIR="$HOME/.local/bin"
elif [[ "$1" == "--system" ]]; then
    INSTALL_TYPE="system"
    INSTALL_DIR="/usr/local/bin"
fi

# Run main installation
main "$@"