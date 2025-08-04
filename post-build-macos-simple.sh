#!/bin/bash

# Simple macOS Post-Build Script - NO dependency bundling
# Dependencies are installed separately via install-macos-deps.sh

set -e  # Exit on any error

echo "🍎 macOS Post-Build: Clean build (no dependency bundling)"

# Configuration
APP_NAME="InnoMonitor"
BUNDLE_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
BINARY_PATH="${BUNDLE_PATH}/Contents/MacOS/${APP_NAME}"

# Check if the app bundle exists
if [ ! -d "$BUNDLE_PATH" ]; then
    echo "❌ App bundle not found at: $BUNDLE_PATH"
    exit 1
fi

echo "✅ Found app bundle at: $BUNDLE_PATH"

# Verify the binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ App binary not found at: $BINARY_PATH"
    exit 1
fi

# Check what libraries the binary is linked against
echo "🔍 Checking binary dependencies..."
if command -v otool >/dev/null 2>&1; then
    echo "Library dependencies:"
    otool -L "$BINARY_PATH"
    
    echo ""
    echo "libpcap-specific dependencies:"
    otool -L "$BINARY_PATH" | grep -i pcap || echo "  No explicit libpcap dependencies found"
else
    echo "⚠️  otool not available, skipping dependency check"
fi

# Set proper permissions
echo "🔧 Setting proper permissions..."
chmod -R 755 "$BUNDLE_PATH"
chmod +x "$BINARY_PATH"

# Create a dependency requirement file
DEPS_INFO_FILE="${BUNDLE_PATH}/Contents/Resources/dependency-requirements.json"
mkdir -p "$(dirname "$DEPS_INFO_FILE")"

cat > "$DEPS_INFO_FILE" << EOF
{
    "required_dependencies": {
        "libpcap": {
            "minimum_version": "1.10.0",
            "install_command": "Run the provided install-macos-deps.sh script",
            "expected_location": "/usr/local/lib/innomonitor/libpcap.dylib",
            "fallback_locations": [
                "/opt/homebrew/lib/libpcap.dylib",
                "/usr/local/lib/libpcap.dylib",
                "/usr/lib/libpcap.dylib"
            ]
        }
    },
    "installer_script": "install-macos-deps.sh",
    "build_date": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "app_version": "1.1.4"
}
EOF

echo "✅ Created dependency requirements file: $DEPS_INFO_FILE"

echo ""
echo "🎉 macOS Post-Build Complete!"
echo ""
echo "📦 Your app bundle: $BUNDLE_PATH"
echo "📋 Dependency requirements: $DEPS_INFO_FILE"
echo ""
echo "✅ The app is ready for distribution with separate dependency installer"
echo ""
echo "🚀 Distribution Instructions:"
echo "   1. Distribute the app bundle: $BUNDLE_PATH"
echo "   2. Provide the dependency installer: scripts/install-macos-deps.sh"
echo "   3. Users run: bash install-macos-deps.sh (before first app launch)"
echo "   4. Users can then run: open InnoMonitor.app"
echo ""
