#!/bin/bash
# install.sh - Easy installer script for smolcase

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# GitHub repository
REPO="simplysabir/smolcase"
BINARY_NAME="smolcase"

# Print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    local os
    local arch
    
    # Detect OS
    case "$(uname -s)" in
        Linux*)     os="linux";;
        Darwin*)    os="macos";;
        CYGWIN*|MINGW*|MSYS*) os="windows";;
        *)          print_error "Unsupported OS: $(uname -s)"; exit 1;;
    esac
    
    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64";;
        aarch64|arm64) arch="aarch64";;
        armv7l) arch="armv7";;
        *) print_error "Unsupported architecture: $(uname -m)"; exit 1;;
    esac
    
    echo "${os}-${arch}"
}

# Get latest release version
get_latest_version() {
    local version
    version=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    
    if [ -z "$version" ]; then
        print_error "Failed to get latest version"
        exit 1
    fi
    
    echo "$version"
}

# Download and install binary
install_binary() {
    local platform="$1"
    local version="$2"
    local install_dir="${3:-/usr/local/bin}"
    
    # Construct download URL
    local filename
    local url
    
    if [[ "$platform" == *"windows"* ]]; then
        filename="${BINARY_NAME}-${platform}.exe.zip"
        url="https://github.com/${REPO}/releases/download/${version}/${filename}"
    else
        filename="${BINARY_NAME}-${platform}.tar.gz"
        url="https://github.com/${REPO}/releases/download/${version}/${filename}"
    fi
    
    print_info "Downloading ${BINARY_NAME} ${version} for ${platform}..."
    print_info "URL: ${url}"
    
    # Create temp directory
    local temp_dir
    temp_dir=$(mktemp -d)
    
    # Download
    if ! curl -L -o "${temp_dir}/${filename}" "$url"; then
        print_error "Failed to download ${filename}"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Extract
    cd "$temp_dir"
    if [[ "$platform" == *"windows"* ]]; then
        unzip -q "$filename"
        binary_path="${BINARY_NAME}.exe"
    else
        tar -xzf "$filename"
        binary_path="${BINARY_NAME}"
    fi
    
    # Check if binary exists
    if [ ! -f "$binary_path" ]; then
        print_error "Binary not found in archive"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Make executable
    chmod +x "$binary_path"
    
    # Install
    if [ -w "$install_dir" ]; then
        mv "$binary_path" "${install_dir}/"
        print_success "Installed ${BINARY_NAME} to ${install_dir}/"
    else
        print_info "Installing to ${install_dir}/ (requires sudo)"
        sudo mv "$binary_path" "${install_dir}/"
        print_success "Installed ${BINARY_NAME} to ${install_dir}/"
    fi
    
    # Cleanup
    rm -rf "$temp_dir"
}

# Verify installation
verify_installation() {
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local version
        version=$("$BINARY_NAME" --version 2>/dev/null || echo "unknown")
        print_success "${BINARY_NAME} installed successfully!"
        print_info "Version: $version"
        print_info "Run '${BINARY_NAME} --help' to get started"
    else
        print_warning "${BINARY_NAME} installed but not found in PATH"
        print_info "You may need to add the installation directory to your PATH"
    fi
}

# Main installation function
main() {
    print_info "Installing ${BINARY_NAME}..."
    
    # Check for curl
    if ! command -v curl >/dev/null 2>&1; then
        print_error "curl is required but not installed"
        exit 1
    fi
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    print_info "Detected platform: $platform"
    
    # Get latest version
    local version
    version=$(get_latest_version)
    print_info "Latest version: $version"
    
    # Custom install directory
    local install_dir="/usr/local/bin"
    if [ -n "$INSTALL_DIR" ]; then
        install_dir="$INSTALL_DIR"
    fi
    
    # Install
    install_binary "$platform" "$version" "$install_dir"
    
    # Verify
    verify_installation
    
    print_success "Installation complete!"
    echo
    print_info "Quick start:"
    echo "  mkdir my-secrets && cd my-secrets"
    echo "  ${BINARY_NAME} init --name 'My Project' --git"
    echo "  ${BINARY_NAME} configure"
}

# Handle command line arguments
case "${1:-}" in
    -h|--help)
        echo "Usage: $0 [options]"
        echo
        echo "Options:"
        echo "  -h, --help    Show this help message"
        echo
        echo "Environment variables:"
        echo "  INSTALL_DIR   Custom installation directory (default: /usr/local/bin)"
        echo
        echo "Examples:"
        echo "  $0                    # Install to /usr/local/bin"
        echo "  INSTALL_DIR=~/.local/bin $0  # Install to ~/.local/bin"
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac