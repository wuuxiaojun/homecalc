# 🏡 HomeCalc

[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-black?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Svelte 5](https://img.shields.io/badge/Svelte_5-Runes-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)](https://svelte.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-Strict-3178C6?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-3.4-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)](https://tailwindcss.com/)
[![Vercel](https://img.shields.io/badge/Vercel-Deployed-000000?style=for-the-badge&logo=vercel&logoColor=white)](https://homecalc-six.vercel.app)
[![License](https://img.shields.io/badge/License-MIT-emerald?style=for-the-badge)](LICENSE)

👉 **Live App**: [https://homecalc-six.vercel.app](https://homecalc-six.vercel.app)

**HomeCalc** is a deterministic, high-precision mortgage and real estate financing scenario calculator. Powered by a high-performance Rust WebAssembly simulation core and a reactive Svelte 5 interface, it enables homebuyers and investors to accurately model complex capital stacks, optimize prepayment schedules, and benchmark financing strategies with mathematical rigor.

---

## ✨ Key Features

- **3-Slot Concurrent Modeling**: Test, clone, and switch between 3 independent financing scenarios with live state persistence.
- **Multi-Instrument Financing**: Structure multi-tier capital stacks combining Conventional Mortgages, Lines of Credit (LOC / HELOC), Cash Down Payments, and custom Prepayment schedules (lump sums or recurring velocity).
- **Comprehensive Analytics**: Interactive SVG balance trajectory charts with hover scrubbing, lifetime cost breakdowns, and annual cash outflow distributions.
- **Financial Statement Ledger**: Detailed month-by-month and year-by-year amortization schedules with instant CSV export.
- **Scenario Comparison & IRR**: Side-by-side differential analysis ($\Delta = B - A$), Net Present Value (NPV), and Strategy Internal Rate of Return (IRR) with House Parity Guard.

---

## ⚡ Quickstart & Local Development

### Web Application

```bash
# Automated build & preview
./run.sh

# Or run the development server directly
cd web && npm install && npm run dev
```

### Interactive Terminal CLI

```bash
cargo run -p cli
```

### Running Tests

```bash
# Rust engine & CLI tests
cargo test

# Playwright end-to-end test suite
cd web && npm run test:e2e
```

---

## 🛠️ Tech Stack

- **Core Engine**: Pure Rust zero-deviation financial simulation core ([`crates/engine`](crates/engine))
- **WASM Bridge**: High-throughput bindings via `wasm-bindgen` ([`crates/engine-wasm`](crates/engine-wasm))
- **Frontend UI**: Svelte 5 (Runes), TypeScript, and Tailwind CSS ([`web`](web))
- **Terminal CLI**: Interactive wizard powered by `crossterm` and `comfy-table` ([`crates/cli`](crates/cli))

---

## 📄 License

Distributed under the [MIT License](LICENSE).
