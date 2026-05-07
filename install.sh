#!/bin/bash
# One-line installer for libtv-cli
# Usage: curl -fsSL https://raw.githubusercontent.com/clawparty-ai/libtv-cli/main/install.sh | bash

set -e

REPO="clawparty-ai/libtv-cli"

# ── Colors ──────────────────────────────────
BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ── Detect platform ─────────────────────────
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS-$ARCH" in
    Linux-x86_64|Linux-amd64)
        ASSET_NAME="libtv-cli-linux-x86_64"
        EXT="tar.gz"
        BINARY_NAME="libtv-cli"
        ;;
    Linux-aarch64|Linux-arm64)
        ASSET_NAME="libtv-cli-linux-arm64"
        EXT="tar.gz"
        BINARY_NAME="libtv-cli"
        ;;
    Darwin-arm64|Darwin-aarch64)
        ASSET_NAME="libtv-cli-macos-m1"
        EXT="tar.gz"
        BINARY_NAME="libtv-cli"
        ;;
    Darwin-x86_64|Darwin-amd64)
        warn "macOS Intel: using M1 binary (Rosetta 2 emulation)"
        ASSET_NAME="libtv-cli-macos-m1"
        EXT="tar.gz"
        BINARY_NAME="libtv-cli"
        ;;
    *)
        error "Unsupported platform: $OS $ARCH"
        echo "Supported: Linux x86_64, Linux ARM64, macOS ARM64/Intel"
        exit 1
        ;;
esac

info "Detected platform: $OS $ARCH"

# ── Fetch latest release ────────────────────
info "Fetching latest release..."
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
LATEST=$(curl -fsSL "$API_URL" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST" ]; then
    error "Failed to fetch latest release"
    exit 1
fi

info "Latest version: ${BOLD}${LATEST}${NC}"

# ── Download ────────────────────────────────
TMP_DIR=$(mktemp -d)
ASSET_URL="https://github.com/${REPO}/releases/download/${LATEST}/${ASSET_NAME}.${EXT}"

info "Downloading ${ASSET_NAME}.${EXT}..."
curl -fsSL -L -o "${TMP_DIR}/archive.${EXT}" "$ASSET_URL"

# ── Extract ─────────────────────────────────
info "Extracting..."
case "$EXT" in
    tar.gz) tar xzf "${TMP_DIR}/archive.${EXT}" -C "$TMP_DIR" ;;
    zip)    unzip -q "${TMP_DIR}/archive.${EXT}" -d "$TMP_DIR" ;;
esac

# Find the actual binary inside extracted dir
EXTRACTED_DIR="${TMP_DIR}/${ASSET_NAME}"
if [ ! -d "$EXTRACTED_DIR" ]; then
    error "Extraction failed: expected directory not found"
    exit 1
fi

BINARY_PATH="${EXTRACTED_DIR}/${BINARY_NAME}"
chmod +x "$BINARY_PATH"

# ── Install binary ──────────────────────────
# Try /usr/local/bin first, fallback to ~/.local/bin
if [ -w "/usr/local/bin" ]; then
    INSTALL_BIN="/usr/local/bin"
    NEED_SUDO=false
elif [ -w "$HOME/.local/bin" ] || mkdir -p "$HOME/.local/bin" 2>/dev/null; then
    INSTALL_BIN="$HOME/.local/bin"
    NEED_SUDO=false
else
    INSTALL_BIN="$HOME/.local/bin"
    NEED_SUDO=false
    mkdir -p "$INSTALL_BIN"
fi

info "Installing binary to ${BOLD}${INSTALL_BIN}${NC}"
cp "$BINARY_PATH" "${INSTALL_BIN}/${BINARY_NAME}"

# ── Install skill files ─────────────────────
DEFAULT_SKILL_DIR="$HOME/.clawparty/skills/libtv-image-gen"
SKILL_DIR="${SKILL_DIR:-$DEFAULT_SKILL_DIR}"

info "Installing skill to ${BOLD}${SKILL_DIR}${NC}"
mkdir -p "$SKILL_DIR"

BASE_RAW="https://raw.githubusercontent.com/${REPO}/main/skill"

curl -fsSL "${BASE_RAW}/SKILL.md"            -o "${SKILL_DIR}/SKILL.md"
curl -fsSL "${BASE_RAW}/generate_image.sh"   -o "${SKILL_DIR}/generate_image.sh"
curl -fsSL "${BASE_RAW}/generate_image.bat"  -o "${SKILL_DIR}/generate_image.bat"
chmod +x "${SKILL_DIR}/generate_image.sh"

# ── Summary ─────────────────────────────────
echo ""
echo "========================================"
echo -e "${GREEN}${BOLD}Installation Complete!${NC}"
echo "========================================"
echo ""
echo -e "Binary:     ${BOLD}${INSTALL_BIN}/${BINARY_NAME}${NC}"
echo -e "Skill:      ${BOLD}${SKILL_DIR}${NC}"
echo -e "Version:    ${BOLD}${LATEST}${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Ensure the install directory is in your PATH:"
echo "   export PATH=\"${INSTALL_BIN}:\$PATH\""
echo ""
echo "2. Set your LiblibAI API credentials:"
echo "   export LIBLIB_ACCESS_KEY=\"your-access-key\""
echo "   export LIBLIB_SECRET_KEY=\"your-secret-key\""
echo ""
echo "3. Verify installation:"
echo "   ${BINARY_NAME} --version"
echo ""
echo "4. Generate your first image:"
echo "   ${BINARY_NAME} text2img \\"
echo '     --prompt "a cute cat in space" \'
echo '     --template-uuid "bf085132c7134622895b783b520b39ff" \'
echo '     -o cat.png'
echo ""
echo "For agents: add this to SOUL.md / IDENTITY.md:"
echo '   "Image generation via libtv-image-gen skill"'
echo "========================================"

# ── Cleanup ─────────────────────────────────
rm -rf "$TMP_DIR"
