#!/bin/bash

# =============================================================================
# Download from Server - Uninstaller Script
# =============================================================================
# This script removes the download-from-server CLI tool from macOS and Linux
# Author: Your Name
# Version: 1.0.0
# =============================================================================

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_NAME="download-from-server"
BINARY_NAME="download-from-server"
SYMLINK_NAME="dfs"

# Functions
print_header() {
    echo -e "${CYAN}============================================${NC}"
    echo -e "${CYAN}🗑️  Download from Server Uninstaller${NC}"
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

find_installation() {
    print_step "Finding installation..."

    # Possible installation locations
    LOCATIONS=(
        "$HOME/.local/bin"
        "/usr/local/bin"
        "/usr/bin"
        "/opt/homebrew/bin"
        "$HOME/bin"
    )

    FOUND_LOCATIONS=()

    for location in "${LOCATIONS[@]}"; do
        if [[ -f "$location/$BINARY_NAME" ]]; then
            FOUND_LOCATIONS+=("$location")
            print_success "Found: $location/$BINARY_NAME"
        fi

        if [[ -L "$location/$SYMLINK_NAME" ]]; then
            FOUND_LOCATIONS+=("$location")
            print_success "Found symlink: $location/$SYMLINK_NAME"
        fi
    done

    if [[ ${#FOUND_LOCATIONS[@]} -eq 0 ]]; then
        print_warning "No installation found"
        echo -e "${YELLOW}The $SCRIPT_NAME binary was not found in standard locations.${NC}"
        exit 0
    fi
}

confirm_uninstall() {
    echo
    echo -e "${RED}This will remove $SCRIPT_NAME from the following locations:${NC}"
    for location in "${FOUND_LOCATIONS[@]}"; do
        echo -e "${YELLOW}  - $location${NC}"
    done
    echo

    read -p "Are you sure you want to continue? [y/N] " confirm
    if [[ ! $confirm =~ ^[Yy]$ ]]; then
        echo "Uninstallation cancelled."
        exit 0
    fi
}

remove_files() {
    print_step "Removing files..."

    for location in "${FOUND_LOCATIONS[@]}"; do
        # Remove binary
        if [[ -f "$location/$BINARY_NAME" ]]; then
            if [[ "$location" == "/usr/local/bin" ]] || [[ "$location" == "/usr/bin" ]] || [[ "$location" == "/opt/homebrew/bin" ]]; then
                # System directory - use sudo
                if command -v sudo &> /dev/null; then
                    sudo rm -f "$location/$BINARY_NAME"
                    print_success "Removed: $location/$BINARY_NAME"
                else
                    print_warning "Cannot remove system file without sudo: $location/$BINARY_NAME"
                fi
            else
                # User directory - no sudo needed
                rm -f "$location/$BINARY_NAME"
                print_success "Removed: $location/$BINARY_NAME"
            fi
        fi

        # Remove symlink
        if [[ -L "$location/$SYMLINK_NAME" ]]; then
            if [[ "$location" == "/usr/local/bin" ]] || [[ "$location" == "/usr/bin" ]] || [[ "$location" == "/opt/homebrew/bin" ]]; then
                # System directory - use sudo
                if command -v sudo &> /dev/null; then
                    sudo rm -f "$location/$SYMLINK_NAME"
                    print_success "Removed symlink: $location/$SYMLINK_NAME"
                else
                    print_warning "Cannot remove system symlink without sudo: $location/$SYMLINK_NAME"
                fi
            else
                # User directory - no sudo needed
                rm -f "$location/$SYMLINK_NAME"
                print_success "Removed symlink: $location/$SYMLINK_NAME"
            fi
        fi
    done
}

remove_config() {
    print_step "Checking for configuration files..."

    CONFIG_DIR="$HOME/.downloader-from-server"
    CONFIG_FILE="$CONFIG_DIR/config.json"

    if [[ -f "$CONFIG_FILE" ]]; then
        echo
        echo -e "${YELLOW}Configuration found at: $CONFIG_FILE${NC}"
        read -p "Remove configuration files? [y/N] " remove_config
        if [[ $remove_config =~ ^[Yy]$ ]]; then
            rm -rf "$CONFIG_DIR"
            print_success "Removed configuration directory: $CONFIG_DIR"
        else
            print_warning "Configuration files kept"
        fi
    else
        print_success "No configuration files found"
    fi
}

cleanup_shell() {
    print_step "Cleaning up shell configuration..."

    # Note: We don't automatically remove PATH entries as it might break other tools
    echo -e "${YELLOW}Note: PATH entries in shell configuration files were not automatically removed.${NC}"
    echo -e "${YELLOW}You may want to manually remove any references to ~/.local/bin if it's no longer needed.${NC}"
}

show_completion() {
    echo
    echo -e "${CYAN}============================================${NC}"
    echo -e "${GREEN}🎉 Uninstallation Complete!${NC}"
    echo -e "${CYAN}============================================${NC}"
    echo
    echo -e "${GREEN}$SCRIPT_NAME has been removed from your system.${NC}"
    echo
    echo -e "${CYAN}Thank you for using $SCRIPT_NAME!${NC}"
    echo -e "${CYAN}If you enjoyed it, consider starring the repository:${NC}"
    echo -e "${YELLOW}https://github.com/yourusername/downloader${NC}"
    echo
}

# Main uninstallation flow
main() {
    print_header
    find_installation
    confirm_uninstall
    remove_files
    remove_config
    cleanup_shell
    show_completion
}

# Check for help flag
if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]]; then
    echo "$SCRIPT_NAME Uninstaller Script"
    echo
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  --force        Skip confirmation prompts"
    echo
    echo "This script removes $SCRIPT_NAME from your system."
    echo "It will remove the binary, symlink, and optionally configuration files."
    exit 0
fi

# Check for force flag
if [[ "$1" == "--force" ]]; then
    # Override confirmation
    confirm_uninstall() {
        echo "Force mode: Skipping confirmation"
    }
fi

# Run main uninstallation
main "$@"