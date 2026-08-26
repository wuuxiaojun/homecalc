# 715-Scenario Financial Simulation: Strategic Acquisition Analysis

**Property**: $1,700,000 Residential Property  
**Engine**: `Homecalc` Rust Engine v2.1.1  
**Simulation Dataset**: [`results.csv`](./results.csv) (715 scenario records)  
**Execution Binary**: [`simulations/src/main.rs`](./src/main.rs)

---

## 1. Executive Summary

This study executes a full-grid deterministic financial optimization over **715 unique purchase and debt repayment configurations** for a **$1,700,000 property**.

The optimization grid models multi-tool allocations across **Cash Down Payment**, **Line of Credit (LOC)**, and **30-Year Fixed Mortgage**, combined with **5 quarterly extra prepayment velocity tiers** ($V_0$ to $V_4$).

### Key Findings at a Glance

- **Optimal Strategy (`C0k_L1100k_M600k_V0`)**: Achieves a **Present Value (PV) of Outflows of $1,451,577**, saving **$413,775 in PV (-22.2%)** and **$628,957 in nominal cash** compared to the conventional baseline.
- **Top Moderate Strategy (`C0k_L900k_M800k_V1`)**: Achieves a **PV of $1,471,913** (a **$393,439 PV savings**, **-21.1%**), accelerating full debt-free payoff from 30.0 years down to **26.2 years** with an accessible prepayment of only **$5,000/quarter** ($1,667/month).
- **Decision Spread**: Across the 715 scenario parameter space, the present value spread between the optimal strategy ($1,451,577) and the least efficient strategy ($2,432,184) is **$980,607**.

---

## 2. Simulation Parameters & Methodology

### 2.1 Property & Financial Baseline Assumptions

| Parameter | Value | Notes |
| :--- | :--- | :--- |
| **Purchase Price** | **$1,700,000.00** | Full transaction price |
| **Property Tax** | **1.20% / yr** | $1,700.00 / month ($20,400.00 / year) |
| **Homeowners Insurance** | **$2,400.00 / yr** | $200.00 / month |
| **HOA Fee** | **$100.00 / mo** | $1,200.00 / year |
| **Cash Yield / Discount Rate** | **3.80% / yr** | Compounded monthly (0.3167% / mo) |
| **30-Year Fixed Mortgage Rate** | **6.55% / yr** | Compounded monthly (0.5458% / mo) |
| **Line of Credit (LOC) Rate** | **5.55% / yr** | Compounded monthly (0.4625% / mo) |
| **Standard Starting Cash Pool** | **$1,000,000.00** | Retained capital earns 3.8% compound yield |

### 2.2 Grid Generation (715 Total Scenarios)

The simulation spans two orthogonal dimensions:

1. **Capital Allocation Dimension (143 Combinations)**:
   - **Cash Down ($C$)**: $0 to $1,000k (step: $100k, 11 tiers).
   - **Line of Credit ($L$)**: $0 to $(1,700k - C)$ (step: $100k).
   - **Mortgage ($M$)**: $1,700k - C - L$.

2. **Prepayment Velocity Dimension (5 Tiers per Allocation)**:
   - **$V_0$ (Baseline)**: $0 extra quarterly principal.
   - **$V_1$ (Conservative)**: $5,000 extra quarterly principal ($1,667/mo).
   - **$V_2$ (Moderate)**: $15,000 extra quarterly principal ($5,000/mo).
   - **$V_3$ (Aggressive)**: $40,000 extra quarterly principal ($13,333/mo).
   - **$V_4$ (Hyper-Velocity)**: $100,000 extra quarterly principal ($33,333/mo).

### 2.3 Repayment Priority & Amortization Rules

- **Scheduled Mortgage**: Standard 30-year amortization schedule ($\text{PMT} = \$8,896.78/\text{mo}$ per $\$1.4\text{M}$).
- **Minimum Monthly LOC Principal**: Equal principal reduction of $\text{Initial LOC} / 360$ each month to ensure non-negative amortization.
- **Extra Prepayment Waterfall**: Quarterly extra cash pays down LOC balance to $\$0$ first (taking advantage of flexible principal reduction), then automatically rolls into Mortgage principal.
- **Payoff Termination**: When debt balances reach $\$0$, debt service terminates immediately, and holding outflows revert strictly to property tax, insurance, and HOA.

---

## 3. Baseline Scenario Reference

The standard benchmark scenario reflects conventional home-buying practice:

