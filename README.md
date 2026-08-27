# HomeCalc

A high-precision mortgage and real estate financing scenario calculator powered by Rust WebAssembly and Svelte 5.

[![Rust](https://img.shields.io/badge/Rust-2024-black?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-654FF0?style=flat-square&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Svelte 5](https://img.shields.io/badge/Svelte_5-Runes-FF3E00?style=flat-square&logo=svelte&logoColor=white)](https://svelte.dev/)
[![Vercel](https://img.shields.io/badge/Vercel-Live_App-000000?style=flat-square&logo=vercel&logoColor=white)](https://homecalc-six.vercel.app)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)

🌐 **Live Web App**: [https://homecalc-six.vercel.app](https://homecalc-six.vercel.app)

---

## ✨ Features

- **Multi-Instrument Financing**: Model mortgages, lines of credit (LOC/HELOC), down payments, and custom prepayment schedules.
- **3-Slot Comparison**: Compare scenarios side-by-side with difference metrics ($\Delta$) and state persistence.
- **Interactive Charts**: Responsive SVG trajectory charts, annual cash outflow distributions, and cost breakdowns.
- **Statement Ledger**: Full month-by-month and year-by-year amortization schedules with CSV export.
- **IRR & House Parity Guard**: Accurate scenario Net Present Value (NPV) and Strategy Internal Rate of Return (IRR) validation.
- **Mobile Responsive**: Clean, modern interface optimized for desktop and mobile screens.

## 🛠️ Tech Stack

- **Core Simulation**: Rust (`crates/engine`)
- **WebAssembly**: `wasm-bindgen` (`crates/engine-wasm`)
- **Frontend**: Svelte 5 (Runes), TypeScript, Tailwind CSS (`web`)
- **Terminal CLI**: Rust interactive terminal app (`crates/cli`)

## 💻 Local Development

### Web App
```bash
cd web
npm install
npm run dev
```

### Terminal CLI
```bash
cargo run -p cli
```

### Running Tests & Type Checks
```bash
cargo test
cd web && npm run check
```

## 📄 License

This project is licensed under the [MIT License](LICENSE).
