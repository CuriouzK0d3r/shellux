#!/bin/bash

# Shellux Syntax Extension Installer for Zed Editor
# This script installs the Shellux syntax highlighting extension to Zed

set -e

EXTENSION_NAME="shellux"
ZED_EXTENSIONS_DIR="$HOME/.config/zed/extensions"
EXTENSION_DIR="$ZED_EXTENSIONS_DIR/$EXTENSION_NAME"

echo "Installing Shellux Syntax Highlighting Extension for Zed..."
echo ""

# Check if Zed is installed
if ! command -v zed &> /dev/null && [ ! -d "$HOME/.config/zed" ]; then
    echo "⚠️  Warning: Zed doesn't appear to be installed"
    echo "   However, we'll install the extension anyway."
    echo ""
fi

# Create extensions directory if it doesn't exist
echo "Creating extensions directory..."
mkdir -p "$ZED_EXTENSIONS_DIR"

# Remove old version if it exists
if [ -d "$EXTENSION_DIR" ]; then
    echo "Removing old version..."
    rm -rf "$EXTENSION_DIR"
fi

# Copy extension files
echo "Copying extension files..."
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cp -r "$SCRIPT_DIR" "$EXTENSION_DIR"

# Remove installer script and docs from destination
rm -f "$EXTENSION_DIR/install.sh"
rm -f "$EXTENSION_DIR/QUICKSTART.md"

echo ""
echo "✅ Extension installed successfully!"
echo ""
echo "Installation location: $EXTENSION_DIR"
echo ""
echo "To activate the extension:"
echo "1. Restart Zed completely"
echo "   or use Command Palette: 'zed: reload extensions'"
echo "2. Open any .sx or .shx file"
echo ""
echo "Test it:"
echo "  echo 'name is \"World\"' > test.sx"
echo "  zed test.sx"
echo ""
echo "Happy coding with Shellux in Zed! ⚡"
