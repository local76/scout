#!/bin/sh
# Resolve script directory and change to it
cd "$(dirname "$0")"

echo "Building Debian package..."

# Create staging directory structure
mkdir -p debian/usr/bin
mkdir -p ../../dist/packages

# Locate and copy binary
if [ -f "../../dist/binaries/rwifi" ]; then
    cp ../../dist/binaries/rwifi debian/usr/bin/rwifi
elif [ -f "../../target/x86_64-unknown-linux-musl/release/rwifi" ]; then
    cp ../../target/x86_64-unknown-linux-musl/release/rwifi debian/usr/bin/rwifi
elif [ -f "../../target/release/rwifi" ]; then
    cp ../../target/release/rwifi debian/usr/bin/rwifi
else
    echo "Error: compiled rwifi binary not found in target/ or dist/binaries/."
    exit 1
fi

chmod 755 debian/usr/bin/rwifi

# Run dpkg-deb to build the package
dpkg-deb --build debian ../../dist/packages/rwifi.deb

# Clean up staging binary
rm -f debian/usr/bin/rwifi