- **Structure**: **$300k Cash Down (17.6%)** + **$1.4M 30-Year Mortgage (6.55%)** + **$0 LOC** + **$0 Extra Prepayments ($V_0$)**
- **Payoff Horizon**: 360 months (30.0 years)
- **Nominal Total Outflows**: **$2,767,460**
- **Total Interest Paid**: **$1,802,214**
- **Present Value of Outflows (@ 3.8% discount)**: **$1,865,352**

---

## 4. Top 15 Best Scenarios (Ranked by Lowest PV of Outflows)

All present values are discounted at the **3.80%** annual benchmark rate.

| Rank | Scenario Name | Cash Down | Initial LOC | Initial Mtg | Velocity | Payoff | Nominal Paid | Total Interest | PV of Outflows (@ 3.8%) | Delta PV vs Base | Strategy IRR |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| 🥇 **#1** | `C0k_L1100k_M600k_V0` | $0 | $1,100,000 | $600,000 | V0 ($0) | 30.0 yrs | $2,138,503 | $1,690,671 | **$1,451,577** | **-$413,775** | N/A |
| 🥈 **#2** | `C0k_L1200k_M500k_V0` | $0 | $1,200,000 | $500,000 | V0 ($0) | 30.0 yrs | $2,124,150 | $1,645,423 | **$1,451,947** | **-$413,405** | N/A |
| 🥉 **#3** | `C0k_L1300k_M400k_V0` | $0 | $1,300,000 | $400,000 | V0 ($0) | 30.0 yrs | $2,109,797 | $1,600,174 | **$1,454,610** | **-$410,742** | N/A |
| **#4** | `C0k_L1400k_M300k_V0` | $0 | $1,400,000 | $300,000 | V0 ($0) | 30.0 yrs | $2,095,443 | $1,554,926 | **$1,457,298** | **-$408,054** | N/A |
| **#5** | `C0k_L1500k_M200k_V0` | $0 | $1,500,000 | $200,000 | V0 ($0) | 30.0 yrs | $2,081,090 | $1,509,678 | **$1,460,239** | **-$405,113** | N/A |
| **#6** | `C0k_L1000k_M700k_V0` | $0 | $1,000,000 | $700,000 | V0 ($0) | 30.0 yrs | $2,152,856 | $1,735,919 | **$1,463,043** | **-$402,309** | N/A |
| **#7** | `C0k_L1600k_M100k_V0` | $0 | $1,600,000 | $100,000 | V0 ($0) | 30.0 yrs | $2,066,737 | $1,464,430 | **$1,463,953** | **-$401,399** | N/A |
| **#8** | `C0k_L1700k_M0k_V0` | $0 | $1,700,000 | $0 | V0 ($0) | 30.0 yrs | $2,052,384 | $1,419,181 | **$1,468,550** | **-$396,802** | N/A |
| **#9** | `C0k_L900k_M800k_V1` | $0 | $900,000 | $800,000 | V1 ($5k/qtr) | 26.2 yrs | $2,050,297 | $1,404,876 | **$1,471,913** | **-$393,439** | N/A |
| **#10** | `C0k_L1000k_M700k_V1` | $0 | $1,000,000 | $700,000 | V1 ($5k/qtr) | 26.1 yrs | $2,036,164 | $1,360,410 | **$1,472,946** | **-$392,406** | N/A |
| **#11** | `C0k_L900k_M800k_V0` | $0 | $900,000 | $800,000 | V0 ($0) | 30.0 yrs | $2,169,289 | $1,781,168 | **$1,479,362** | **-$385,990** | N/A |
| **#12** | `C0k_L1100k_M600k_V1` | $0 | $1,100,000 | $600,000 | V1 ($5k/qtr) | 26.0 yrs | $2,022,651 | $1,313,595 | **$1,482,552** | **-$382,800** | N/A |
| **#13** | `C0k_L800k_M900k_V1` | $0 | $800,000 | $900,000 | V1 ($5k/qtr) | 26.0 yrs | $2,086,392 | $1,445,987 | **$1,482,763** | **-$382,589** | N/A |
| **#14** | `C0k_L800k_M900k_V0` | $0 | $800,000 | $900,000 | V0 ($0) | 30.0 yrs | $2,195,776 | $1,826,416 | **$1,495,694** | **-$369,658** | **+0.86%** |
| **#15** | `C0k_L1200k_M500k_V1` | $0 | $1,200,000 | $500,000 | V1 ($5k/qtr) | 25.8 yrs | $2,016,591 | $1,269,919 | **$1,498,228** | **-$367,124** | N/A |

---

## 5. Top 5 Moderate Velocity Scenarios ($5k–$15k / Quarter)

For buyers seeking a realistic, moderate acceleration plan without hyper-aggressive cash outflow requirements:

