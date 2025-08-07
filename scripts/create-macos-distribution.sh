#!/bin/bash

# Create distribution package for macOS
# This script packages the app and dependency installer together

set -e

echo "📦 Creating InnoMonitor macOS Distribution Package"
echo "================================================="

# Configuration
APP_NAME="InnoMonitor"
VERSION="1.1.601"
DIST_DIR="dist/macos"
BUNDLE_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
INSTALLER_SCRIPT="scripts/install-macos-deps.sh"
LAUNCHER_SCRIPT="scripts/launch-innomonitor.sh"

# Check prerequisites
if [ ! -d "$BUNDLE_PATH" ]; then
    echo "❌ App bundle not found. Please run 'npm run tauri:build:macos' first"
    exit 1
fi

if [ ! -f "$INSTALLER_SCRIPT" ]; then
    echo "❌ Dependency installer script not found at: $INSTALLER_SCRIPT"
    exit 1
fi

if [ ! -f "$LAUNCHER_SCRIPT" ]; then
    echo "❌ Launcher script not found at: $LAUNCHER_SCRIPT"
    exit 1
fi

# Create distribution directory
echo "📁 Creating distribution directory..."
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Copy app bundle
echo "📦 Copying app bundle..."
cp -R "$BUNDLE_PATH" "$DIST_DIR/"

# Copy installer script
echo "📋 Copying dependency installer..."
cp "$INSTALLER_SCRIPT" "$DIST_DIR/"
chmod +x "$DIST_DIR/install-macos-deps.sh"

# Copy launcher script
echo "🚀 Copying launcher script..."
cp "$LAUNCHER_SCRIPT" "$DIST_DIR/"
chmod +x "$DIST_DIR/launch-innomonitor.sh"

# Create README for users
cat > "$DIST_DIR/README.txt" << EOF
InnoMonitor v${VERSION} for macOS
================================

IMPORTANT: Setup Instructions (Run in order):

1. Install Dependencies (REQUIRED):
   bash install-macos-deps.sh

2. Launch InnoMonitor (IMPORTANT - Use the launcher):
   bash launch-innomonitor.sh

   OR manually add to System Settings:
   - System Settings → Privacy & Security → Full Disk Access
   - Add InnoMonitor.app
   - Then: open InnoMonitor.app

DO NOT run InnoMonitor directly with 'sudo' - this causes security errors.

Installation Requirements:
- macOS 10.13 or later
- Homebrew (will be checked during dependency installation)
- Admin access (for dependency installation only)

Troubleshooting:
- If app won't open: Use launch-innomonitor.sh script
- If dependencies fail: Install Homebrew first, then retry
- For setugid errors: Never use 'sudo' directly with the app
- For errors: Check /tmp/innomonitor-deps-install.log

Files in this package:
- InnoMonitor.app: The main application
- install-macos-deps.sh: Installs required dependencies
- launch-innomonitor.sh: Properly launches the app with permissions
- uninstall-dependencies.sh: Removes installed dependencies

Uninstallation:
- Remove app: Delete InnoMonitor.app
- Remove dependencies: bash uninstall-dependencies.sh

For support: https://github.com/omnimuh730/rs-fairsight
EOF

# Create uninstaller script
cat > "$DIST_DIR/uninstall-dependencies.sh" << 'EOF'
#!/bin/bash

echo "🗑️  InnoMonitor Dependency Uninstaller"
echo "======================================"

DEPS_DIR="/usr/local/lib/innomonitor"

if [ -d "$DEPS_DIR" ]; then
    echo "Removing dependencies from: $DEPS_DIR"
    sudo rm -rf "$DEPS_DIR"
    echo "✅ Dependencies removed successfully"
else
    echo "ℹ️  No dependencies found to remove"
fi

echo ""
echo "Note: This does not remove Homebrew or libpcap if installed system-wide"
echo "To remove libpcap completely: brew uninstall libpcap"
EOF

chmod +x "$DIST_DIR/uninstall-dependencies.sh"

# Create distribution info file
cat > "$DIST_DIR/distribution-info.json" << EOF
{
    "app_name": "$APP_NAME",
    "app_version": "$VERSION",
    "distribution_type": "separate_dependencies",
    "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "requires_dependency_installer": true,
    "dependency_installer": "install-macos-deps.sh",
    "minimum_macos_version": "10.13",
    "architecture": "universal",
    "files": [
        "InnoMonitor.app",
        "install-macos-deps.sh",
        "launch-innomonitor.sh",
        "uninstall-dependencies.sh",
        "README.txt"
    ]
}
EOF

# Create ZIP archive
echo "🗜️  Creating ZIP archive..."
cd "$DIST_DIR"
zip -r "../InnoMonitor-v${VERSION}-macos.zip" .
cd - > /dev/null

# Create DMG (if hdiutil is available)
if command -v hdiutil &> /dev/null; then
    echo "💿 Creating DMG image..."
    hdiutil create -volname "InnoMonitor v${VERSION}" \
        -srcfolder "$DIST_DIR" \
        -ov -format UDZO \
        "dist/InnoMonitor-v${VERSION}-macos.dmg"
    echo "✅ DMG created: dist/InnoMonitor-v${VERSION}-macos.dmg"
else
    echo "⚠️  hdiutil not available, skipping DMG creation"
fi

echo ""
echo "🎉 Distribution package created successfully!"
echo ""
echo "📁 Distribution directory: $DIST_DIR"
echo "📦 ZIP archive: dist/InnoMonitor-v${VERSION}-macos.zip"
if [ -f "dist/InnoMonitor-v${VERSION}-macos.dmg" ]; then
    echo "💿 DMG image: dist/InnoMonitor-v${VERSION}-macos.dmg"
fi
echo ""
echo "📋 Distribution contents:"
ls -la "$DIST_DIR"
echo ""
echo "🚀 Ready for distribution!"
