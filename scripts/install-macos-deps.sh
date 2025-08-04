#!/bin/bash

# macOS Dependency Installer for InnoMonitor
# This script installs libpcap to a standardized location

set -e  # Exit on any error

echo "🍎 InnoMonitor macOS Dependency Installer"
echo "=========================================="

# Configuration
DEPS_DIR="/usr/local/lib/innomonitor"
LIBPCAP_VERSION="1.10.5"  # Target a specific version for consistency
INSTALL_LOG="/tmp/innomonitor-deps-install.log"

# Check if running as root/sudo
if [[ $EUID -eq 0 ]]; then
    echo "❌ This script should NOT be run as root/sudo for security reasons"
    echo "   Please run without sudo - the script will ask for password when needed"
    exit 1
fi

echo "📍 Target installation directory: $DEPS_DIR"
echo "📋 Installation log: $INSTALL_LOG"
echo ""

# Function to log messages
log_message() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$INSTALL_LOG"
}

# Check if Homebrew is installed
check_homebrew() {
    if ! command -v brew &> /dev/null; then
        echo "❌ Homebrew is not installed."
        echo "   InnoMonitor requires Homebrew to manage libpcap dependency."
        echo ""
        echo "🔧 To install Homebrew, run:"
        echo '   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
        echo ""
        echo "   After installing Homebrew, run this script again."
        exit 1
    fi
    
    log_message "✅ Homebrew found at: $(which brew)"
}

# Install libpcap via Homebrew
install_libpcap() {
    log_message "🔄 Installing/updating libpcap via Homebrew..."
    
    if brew list libpcap &> /dev/null; then
        log_message "📦 libpcap already installed, checking version..."
        brew upgrade libpcap || log_message "libpcap is already up to date"
    else
        log_message "📦 Installing libpcap..."
        brew install libpcap
    fi
    
    # Verify installation
    if brew list libpcap &> /dev/null; then
        INSTALLED_VERSION=$(brew list --versions libpcap | awk '{print $2}')
        log_message "✅ libpcap $INSTALLED_VERSION installed successfully"
    else
        log_message "❌ Failed to install libpcap"
        exit 1
    fi
}

# Create symlinks to standardized location
create_standardized_links() {
    log_message "🔗 Creating standardized library links..."
    
    # Create target directory
    sudo mkdir -p "$DEPS_DIR"
    sudo chown $(whoami):staff "$DEPS_DIR"
    
    # Get Homebrew prefix (different for Intel vs Apple Silicon)
    BREW_PREFIX=$(brew --prefix)
    LIBPCAP_LIB="$BREW_PREFIX/lib"
    LIBPCAP_INCLUDE="$BREW_PREFIX/include"
    
    log_message "📁 Homebrew prefix: $BREW_PREFIX"
    
    # Create symlinks to the standardized location
    if [[ -f "$LIBPCAP_LIB/libpcap.dylib" ]]; then
        ln -sf "$LIBPCAP_LIB/libpcap.dylib" "$DEPS_DIR/libpcap.dylib"
        log_message "✅ Created symlink: $DEPS_DIR/libpcap.dylib -> $LIBPCAP_LIB/libpcap.dylib"
    else
        log_message "❌ libpcap.dylib not found at $LIBPCAP_LIB"
        exit 1
    fi
    
    # Also create symlinks for versioned libraries if they exist
    for lib_file in "$LIBPCAP_LIB"/libpcap.*.dylib; do
        if [[ -f "$lib_file" ]]; then
            lib_name=$(basename "$lib_file")
            ln -sf "$lib_file" "$DEPS_DIR/$lib_name"
            log_message "✅ Created symlink: $DEPS_DIR/$lib_name -> $lib_file"
        fi
    done
    
    # Create info file for the app to verify dependencies
    cat > "$DEPS_DIR/dependency-info.json" << EOF
{
    "installed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "libpcap_version": "$(pkg-config --modversion libpcap 2>/dev/null || echo 'unknown')",
    "homebrew_prefix": "$BREW_PREFIX",
    "libpcap_path": "$LIBPCAP_LIB/libpcap.dylib",
    "installer_version": "1.0.0"
}
EOF
    
    log_message "✅ Created dependency info file: $DEPS_DIR/dependency-info.json"
}

# Verify installation
verify_installation() {
    log_message "🔍 Verifying installation..."
    
    if [[ -f "$DEPS_DIR/libpcap.dylib" ]]; then
        log_message "✅ libpcap available at: $DEPS_DIR/libpcap.dylib"
        
        # Test library loading
        if otool -L "$DEPS_DIR/libpcap.dylib" &> /dev/null; then
            log_message "✅ Library structure is valid"
        else
            log_message "⚠️  Warning: Library structure check failed"
        fi
        
        # Test pkg-config
        if pkg-config --exists libpcap; then
            log_message "✅ pkg-config can find libpcap"
            log_message "   Version: $(pkg-config --modversion libpcap)"
            log_message "   Libs: $(pkg-config --libs libpcap)"
        else
            log_message "⚠️  Warning: pkg-config cannot find libpcap"
        fi
        
    else
        log_message "❌ Installation verification failed"
        exit 1
    fi
}

# Main installation process
main() {
    log_message "🚀 Starting InnoMonitor dependency installation..."
    
    check_homebrew
    install_libpcap
    create_standardized_links
    verify_installation
    
    echo ""
    echo "🎉 InnoMonitor dependencies installed successfully!"
    echo ""
    echo "📁 Dependencies installed to: $DEPS_DIR"
    echo "📋 Installation log: $INSTALL_LOG"
    echo ""
    echo "✅ You can now run InnoMonitor - it will automatically find the dependencies."
    echo ""
    echo "🔧 To uninstall dependencies later, run:"
    echo "   sudo rm -rf $DEPS_DIR"
    echo "   brew uninstall libpcap  # (optional - removes Homebrew package)"
    echo ""
}

# Run main function
main "$@"
