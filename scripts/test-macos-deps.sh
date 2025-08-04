#!/bin/bash

echo "🧪 Testing InnoMonitor macOS Dependency Installation"
echo "=================================================="

DEPS_DIR="/usr/local/lib/innomonitor"

echo "1. Checking if dependency directory exists..."
if [ -d "$DEPS_DIR" ]; then
    echo "   ✅ Directory exists: $DEPS_DIR"
else
    echo "   ❌ Directory missing: $DEPS_DIR"
    echo "   Run install-macos-deps.sh first"
    exit 1
fi

echo ""
echo "2. Checking libpcap files..."
if [ -f "$DEPS_DIR/libpcap.dylib" ]; then
    echo "   ✅ Main library: $DEPS_DIR/libpcap.dylib"
    
    # Check what the symlink points to
    REAL_PATH=$(readlink "$DEPS_DIR/libpcap.dylib")
    echo "   📍 Points to: $REAL_PATH"
    
    # Verify the target exists
    if [ -f "$REAL_PATH" ]; then
        echo "   ✅ Target file exists"
    else
        echo "   ❌ Target file missing: $REAL_PATH"
    fi
else
    echo "   ❌ Main library missing: $DEPS_DIR/libpcap.dylib"
fi

echo ""
echo "3. Checking dependency info..."
if [ -f "$DEPS_DIR/dependency-info.json" ]; then
    echo "   ✅ Info file exists"
    echo "   📄 Contents:"
    cat "$DEPS_DIR/dependency-info.json" | jq . 2>/dev/null || cat "$DEPS_DIR/dependency-info.json"
else
    echo "   ❌ Info file missing: $DEPS_DIR/dependency-info.json"
fi

echo ""
echo "4. Testing library loading..."
if otool -L "$DEPS_DIR/libpcap.dylib" >/dev/null 2>&1; then
    echo "   ✅ Library structure is valid"
    echo "   📚 Dependencies:"
    otool -L "$DEPS_DIR/libpcap.dylib" | grep -v ":" | head -5
else
    echo "   ❌ Library structure check failed"
fi

echo ""
echo "5. Testing pkg-config..."
LIBPCAP_PREFIX=$(brew --prefix libpcap 2>/dev/null)
if [ -n "$LIBPCAP_PREFIX" ]; then
    echo "   📁 libpcap prefix: $LIBPCAP_PREFIX"
    export PKG_CONFIG_PATH="$LIBPCAP_PREFIX/lib/pkgconfig:$PKG_CONFIG_PATH"
    
    if pkg-config --exists libpcap; then
        echo "   ✅ pkg-config can find libpcap"
        echo "   📊 Version: $(pkg-config --modversion libpcap)"
        echo "   🔗 Libs: $(pkg-config --libs libpcap)"
    else
        echo "   ❌ pkg-config cannot find libpcap"
    fi
else
    echo "   ⚠️  Cannot determine libpcap prefix (Homebrew issue?)"
fi

echo ""
echo "6. Listing all files in dependency directory..."
ls -la "$DEPS_DIR"

echo ""
echo "🎯 Test Summary:"
if [ -f "$DEPS_DIR/libpcap.dylib" ] && otool -L "$DEPS_DIR/libpcap.dylib" >/dev/null 2>&1; then
    echo "   ✅ Dependencies appear to be correctly installed"
    echo "   🚀 InnoMonitor should be able to find libpcap"
else
    echo "   ❌ Dependencies have issues"
    echo "   🔧 Try running install-macos-deps.sh again"
fi
