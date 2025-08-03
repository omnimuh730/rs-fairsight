#!/bin/bash

# Post-build script for macOS to bundle libpcap with the app
# This ensures the app works on machines without Homebrew/libpcap installed

set -e  # Exit on any error

echo "🍎 macOS Post-Build: Bundling libpcap with app..."

# Configuration
APP_NAME="InnoMonitor"
BUNDLE_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
FRAMEWORKS_DIR="${BUNDLE_PATH}/Contents/Frameworks"
BINARY_PATH="${BUNDLE_PATH}/Contents/MacOS/${APP_NAME}"

# Check if the app bundle exists
if [ ! -d "$BUNDLE_PATH" ]; then
    echo "❌ App bundle not found at: $BUNDLE_PATH"
    echo "   Please run 'npm run tauri build' first"
    
    # In CI environment, this might be expected if we're running pre-build
    if [ -n "$GITHUB_ACTIONS" ]; then
        echo "ℹ️  GitHub Actions environment detected - this might be a pre-build step"
        echo "🎉 macOS Post-Build: Skipping bundling (will be handled after Tauri build)"
        exit 0
    fi
    
    exit 1
fi

echo "✅ Found app bundle at: $BUNDLE_PATH"

# Verify the binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ App binary not found at: $BINARY_PATH"
    echo "   Bundle exists but binary is missing"
    exit 1
fi

# Create Frameworks directory if it doesn't exist
if [ ! -d "$FRAMEWORKS_DIR" ]; then
    echo "📁 Creating Frameworks directory..."
    mkdir -p "$FRAMEWORKS_DIR"
fi

# Find libpcap on the system
LIBPCAP_PATHS=(
    "/opt/homebrew/lib/libpcap.dylib"
    "/opt/homebrew/Cellar/libpcap/1.10.5/lib/libpcap.1.10.5.dylib"
    "/opt/homebrew/Cellar/libpcap/1.10.4/lib/libpcap.1.10.4.dylib"
    "/usr/local/lib/libpcap.dylib"
    "/usr/local/Cellar/libpcap/1.10.5/lib/libpcap.1.10.5.dylib"
    "/usr/local/Cellar/libpcap/1.10.4/lib/libpcap.1.10.4.dylib"
    "/usr/lib/libpcap.dylib"
)

LIBPCAP_SOURCE=""
for path in "${LIBPCAP_PATHS[@]}"; do
    if [ -f "$path" ]; then
        LIBPCAP_SOURCE="$path"
        echo "✅ Found libpcap at: $path"
        break
    fi
done

if [ -z "$LIBPCAP_SOURCE" ]; then
    echo "❌ libpcap not found in any standard location!"
    echo "   Please install libpcap: brew install libpcap"
    exit 1
fi

# Get the actual filename and version
LIBPCAP_FILENAME=$(basename "$LIBPCAP_SOURCE")
LIBPCAP_DEST="${FRAMEWORKS_DIR}/${LIBPCAP_FILENAME}"

# Copy libpcap to the app bundle
echo "📦 Copying libpcap to app bundle..."
cp "$LIBPCAP_SOURCE" "$LIBPCAP_DEST"

# Make it executable
chmod +x "$LIBPCAP_DEST"

# Check what libraries the binary is currently linked against
echo "🔍 Checking current library dependencies..."
if command -v otool >/dev/null 2>&1; then
    echo "Current libpcap dependencies:"
    otool -L "$BINARY_PATH" | grep -i pcap || echo "  No libpcap dependencies found"
else
    echo "⚠️  otool not available, skipping dependency check"
fi

# Update the library path in the binary to use the bundled version
echo "🔧 Updating library paths in binary..."

# Define the new path (relative to the binary)
NEW_LIBPCAP_PATH="@executable_path/../Frameworks/${LIBPCAP_FILENAME}"

# List of possible current libpcap paths in the binary
CURRENT_PATHS=(
    "/opt/homebrew/lib/libpcap.dylib"
    "/opt/homebrew/lib/libpcap.1.dylib"
    "/opt/homebrew/lib/libpcap.1.10.5.dylib"
    "/opt/homebrew/lib/libpcap.1.10.4.dylib"
    "/usr/local/lib/libpcap.dylib"
    "/usr/local/lib/libpcap.1.dylib"
    "/usr/local/lib/libpcap.1.10.5.dylib"
    "/usr/local/lib/libpcap.1.10.4.dylib"
    "/usr/lib/libpcap.dylib"
    "/usr/lib/libpcap.1.dylib"
)

# Try to update each possible path
for current_path in "${CURRENT_PATHS[@]}"; do
    if otool -L "$BINARY_PATH" 2>/dev/null | grep -q "$current_path"; then
        echo "🔄 Updating path: $current_path -> $NEW_LIBPCAP_PATH"
        if install_name_tool -change "$current_path" "$NEW_LIBPCAP_PATH" "$BINARY_PATH"; then
            echo "✅ Successfully updated library path"
        else
            echo "⚠️  Warning: Failed to update library path for $current_path"
        fi
    fi
done

# Also update the ID of the copied libpcap library itself
echo "🔧 Updating libpcap library ID..."
if install_name_tool -id "$NEW_LIBPCAP_PATH" "$LIBPCAP_DEST"; then
    echo "✅ Successfully updated libpcap library ID"
else
    echo "⚠️  Warning: Failed to update libpcap library ID"
fi

# Verify the changes
echo "🔍 Verifying updated dependencies..."
if command -v otool >/dev/null 2>&1; then
    echo "Updated libpcap dependencies:"
    otool -L "$BINARY_PATH" | grep -E "(pcap|@executable_path)" || echo "  No updated dependencies found"
    
    echo "Bundled libpcap info:"
    otool -L "$LIBPCAP_DEST" | head -3
else
    echo "⚠️  otool not available, skipping verification"
fi

# Check file architecture compatibility
echo "🔍 Checking architecture compatibility..."
if command -v file >/dev/null 2>&1; then
    echo "Binary architecture:"
    file "$BINARY_PATH"
    echo "libpcap architecture:"
    file "$LIBPCAP_DEST"
else
    echo "⚠️  file command not available, skipping architecture check"
fi

# Optional: Code signing (uncomment if you have a developer certificate)
# echo "✍️  Code signing app bundle..."
# if command -v codesign >/dev/null 2>&1; then
#     # Replace "Developer ID Application: Your Name" with your actual certificate
#     # codesign --force --deep --sign "Developer ID Application: Your Name" "$BUNDLE_PATH"
#     # echo "✅ App bundle signed successfully"
#     echo "⚠️  Code signing skipped (no certificate configured)"
# else
#     echo "⚠️  codesign not available, skipping code signing"
# fi

echo ""
echo "🎉 macOS Post-Build Complete!"
echo ""
echo "📦 Your app bundle now includes:"
echo "   • Bundled libpcap: $LIBPCAP_DEST"
echo "   • Updated binary paths: $BINARY_PATH"
echo ""
echo "✅ The app should now work on macOS machines without Homebrew/libpcap installed"
echo ""
echo "🧪 To test on another machine:"
echo "   1. Copy the entire app bundle: $BUNDLE_PATH"
echo "   2. Run: open $APP_NAME.app"
echo "   3. Check Console.app for any library loading errors"
echo ""
