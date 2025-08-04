#!/bin/bash

# InnoMonitor macOS Launcher
# This script properly launches InnoMonitor with required permissions

set -e

echo "🚀 InnoMonitor Launcher"
echo "======================="

APP_PATH="/Applications/InnoMonitor.app"
BINARY_PATH="$APP_PATH/Contents/MacOS/InnoMonitor"
DEPS_DIR="/usr/local/lib/innomonitor"

# Check if app exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ InnoMonitor not found at: $APP_PATH"
    echo "   Please install InnoMonitor first"
    exit 1
fi

# Check if dependencies are installed
if [ ! -f "$DEPS_DIR/libpcap.dylib" ]; then
    echo "❌ Dependencies not installed"
    echo "   Please run: bash install-macos-deps.sh"
    exit 1
fi

echo "✅ Found InnoMonitor at: $APP_PATH"
echo "✅ Found dependencies at: $DEPS_DIR"
echo ""

# Method 1: Try to launch without sudo first (for signed apps)
echo "🔄 Attempting to launch InnoMonitor..."
echo "   (You may be prompted for network monitoring permission)"

# Set up environment variables
export PKG_CONFIG_PATH="$(brew --prefix libpcap)/lib/pkgconfig:$PKG_CONFIG_PATH"
export LIBPCAP_LIBDIR="$DEPS_DIR"

# Try normal launch first
if open "$APP_PATH" 2>/dev/null; then
    echo "✅ InnoMonitor launched successfully!"
    echo ""
    echo "📱 If the app requests permission for network monitoring, please:"
    echo "   1. Click 'Allow' in any permission dialogs"
    echo "   2. Check System Settings → Privacy & Security if needed"
    echo ""
    echo "🔍 The app should now be running. Check your Applications or Dock."
else
    echo "⚠️  Normal launch failed, trying with elevated permissions..."
    echo ""
    echo "🔐 InnoMonitor requires special permissions for network monitoring."
    echo "   Please enter your password when prompted:"
    echo ""
    
    # Create a temporary script that runs the binary directly with proper environment
    TEMP_SCRIPT=$(mktemp)
    cat > "$TEMP_SCRIPT" << EOF
#!/bin/bash
export PKG_CONFIG_PATH="$(brew --prefix libpcap)/lib/pkgconfig:\$PKG_CONFIG_PATH"
export LIBPCAP_LIBDIR="$DEPS_DIR"
cd "$APP_PATH/Contents/MacOS"
exec ./InnoMonitor
EOF
    
    chmod +x "$TEMP_SCRIPT"
    
    # Run with sudo
    if sudo "$TEMP_SCRIPT"; then
        echo "✅ InnoMonitor launched with elevated permissions!"
    else
        echo "❌ Failed to launch InnoMonitor"
        echo ""
        echo "🔧 Troubleshooting:"
        echo "   1. Make sure you entered the correct password"
        echo "   2. Try: System Settings → Privacy & Security → Full Disk Access"
        echo "   3. Add InnoMonitor.app to the allowed apps list"
        echo "   4. Try launching again"
    fi
    
    # Clean up
    rm -f "$TEMP_SCRIPT"
fi

echo ""
echo "📖 For future launches, you can:"
echo "   1. Use this launcher script: bash launch-innomonitor.sh"
echo "   2. Add InnoMonitor to Full Disk Access in System Settings"
echo "   3. Then use: open /Applications/InnoMonitor.app"
echo ""
