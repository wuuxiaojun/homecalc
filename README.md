# 🏡 HomeCalc

[![Rust](https://img.shields.io/badge/Rust-2024%20Edition-black?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Svelte 5](https://img.shields.io/badge/Svelte_5-Runes-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)](https://svelte.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-Strict-3178C6?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-3.4-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)](https://tailwindcss.com/)
[![Vercel](https://img.shields.io/badge/Vercel-Deployed-000000?style=for-the-badge&logo=vercel&logoColor=white)](https://homecalc-six.vercel.app)
[![License](https://img.shields.io/badge/License-MIT-emerald?style=for-the-badge)](LICENSE)

> **High-Precision Real Estate Financing & Scenario Comparison Engine**  
> Model, compare, and optimize home acquisition strategies across conventional mortgages, lines of credit (LOC), cash reserves, and custom prepayment velocity schedules with zero math deviations.

🌐 **Live Web Application**: [https://homecalc-six.vercel.app](https://homecalc-six.vercel.app)

---

## 📑 Table of Contents

- [Overview & Value Proposition](#-overview--value-proposition)
- [System Architecture](#-system-architecture)
- [Web Interface User Guide](#-web-interface-user-guide)
  - [3-Slot Scenario Architecture](#3-slot-scenario-architecture)
  - [Parameter Input Pane (Left)](#parameter-input-pane-left)
  - [Analysis Workspaces (Right)](#analysis-workspaces-right)
  - [Scenario Management](#scenario-management)
- [Interactive CLI Guide](#-interactive-cli-guide)
- [Local Setup & Development](#-local-setup--development)
  - [Prerequisites](#prerequisites)
  - [Quick Start](#quick-start)
  - [Building Rust Crates](#building-rust-crates)
  - [Compiling WebAssembly](#compiling-webassembly)
  - [Running the Svelte 5 Web App](#running-the-svelte-5-web-app)
  - [Running Test Suites](#running-test-suites)
- [Mathematical Precision & Algorithms](#-mathematical-precision--algorithms)
- [License & Authors](#-license--authors)

---

## 🌟 Overview & Value Proposition

HomeCalc is an institutional-grade home acquisition modeling platform engineered for homebuyers, investors, and financial analysts. Traditional mortgage calculators rely on simplified heuristics or single-loan assumptions that fail to capture the complex interaction between multi-tier financing instruments, tax deductions, cash interest yields, and accelerated principal velocity.

HomeCalc solves this through:
- **Deterministic Month-by-Month Simulation**: Simulates the exact compound interest amortization, holding costs (property taxes, homeowner's insurance, HOA, maintenance), cash yield, and tax deductions across every single month up to a 30-year horizon.
- **Multi-Instrument Capital Stacks**: Combine Conventional/Fixed Mortgages, Lines of Credit (LOC / HELOC), and All-Cash allocations in a single capital stack with dynamic real-time validation.
- **Prepayment Velocity Schedules**: Model custom one-time lump-sum injections and recurring monthly prepayment intervals to calculate exact interest savings and accelerated payoff dates.
- **Pairwise Strategy Comparison & IRR**: Benchmark alternative scenarios against a baseline with differential cash flow analysis, Present Value (PV @ 6.5% discount rate), and a hybrid numerical solver for Internal Rate of Return (IRR) on accelerated equity.
- **Dual-Interface Flexibility**: Run locally in the terminal with an interactive, keyboard-driven CLI wizard or launch the responsive, browser-based WebAssembly application.

---

## 🏗️ System Architecture

HomeCalc is designed as a modular workspace cleanly separating pure calculation logic, WebAssembly bindings, modern reactive user interfaces, and command-line workflows.

```
homecalc/
├── crates/
│   ├── engine/          # Pure Rust core domain models & simulation engine
│   ├── engine-wasm/     # WebAssembly bridge compiled with wasm-bindgen
│   └── cli/             # Interactive terminal CLI wizard (crossterm + comfy-table)
├── web/                 # Svelte 5 + TypeScript + Tailwind CSS Web Application
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/  # Modals, Parameter Panes, Statements, Charts, Views
│   │   │   ├── engine/      # WASM loader & TypeScript service layer
│   │   │   ├── state/       # Reactive Svelte 5 rune state (appState.svelte.ts)
│   │   │   └── wasm/        # Generated WebAssembly binary & JS bindings
└── simulations/         # Pre-configured scenario templates & JSON benchmarks
```

### Key Components

1. **`crates/engine` (Core Rust Engine)**
   - Zero-dependency financial simulation core written in idiomatic Rust.
   - Houses domain models (`House`, `Purchase`, `FinancialTool`, `Scenario`, `MonthlyStatementRow`, `YearlyStatementRow`, `TotalStatement`).
   - Implements deterministic compound amortization, tax savings algorithms, holding cost inflation projections, and present value discounting.
   - Features a robust **Hybrid Bisection / Newton-Raphson numerical solver** for calculating the Internal Rate of Return (IRR) on strategy cash flows.

2. **`crates/engine-wasm` (WebAssembly Bridge)**
   - High-throughput WASM interface generated with `wasm-bindgen` and `serde-wasm-bindgen`.
   - Exposes zero-copy calculation bindings (`create_scenario_wasm`, `analyze_scenario_wasm`, `compare_scenarios_wasm`) directly to the browser runtime.
   - Provides sub-millisecond calculation updates during real-time UI slider adjustments.

3. **`web/` (Modern Svelte 5 Web Application)**
   - Built on the Svelte 5 Runes architecture (`$state`, `$derived`, `$props`, `$effect`) for fine-grained reactivity.
   - Styled with Tailwind CSS, dark-mode optimized palette, and custom SVG charts.
   - Responsive split-pane layout with mobile drawer and desktop dual-canvas views.
   - Client-side persistence via `localStorage` with JSON import/export and preset library.

4. **`crates/cli` (Native Terminal Wizard)**
   - Interactive terminal UI powered by `crossterm`, `inquire`, and `comfy-table`.
   - Direct scenario creation, file saving/loading, tabular monthly/yearly statement rendering, and ANSI-colored scenario comparison.

---

## 🖥️ Web Interface User Guide

The web application provides a real-time reactive workspace divided into a **Left Parameter Pane** (inputs & financing configuration) and a **Right Analysis Canvas** (workspaces & visualizations).

### 3-Slot Scenario Architecture

HomeCalc features a **3-Slot Scenario Architecture** located in the top navigation header:
- **Slot 1, Slot 2, Slot 3**: Work with up to three distinct financial scenarios concurrently.
- **Instant Slot Switching**: Switch between scenarios with a single click to inspect separate properties or financing strategies.
- **Copy & Clone**: Clone any slot into another slot with one click to test variations (e.g., adding an extra $500/month prepayment).
- **Persistent State**: All active slots automatically persist to browser `localStorage`.

---

### Parameter Input Pane (Left)

Configure the complete acquisition parameters with live validation:

#### 1. Property Information
- **Property Name**: Identifier for the scenario (e.g., *"Suburban Single Family"*).
- **Purchase Price**: Total acquisition price.
- **Annual Property Tax Rate**: Annual property tax percentage (e.g., `1.25%`).
- **Annual Homeowner's Insurance**: Estimated annual insurance premium.
- **Monthly HOA Fees**: Monthly Homeowners Association dues (if applicable).
- **Annual Maintenance Rate**: Estimated annual upkeep percentage (e.g., `1.00%`).

#### 2. Financing Structure & Capital Stack
Mix and match financing tools to fund 100% of the purchase price:
- **Conventional Mortgage**:
  - Loan Amount & Down Payment ($ or %).
  - Annual Interest Rate (Fixed APR) & Loan Term (15-year, 30-year, or custom months).
- **Line of Credit (LOC / HELOC)**:
  - Credit Facility Amount & Variable Interest Rate.
  - Draw Period & Repayment Period structures.
- **Cash Down Payment**:
  - Equity capital allocated from liquid reserves.
- **Live Capital Stack Bar**: Visual color-coded allocation bar showing current funding percentage, remaining shortfall, or over-allocation errors.

#### 3. Prepayment Schedule Editor
- Add custom accelerated principal prepayments:
  - **One-time Lump Sums**: Inject extra capital at a specific target month.
  - **Recurring Velocity Schedules**: Monthly extra principal over custom start months and duration spans.

---

### Analysis Workspaces (Right)

Switch seamlessly between four dedicated workspaces via the right canvas navigation bar:

#### 📊 1. Dashboard (Overview)
- **Structural Foundation Cards**: Property overview, initial capital required, and total debt service.
- **Initial Monthly Payment Breakdown**: Granular visual breakdown of Month 1 P&I, Property Taxes, Insurance, and HOA dues.
- **Lifetime Financial Highlights**:
  - Total Lifetime Gross Outlay.
  - Total Interest Paid vs. Principal.
  - Estimated Payoff Date & Term Acceleration.
  - Tax Deductions Realized & Waste Ratio (non-equity costs vs. total outlay).
- **3-Stage Progression Milestone**: Interactive cards illustrating how monthly payment composition shifts across **Month 1 (Initial)**, **Midpoint**, and **Final Payoff Month**.

#### 📈 2. Charts (Visual Analytics)
- **Interactive Balance Trajectory Chart**:
  - Responsive SVG trajectory illustrating mortgage and LOC balance paydown over time.
  - Integrated **Touch/Hover Scrubber** displaying month-by-month balance, cumulative interest paid, and remaining debt.
- **Lifetime Cost Breakdown**:
  - Proportional donut chart detailing Principal, Total Interest Paid, Property Taxes, Home Insurance, and HOA Fees over the entire lifespan.
- **Annual Cash Outflow**:
  - Bar visualization of annual debt service vs. holding costs across every year of ownership.

#### 📑 3. Statement Ledger (Statements)
- **Monthly & Yearly Amortization Schedules**:
  - Complete month-by-month and year-by-year financial ledger tables.
  - Columns: Period, Beginning Balance, Scheduled Payment, Principal Paid, Interest Paid, Extra Prepayments, Holding Costs, Tax Savings, Ending Balance.
- **Horizontal Scrolling & Mobile Touch Navigation**: Built-in touch momentum scrolling.
- **CSV Export**: One-click export of complete monthly and yearly schedules to `.csv` spreadsheets.

#### ⚖️ 4. Compare View (Scenario Differential Analysis)
- **Pairwise Differential ($\Delta = B - A$)**:
  - Select any two slots (e.g., **Slot A: Baseline 30-Year** vs. **Slot B: 15-Year with Prepayments**).
- **Differential KPI Cards**:
  - 💳 **Gross Outlay Delta**: Lifetime total cash saved or added.
  - 📉 **Interest Paid Delta**: Total interest reduction achieved.
  - 📊 **Present Value Delta**: Net present value differential discounted at standard market rate (6.5%).
  - 📈 **Strategy IRR**: Internal Rate of Return realized on accelerated equity.
- **House Parity Guard**:
  - Ensures mathematical rigor by validating that property characteristics (purchase price, tax rates, insurance) match identically before computing strategy IRR on incremental cash flows.
- **4-Column Metric Differential Table**: Side-by-side metric comparison covering Payoff Timeline, Extra Principal, Interest Paid, Cash Yield, Tax Deductions, and Net Outlay.

---

### Scenario Management

- **Scenario Library**: Built-in scenario library with pre-configured templates (e.g., *Conventional 30-Year 20% Down*, *15-Year Accelerated Equity*, *Hybrid Mortgage + LOC*, *All-Cash Acquisition*).
- **Save Custom Presets**: Save current configurations directly into your browser library.
- **JSON Export & Import**:
  - Export single scenarios or the entire 3-slot workspace state to standard `.json` files.
  - Import existing `.json` scenario definitions with automatic schema validation.

---

## 💻 Interactive CLI Guide

HomeCalc includes a native terminal interface featuring interactive menus, keyboard shortcuts, and formatted tables.

### Running the CLI

Launch the interactive CLI from the repository root:

```bash
cargo run -p cli
```

### CLI Menu Navigation

- `1`-`9`: Instant numerical option selection.
- `↑` / `k` and `↓` / `j`: Navigate up/down through menu items.
- `Enter` / `Space`: Confirm highlighted option.
- `Esc` / `q`: Back / Cancel / Exit.

### CLI Workflow

```
================================================================================
                           🏡 Homecalc CLI v2.1.1                               
================================================================================
> 1. 📝 Create Scenario
  2. 📂 Load Scenario
  3. 📊 Compare Scenarios
  4. 🚪 Exit
```

1. **📝 Create Scenario**: Guided wizard to configure property price, taxes, insurance, HOA, financing tools (Mortgage/LOC/Cash), and prepayment rules.
2. **📂 Load Scenario**: Browse and load saved `.json` scenario files from the `scenarios/` directory.
3. **📅 View Statement**: Render formatted monthly and yearly amortization tables powered by `comfy-table`.
4. **🔍 View Analysis**: Inspect detailed financial ratios, waste ratios, and holding cost projections.
5. **📊 Compare Scenarios**: Select two scenario files to output a comprehensive ANSI-colored differential matrix.

---

## 🛠️ Local Setup & Development

### Prerequisites

Ensure the following toolchains are installed:
- **Rust**: 1.85.0 or newer ([rustup.rs](https://rustup.rs/))
- **Node.js**: 20.x or newer & `npm` ([nodejs.org](https://nodejs.org/))
- **wasm-pack**: WebAssembly build tool (`cargo install wasm-pack`)

---

### Quick Start

Run the automated build and preview script from the repository root:

```bash
chmod +x run.sh
./run.sh
```

This compiles the web production bundle and launches the local Vite server at `http://localhost:5173`.

---

### Building Rust Crates

Build the entire workspace (engine, engine-wasm, cli):

```bash
# Check compilation across all crates
cargo check --workspace

# Build optimized release binaries
cargo build --workspace --release

# Run native terminal CLI
cargo run -p cli
```

---

### Compiling WebAssembly

To compile the `engine-wasm` crate for browser consumption:

```bash
wasm-pack build --target web --out-dir ../../web/src/lib/wasm crates/engine-wasm
```

This compiles `crates/engine-wasm` into optimized `.wasm` binaries accompanied by TypeScript definitions (`engine_wasm.d.ts`) inside `web/src/lib/wasm/`.

---

### Running the Svelte 5 Web App

```bash
# Navigate to web directory
cd web

# Install dependencies
npm install

# Start Vite development server (with HMR)
npm run dev

# Type check Svelte components & TypeScript
npm run check

# Build production distribution
npm run build

# Preview production build
npm run preview
```

---

### Running Test Suites

HomeCalc maintains comprehensive unit, integration, parity, and end-to-end test suites:

```bash
# 1. Run all Rust engine, CLI, and integration tests
cargo test --workspace

# 2. Run WASM engine parity tests
wasm-pack test --headless --firefox crates/engine-wasm

# 3. Run Svelte TypeScript type checks
cd web && npm run check

# 4. Run Playwright E2E verification test suite
cd web && npm run test:e2e
```

---

## 📐 Mathematical Precision & Algorithms

HomeCalc is built on strict financial formulas with zero floating-point drift:

### 1. Monthly Amortization Formula
For a principal $P$, annual interest rate $r$, and loan term in months $N$:

$$M = P \cdot \frac{\frac{r}{12} \cdot \left(1 + \frac{r}{12}\right)^N}{\left(1 + \frac{r}{12}\right)^N - 1}$$

For each month $t$:
- Monthly Interest: $I_t = B_{t-1} \cdot \frac{r}{12}$
- Scheduled Principal: $P_t = M - I_t$
- Total Principal Paid: $P_{\text{total}, t} = P_t + \text{ExtraPrepayment}_t$
- Ending Balance: $B_t = B_{t-1} - P_{\text{total}, t}$

### 2. Present Value of Outflows (PV)
Given monthly cash outflow $C_t$ and annual discount rate $r_{\text{discount}} = 6.5\%$:

$$\text{PV} = \sum_{t=0}^{T} \frac{C_t}{\left(1 + \frac{r_{\text{discount}}}{12}\right)^t}$$

### 3. Strategy Internal Rate of Return (IRR)
Calculates the effective rate $r^*$ satisfying Net Present Value of incremental cash flows $\Delta C_t = C_{A, t} - C_{B, t}$ (plus terminal equity differential at horizon $T$):

$$\text{NPV}(r^*) = \sum_{t=0}^{T} \frac{\Delta C_t}{(1 + r^*)^t} = 0$$

The engine utilizes an initial **Newton-Raphson** iteration bounded within monthly rate bracket $[-0.99, 1.0]$ with an automatic fallback to **Bisection Search** to guarantee numerical convergence for non-standard cash flow curves.

---

## 📄 License & Authors

- **Author**: Yanjun Wu ([wuyj1996@gmail.com](mailto:wuyj1996@gmail.com))
- **License**: Released under the [MIT License](LICENSE).
