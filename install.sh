#!/usr/bin/env bash
# Installer script for nuewframe-timeout
# Usage: curl -fsSL https://raw.githubusercontent.com/nuewframe/timeout/main/install.sh | bash

set -e

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64|arm64) TARGET="x86_64-apple-darwin" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

# Configuration
REPO="nuewframe/timeout"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="timeout"

echo "Installing timeout for $OS/$ARCH..."

# Get latest release tag
LATEST_TAG=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "Error: Could not determine latest release"
    exit 1
fi

echo "Latest version: $LATEST_TAG"

# Construct download URL
if [ "$OS" = "Linux" ] || [ "$OS" = "Darwin" ]; then
    ARCHIVE="timeout-${LATEST_TAG}-${TARGET}.tar.gz"
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$ARCHIVE"
else
    echo "Unsupported OS for this installer"
    exit 1
fi

# Create temporary directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

echo "Downloading from $DOWNLOAD_URL..."
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ARCHIVE"

# Extract
echo "Extracting..."
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Install binary
echo "Installing to $INSTALL_DIR/$BINARY_NAME..."
mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo ""
echo "✓ timeout installed successfully!"
echo ""

# Check if install directory is in PATH
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        echo "⚠️  Note: $INSTALL_DIR is not in your PATH"
        echo "   Add it by running:"
        echo ""
        if [ -f "$HOME/.zshrc" ]; then
            echo "   echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc"
            echo "   source ~/.zshrc"
        elif [ -f "$HOME/.bashrc" ]; then
            echo "   echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
            echo "   source ~/.bashrc"
        else
            echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
        fi
        echo ""
        ;;
esac

# Verify installation
if command -v timeout >/dev/null 2>&1; then
    echo "Run 'timeout --version' to verify the installation"
else
    echo "Run '$INSTALL_DIR/timeout --version' to verify the installation"
fi
