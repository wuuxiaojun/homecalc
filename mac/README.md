# HomeCalc Standalone macOS Launcher Bundle

This directory contains the standalone macOS application bundle builder for **HomeCalc**. It packages the high-performance Rust WASM financial engine and the modern Svelte SPA frontend into a single native macOS application with **zero runtime dependencies** (no Node.js, Python, or external package managers required on the target machine).

---

## 🏗️ Architecture & How It Works

1. **Embedded Assets**: The compiled web assets (`web/dist/`)—including HTML, CSS, JavaScript, and WebAssembly (`engine_wasm`)—are embedded directly into the Rust launcher binary via compile-time static embedding (`rust-embed`).
2. **Built-in HTTP & SPA Engine**: An ultra-fast, lightweight asynchronous HTTP server (`axum` + `tokio`) serves the embedded assets locally with proper MIME type guessing (`mime_guess`) and Single Page Application (SPA) client-side routing fallbacks.
3. **Smart Port Binding**: The launcher attempts to bind to preferred local ports (e.g., `8080`, `8081`, `8082`, `3000`, `5173`) and falls back automatically to dynamic OS-allocated ephemeral ports (`127.0.0.1:0`), avoiding port collision errors. A custom port can also be specified via the `PORT` or `HOMECALC_PORT` environment variable.
4. **Native Browser Launch**: Upon startup, the launcher automatically opens the application in the user's default macOS browser (`open` crate).
5. **Native macOS `.app` Bundle**: The build script structures the application into a standard `HomeCalc.app` bundle containing `Info.plist`, `MacOS/HomeCalc` executable, and packaging metadata, plus a zipped distribution (`HomeCalc-macOS.zip`).

---

## 📋 Prerequisites (For Building)

To build the macOS bundle from source, ensure the following tools are installed:

- **Rust toolchain & Cargo** (1.70+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **wasm-pack**: `cargo install wasm-pack` or `brew install wasm-pack`
- **Node.js & npm** (v18+): `brew install node`
- **zip** (standard on macOS and Linux)

---

## 🚀 Building the macOS App Bundle

Run the automated build script from the repository root or within the `mac/` directory:

```bash
# From repo root:
./mac/build_mac.sh

# Or from inside mac/ directory:
cd mac
./build_mac.sh
```

### Build Steps Automated by `build_mac.sh`:
1. **Prerequisite Check**: Validates `cargo`, `wasm-pack`, and `npm`.
2. **WASM Compilation**: Compiles `crates/engine-wasm` to WebAssembly target.
3. **Frontend Build**: Installs dependencies and compiles the Svelte frontend into `web/dist`.
4. **Binary Compilation**: Builds the release Rust launcher binary `mac/target/release/homecalc-mac-launcher` with embedded frontend assets.
5. **Bundle Assembly**: Assembles `HomeCalc.app/Contents/MacOS/HomeCalc` with `Info.plist` and compresses it into `HomeCalc-macOS.zip`.

---

## 💻 Running the Application

### On macOS:
- **Finder**: Double-click `HomeCalc.app` or drag it to `/Applications`.
- **Terminal**:
  ```bash
  open mac/HomeCalc.app
  ```
  or run the binary directly:
  ```bash
  ./mac/HomeCalc.app/Contents/MacOS/HomeCalc
  ```

### Custom Port:
To run on a specific port:
```bash
PORT=3000 ./mac/HomeCalc.app/Contents/MacOS/HomeCalc
```

---

## 📦 Directory Structure

```
mac/
├── Cargo.toml          # Standalone Rust binary crate & workspace definition
├── src/
│   └── main.rs         # Embedded web server, SPA routing & browser opener
├── build_mac.sh        # Complete build automation script
├── README.md           # Documentation and usage instructions
├── HomeCalc.app/       # Generated macOS Application Bundle (post-build)
│   └── Contents/
│       ├── Info.plist  # Bundle metadata and configuration
│       ├── PkgInfo     # APPL signature
│       ├── MacOS/
│       │   └── HomeCalc # Native launcher binary
│       └── Resources/  # Assets and icons
└── HomeCalc-macOS.zip  # Ready-to-distribute compressed archive (post-build)
```
