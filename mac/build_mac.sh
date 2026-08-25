#!/usr/bin/env bash
set -euo pipefail

# ANSI color codes
BOLD='\033[1m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo -e "${BOLD}${BLUE}====================================================${NC}"
echo -e "${BOLD}${BLUE}   HomeCalc macOS Standalone App Bundle Builder     ${NC}"
echo -e "${BOLD}${BLUE}====================================================${NC}"

# -----------------------------------------------------------------------------
# 1. Verify Prerequisites
# -----------------------------------------------------------------------------
echo -e "\n${BOLD}[1/5] Checking prerequisites...${NC}"

for cmd in wasm-pack npm cargo; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo -e "${RED}Error: '$cmd' is not installed or not in PATH.${NC}" >&2
        exit 1
    fi
    echo -e "  ✓ ${GREEN}$cmd${NC} found: $(command -v "$cmd")"
done

# -----------------------------------------------------------------------------
# 2. Build WASM Engine
# -----------------------------------------------------------------------------
echo -e "\n${BOLD}[2/5] Building WASM engine (crates/engine-wasm)...${NC}"
mkdir -p "$REPO_ROOT/web/src/lib/wasm"

wasm-pack build "$REPO_ROOT/crates/engine-wasm" \
    --target web \
    --out-dir "$REPO_ROOT/web/src/lib/wasm" \
    --out-name engine_wasm \
    --release

echo -e "${GREEN}  ✓ WASM engine build complete.${NC}"

# -----------------------------------------------------------------------------
# 3. Build Web Frontend
# -----------------------------------------------------------------------------
echo -e "\n${BOLD}[3/5] Building web frontend bundle...${NC}"
cd "$REPO_ROOT/web"
if [ ! -d "node_modules" ]; then
    echo "  - Running npm install..."
    npm install --silent --no-save
fi
echo "  - Running npm run build..."
npm run build
cd "$SCRIPT_DIR"
echo -e "${GREEN}  ✓ Frontend production build complete.${NC}"

# -----------------------------------------------------------------------------
# 4. Compile Standalone Mac Launcher Binary
# -----------------------------------------------------------------------------
echo -e "\n${BOLD}[4/5] Compiling standalone launcher binary...${NC}"
cd "$SCRIPT_DIR"
cargo build --release --manifest-path "$SCRIPT_DIR/Cargo.toml"

BINARY_PATH="$SCRIPT_DIR/target/release/homecalc-mac-launcher"
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}Error: Compiled binary not found at $BINARY_PATH${NC}" >&2
    exit 1
fi
echo -e "${GREEN}  ✓ Launcher binary compiled successfully at $BINARY_PATH${NC}"

# -----------------------------------------------------------------------------
# 5. Assemble macOS .app Bundle & Archive
# -----------------------------------------------------------------------------
echo -e "\n${BOLD}[5/5] Assembling macOS .app bundle...${NC}"

APP_BUNDLE="$SCRIPT_DIR/HomeCalc.app"
CONTENTS_DIR="$APP_BUNDLE/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

# Clean up any existing bundle & zip
rm -rf "$APP_BUNDLE" "$SCRIPT_DIR/HomeCalc-macOS.zip"

mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy binary to HomeCalc.app/Contents/MacOS/HomeCalc
cp "$BINARY_PATH" "$MACOS_DIR/HomeCalc"
chmod +x "$MACOS_DIR/HomeCalc"

# Write standard Info.plist
cat <<'EOF' > "$CONTENTS_DIR/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>HomeCalc</string>
    <key>CFBundleExecutable</key>
    <string>HomeCalc</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>com.homecalc.launcher</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>HomeCalc</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>2.1.0</string>
    <key>CFBundleVersion</key>
    <string>2.1.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.finance</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2026 HomeCalc. All rights reserved.</string>
</dict>
</plist>
EOF

# Write PkgInfo
echo -n "APPL????" > "$CONTENTS_DIR/PkgInfo"

echo -e "${GREEN}  ✓ HomeCalc.app structure created.${NC}"

# Archive bundle into HomeCalc-macOS.zip
echo "  - Creating HomeCalc-macOS.zip archive..."
if command -v ditto >/dev/null 2>&1; then
    ditto -c -k --keepParent "$APP_BUNDLE" "$SCRIPT_DIR/HomeCalc-macOS.zip"
    echo -e "${GREEN}  ✓ Archive created with ditto: $SCRIPT_DIR/HomeCalc-macOS.zip${NC}"
elif command -v zip >/dev/null 2>&1; then
    (cd "$SCRIPT_DIR" && zip -q -r -y "HomeCalc-macOS.zip" "HomeCalc.app")
    echo -e "${GREEN}  ✓ Archive created with zip: $SCRIPT_DIR/HomeCalc-macOS.zip${NC}"
elif command -v python3 >/dev/null 2>&1; then
    python3 -c "
import os, zipfile, stat
zip_path = '$SCRIPT_DIR/HomeCalc-macOS.zip'
app_dir = '$SCRIPT_DIR/HomeCalc.app'
with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zf:
    for root, dirs, files in os.walk(app_dir):
        for f in files:
            full_path = os.path.join(root, f)
            rel_path = os.path.relpath(full_path, '$SCRIPT_DIR')
            zinfo = zipfile.ZipInfo.from_file(full_path, rel_path)
            mode = os.stat(full_path).st_mode
            zinfo.external_attr = (mode & 0xFFFF) << 16
            with open(full_path, 'rb') as fp:
                zf.writestr(zinfo, fp.read())
"
    echo -e "${GREEN}  ✓ Archive created with python3: $SCRIPT_DIR/HomeCalc-macOS.zip${NC}"
else
    echo -e "${YELLOW}  ! No zip utility found, skipping zip archive creation.${NC}"
fi

echo -e "\n${BOLD}${GREEN}====================================================${NC}"
echo -e "${BOLD}${GREEN}   macOS Standalone App Build Complete!             ${NC}"
echo -e "${BOLD}${GREEN}====================================================${NC}"
echo -e "App Bundle: ${BOLD}$APP_BUNDLE${NC}"
if [ -f "$SCRIPT_DIR/HomeCalc-macOS.zip" ]; then
    echo -e "Zip Distribution: ${BOLD}$SCRIPT_DIR/HomeCalc-macOS.zip${NC}"
fi
echo -e "\nTo run directly on macOS:"
echo -e "  open \"$APP_BUNDLE\""
echo -e "or execute:"
echo -e "  \"$MACOS_DIR/HomeCalc\""
