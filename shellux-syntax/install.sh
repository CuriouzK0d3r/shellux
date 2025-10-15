#!/bin/bash

# Shellux Syntax Extension Installer
# This script installs the Shellux syntax highlighting extension to VS Code

set -e

EXTENSION_NAME="shellux-syntax"
EXTENSION_DIR="$HOME/.vscode/extensions/$EXTENSION_NAME"

echo "Installing Shellux Syntax Highlighting Extension..."

# Optional: check for VS Code CLI (not required to copy the extension)
if ! command -v code &> /dev/null; then
    echo "Warning: VS Code 'code' command not found in PATH."
    echo "We'll still install the files; use VS Code UI to reload or install from VSIX."
fi

# Create extensions directory if it doesn't exist
mkdir -p "$HOME/.vscode/extensions"

# Remove old version if it exists
if [ -d "$EXTENSION_DIR" ]; then
    echo "Removing old version..."
    rm -rf "$EXTENSION_DIR"
fi

# Copy extension files
echo "Copying extension files..."
cp -r "$(dirname "$0")" "$EXTENSION_DIR"

# Remove installer script from destination
rm -f "$EXTENSION_DIR/install.sh"

echo "✓ Extension installed successfully!"
echo ""
echo "To activate the extension:"
echo "1. Reload VS Code window (Cmd+R on macOS, Ctrl+R on Linux/Windows)"
echo "   or restart VS Code"
echo "2. Open any .sx or .shx file"
echo ""
echo "Happy coding with Shellux!"
