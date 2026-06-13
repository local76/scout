#!/bin/bash
# build.sh: Build scout and package it into the dist/ directory.
set -e

# Navigate to project root relative to this script
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

echo "=== Building scout in release mode ==="
cargo build --release

# Collect artifacts
DIST_DIR="$PROJECT_ROOT/dist"
BIN_DIR="$DIST_DIR/binaries"
rm -rf "$DIST_DIR"
mkdir -p "$BIN_DIR"

echo "=== Collecting binaries ==="
if [ -f "target/release/scout" ]; then
    cp "target/release/scout" "$BIN_DIR/"
    echo "Copied scout binary to $BIN_DIR/scout"
fi
if [ -f "target/release/scout.exe" ]; then
    cp "target/release/scout.exe" "$BIN_DIR/"
    echo "Copied scout.exe to $BIN_DIR/scout.exe"
fi

# Build debian package if cargo-deb is installed
if command -v cargo-deb &> /dev/null; then
    echo "=== Building DEB package ==="
    if cargo deb; then
        cp target/debian/*.deb "$BIN_DIR/" 2>/dev/null || true
        echo "DEB package created and copied to $BIN_DIR"
    else
        echo "Warning: cargo-deb build failed."
    fi
else
    echo "Skipping DEB package build (cargo-deb not installed)."
fi

echo "=== Build completed successfully! Output in: $DIST_DIR ==="