| Rank | Scenario Name | Cash Down | Initial LOC | Initial Mtg | Velocity | Payoff | Nominal Paid | Total Interest | PV of Outflows (@ 3.8%) | Delta PV vs Base |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **#1** | `C0k_L900k_M800k_V1` | $0 | $900,000 | $800,000 | V1 ($5k/qtr) | 26.2 yrs | $2,050,297 | $1,404,876 | **$1,471,913** | **-$393,439** |
| **#2** | `C0k_L1000k_M700k_V1` | $0 | $1,000,000 | $700,000 | V1 ($5k/qtr) | 26.1 yrs | $2,036,164 | $1,360,410 | **$1,472,946** | **-$392,406** |
| **#3** | `C0k_L1100k_M600k_V1` | $0 | $1,100,000 | $600,000 | V1 ($5k/qtr) | 26.0 yrs | $2,022,651 | $1,313,595 | **$1,482,552** | **-$382,800** |
| **#4** | `C0k_L800k_M900k_V1` | $0 | $800,000 | $900,000 | V1 ($5k/qtr) | 26.0 yrs | $2,086,392 | $1,445,987 | **$1,482,763** | **-$382,589** |
| **#5** | `C0k_L1200k_M500k_V1` | $0 | $1,200,000 | $500,000 | V1 ($5k/qtr) | 25.8 yrs | $2,016,591 | $1,269,919 | **$1,498,228** | **-$367,124** |

---

## 6. Strategic Analysis & Core Economic Drivers

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       FINANCIAL ARBITRAGE MECHANICS                         │
├──────────────────────────────────────┬──────────────────────────────────────┤
│ 1. LOC Interest Spread Advantage     │ 5.55% LOC vs. 6.55% Mortgage (100bp) │
│ 2. Cash Compounding Opportunity Cost │ 3.80% compounding on $1M cash pool   │
│ 3. Prepayment Velocity Shield        │ Eliminates compounding interest tail │
└──────────────────────────────────────┴──────────────────────────────────────┘
```

### 6.1 The 100 bps LOC Spread Arbitrage
The Line of Credit offers a **100 basis point rate advantage (5.55% vs. 6.55%)** over the 30-year fixed mortgage:
- When comparing an identical $300k Cash Down + $0 Prepayment structure, transitioning from a pure Mortgage (`C300k_L0k_M1400k_V0`, Baseline) to a pure LOC (`C300k_L1400k_M0k_V0`) lowers total nominal interest from **$1,802,214** to **$1,168,738**—a direct interest savings of **$633,476** and a **$114,964 PV reduction (-6.2%)**.

### 6.2 The Cash Down Opportunity Cost Dynamic
A critical finding of the Present Value analysis is the **high opportunity cost of upfront cash**:
- Paying $1,000,000 cash down at $t=0$ incurs a 100% nominal present value impact immediately ($\text{PV} = \$1,000,000$) and completely depletes the cash reserve.
- In contrast, retaining $1,000,000 in cash earning **3.80% compound yield** generates substantial monthly interest cash inflows that offset mortgage and LOC payments over time.
- Consequently, **zero-cash-down and low-cash-down allocations consistently dominate the top 20 rankings** in Net Present Value terms.

### 6.3 Prepayment Velocity vs. Capital Preservation
- Moving from **$V_0$ ($0/qtr)** to **$V_1$ ($5k/qtr)** cuts the loan payoff horizon by nearly **4.0 years** (from 30.0 to 26.0 years) and reduces total nominal interest by **$377,000+**.
- Hyper-velocity prepayments ($V_4$: $100k/qtr) accelerate payoff to under 4.0 years, but require heavy liquidity outlays that shift cash flow burdens into the earliest years.

---

## 7. Actionable Recommendations

1. **Optimal Hybrid Debt Structure**: Blend a **$1.1M–$1.2M Line of Credit** with a **$500k–$600k 30-Year Fixed Mortgage** to maximize rate arbitrage while retaining the stability of a fixed amortization backbone.
2. **Preserve Cash Liquidity**: Avoid putting excessive equity down if safe cash vehicles yield ~3.80% or higher, as the compounding shield on retained cash provides higher net present value efficiency.
3. **Implement Conservative Prepayment Schedules ($V_1$)**: Dedicate $5,000 per quarter ($1,667/month) to debt acceleration to save ~$400k in nominal interest without sacrificing emergency operational liquidity.

---

## 8. Dataset Verification

The full simulation dataset containing all 715 scenario records, metrics, and rank orders is available in:
- File: [`simulations/results.csv`](./results.csv)
- Total Records: 715
- Re-run Command: `cargo run -p simulations`
