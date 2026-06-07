#!/usr/bin/env bash
# install.sh — build dashboard in release mode and install it to your PATH.
#
# Usage:
#   ./install.sh              # installs to /usr/local/bin (may need sudo)
#   ./install.sh ~/.local/bin # installs to a custom directory
#
# Uninstall:
#   rm $(which dashboard)

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

info()    { echo -e "${CYAN}${BOLD}==>${RESET} $*"; }
success() { echo -e "${GREEN}${BOLD}  ✓${RESET} $*"; }
error()   { echo -e "${RED}${BOLD}  ✗${RESET} $*" >&2; }

INSTALL_DIR="${1:-/usr/local/bin}"

if ! command -v cargo &>/dev/null; then
    error "cargo not found. Install Rust from https://rustup.rs and try again."
    exit 1
fi

if [[ ! -f "Cargo.toml" ]]; then
    error "Run this script from the dashboard project root (where Cargo.toml is)."
    exit 1
fi

if ! grep -q 'name = "dashboard"' Cargo.toml; then
    error "This doesn't look like the dashboard project (name not found in Cargo.toml)."
    exit 1
fi

info "Building release binary..."
cargo build --release 2>&1

BINARY="target/release/dashboard"

if [[ ! -f "$BINARY" ]]; then
    error "Build succeeded but binary not found at $BINARY."
    exit 1
fi

success "Build complete ($(du -sh "$BINARY" | cut -f1))"

info "Installing to ${INSTALL_DIR}/dashboard..."

mkdir -p "$INSTALL_DIR"

if cp "$BINARY" "$INSTALL_DIR/dashboard" 2>/dev/null; then
    success "Installed to ${INSTALL_DIR}/dashboard"
else
    info "Permission denied — retrying with sudo..."
    sudo cp "$BINARY" "$INSTALL_DIR/dashboard"
    success "Installed to ${INSTALL_DIR}/dashboard (via sudo)"
fi

chmod +x "$INSTALL_DIR/dashboard"

if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
    echo ""
    echo -e "${BOLD}Note:${RESET} ${INSTALL_DIR} is not in your PATH."
    echo    "Add this line to your shell config (~/.zshrc, ~/.bashrc, etc.):"
    echo ""
    echo -e "    ${CYAN}export PATH=\"${INSTALL_DIR}:\$PATH\"${RESET}"
    echo ""
    echo    "Then restart your shell or run:  source ~/.zshrc"
else
    echo ""
    success "Done! Run it with:  dashboard"
    echo    "         or with a custom refresh:  dashboard --refresh-ms 2000"
fi
