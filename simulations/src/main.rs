//! main.rs
//! Standalone 715-Scenario Financial Simulation Engine
//! Evaluates $1.7M Property Purchase across Cash, LOC, Mortgage allocations and Prepayment Velocities.

use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::scenario::Scenario;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use engine::service::comparison::{calculate_strategy_irr, extract_monthly_outflow};
use engine::service::simulation::create_scenario;
use engine::service::utility::clamp_zero;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const PROPERTY_PRICE: f64 = 1_700_000.0;
const PROPERTY_TAX_RATE: f64 = 1.20; // $1,700/mo ($20,400/yr)
const ANNUAL_INSURANCE: f64 = 2_400.0; // $200/mo
const MONTHLY_HOA: f64 = 100.0; // $100/mo

const DISCOUNT_RATE: f64 = 0.038; // 3.8% annual cash yield / discount rate
const MORTGAGE_RATE: f64 = 6.55; // 6.55% 30-year fixed
const LOC_RATE: f64 = 5.55; // 5.55% Line of Credit
const MORTGAGE_TERM: u32 = 30;

#[derive(Debug, Clone, Copy)]
struct VelocityTier {
    name: &'static str,
    quarterly_amount: f64,
}

const VELOCITY_TIERS: [VelocityTier; 5] = [
    VelocityTier {
        name: "V0",
        quarterly_amount: 0.0,
    },
    VelocityTier {
        name: "V1",
        quarterly_amount: 5_000.0,
    },
    VelocityTier {
        name: "V2",
        quarterly_amount: 15_000.0,
    },
    VelocityTier {
        name: "V3",
        quarterly_amount: 40_000.0,
    },
    VelocityTier {
        name: "V4",
        quarterly_amount: 100_000.0,
    },
];

#[derive(Debug, Clone)]
struct ScenarioResult {
    id: usize,
    name: String,
    cash_down: f64,
    loc_initial: f64,
    mortgage_initial: f64,
    velocity_tier: &'static str,
    quarterly_extra: f64,
    payoff_month: u32,
    total_nominal_paid: f64,
    total_interest_paid: f64,
    total_tax_savings: f64,
    pv_outflows: f64,
    irr_vs_baseline: Option<f64>,
}

fn format_currency(val: f64) -> String {
    let is_negative = val < 0.0;
    let abs_val = val.abs().round() as u64;
    let s = abs_val.to_string();
    let mut result = String::new();
    for (count, c) in s.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    let formatted: String = result.chars().rev().collect();
    if is_negative {
        format!("-${}", formatted)
    } else {
        format!("${}", formatted)
    }
}

fn calculate_mortgage_pmt(principal: f64, rate: f64, year: u32) -> f64 {
    if principal <= 0.0 || year == 0 {
        return 0.0;
    }
    let total_payments = (year * 12) as i32;
    if rate <= 0.0 {
        return principal / total_payments as f64;
    }
    let monthly_rate = (rate * 0.01) / 12.0;
    let factor = (1.0 + monthly_rate).powi(total_payments);
    principal * (monthly_rate * factor) / (factor - 1.0)
}

fn generate_repayment_schedule(
    initial_loc: f64,
    initial_mtg: f64,
    quarterly_extra: f64,
) -> (BTreeMap<u32, f64>, BTreeMap<u32, f64>) {
    let mut loc_repay = BTreeMap::new();
    let mut mtg_repay = BTreeMap::new();

    let mut current_loc = initial_loc;
    let mut current_mtg = initial_mtg;

    let monthly_mtg_rate = (MORTGAGE_RATE * 0.01) / 12.0;
    let mtg_pmt = if initial_mtg > 0.0 {
        calculate_mortgage_pmt(initial_mtg, MORTGAGE_RATE, MORTGAGE_TERM)
    } else {
        0.0
    };

    for month in 1..=360 {
        if current_loc <= 0.0 && current_mtg <= 0.0 {
            break;
        }

        // 1. Scheduled Mortgage Amortization
        let sched_principal = if current_mtg > 0.0 {
            let interest = current_mtg * monthly_mtg_rate;
            let interest_paid = interest.min(mtg_pmt);
            (mtg_pmt - interest_paid).min(current_mtg).max(0.0)
        } else {
            0.0
        };
        let mtg_after_sched = (current_mtg - sched_principal).max(0.0);

        // 2. Minimum Monthly LOC principal: initial_loc / 360
        let min_loc_principal = if initial_loc > 0.0 && current_loc > 0.0 {
            (initial_loc / 360.0).min(current_loc)
        } else {
            0.0
        };
        let loc_after_min = (current_loc - min_loc_principal).max(0.0);

        // 3. Extra quarterly principal (t % 3 == 0): LOC first, then Mortgage
        let (extra_loc, extra_mtg) = if month % 3 == 0 && quarterly_extra > 0.0 {
            let to_loc = quarterly_extra.min(loc_after_min);
            let loc_rem = loc_after_min - to_loc;
            let _ = loc_rem;
            let remaining_cash = quarterly_extra - to_loc;
            let to_mtg = remaining_cash.min(mtg_after_sched);
            (to_loc, to_mtg)
        } else {
            (0.0, 0.0)
        };

        let total_loc_month = min_loc_principal + extra_loc;
        if total_loc_month > 0.0 {
            loc_repay.insert(month, total_loc_month);
        }
        if extra_mtg > 0.0 {
            mtg_repay.insert(month, extra_mtg);
        }

        current_loc = (loc_after_min - extra_loc).max(0.0);
        current_mtg = (mtg_after_sched - extra_mtg).max(0.0);
    }

    (loc_repay, mtg_repay)
}

fn calculate_pv_at_rate(scenario: &Scenario, annual_discount_rate: f64) -> f64 {
    let monthly_r = annual_discount_rate / 12.0;
    let mut total_pv = 0.0;
    let base = 1.0 + monthly_r;

    for (month_idx, row) in scenario.monthly_statement.iter().enumerate() {
        let net_outflow = extract_monthly_outflow(scenario, month_idx);
        let m = row.month as i32;
        let discount_factor = base.powi(m);
        total_pv += net_outflow / discount_factor;
    }

    clamp_zero(total_pv)
}

fn build_purchase(
    name: String,
    cash: f64,
    loc: f64,
    mtg: f64,
    quarterly_extra: f64,
) -> Purchase {
    let house = House {
        purchase_price: PROPERTY_PRICE,
        annual_property_tax_rate: PROPERTY_TAX_RATE,
        annual_insurance: ANNUAL_INSURANCE,
        monthly_hoa: MONTHLY_HOA,
    };

    let mut tools = Vec::new();
    tools.push(Tool::Cash(Cash {
        amount: cash,
        rate: DISCOUNT_RATE * 100.0, // 3.8%
    }));

    if mtg > 0.0 {
        tools.push(Tool::Mortgage(Mortgage {
            amount: mtg,
            rate: MORTGAGE_RATE,
            term: MORTGAGE_TERM,
        }));
    }

    if loc > 0.0 {
        tools.push(Tool::Loc(Loc {
            amount: loc,
            rate: LOC_RATE,
        }));
    }

    let (loc_repay, mortgage_repay) = generate_repayment_schedule(loc, mtg, quarterly_extra);

    Purchase {
        name,
        house,
        tools,
        mortgage_repay,
        loc_repay,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===============================================================================");
    println!("  HOMECALC STANDALONE RESEARCH ENGINE: 715 SCENARIO MULTI-DIMENSIONAL GRID    ");
    println!("===============================================================================");
    println!("Property Price:        {}", format_currency(PROPERTY_PRICE));
    println!("Property Tax:          1.20% ({}/mo)", format_currency((PROPERTY_PRICE * 0.012) / 12.0));
    println!("Insurance:             {}/yr ({}/mo)", format_currency(ANNUAL_INSURANCE), format_currency(ANNUAL_INSURANCE / 12.0));
    println!("HOA:                   {}/mo", format_currency(MONTHLY_HOA));
    println!("Discount / Cash Rate:  {:.2}%", DISCOUNT_RATE * 100.0);
    println!("Mortgage Note Rate:    {:.2}% (30-Year Fixed)", MORTGAGE_RATE);
    println!("LOC Note Rate:         {:.2}%", LOC_RATE);
    println!("-------------------------------------------------------------------------------");

    // 1. Build Baseline Scenario ($300k Cash, $1,400k Mtg, $0 LOC, V0)
    let baseline_purchase = build_purchase(
        "Baseline (C300k_L0k_M1400k_V0)".to_string(),
        300_000.0,
        0.0,
        1_400_000.0,
        0.0,
    );
    let baseline_scenario = create_scenario(baseline_purchase);
    let baseline_pv = calculate_pv_at_rate(&baseline_scenario, DISCOUNT_RATE);

    let baseline_payoff = baseline_scenario.monthly_statement.last().map_or(0, |r| r.month);
    println!("Baseline Initialized:  C=$300k | LOC=$0k | Mtg=$1,400k | Velocity=V0");
    println!("Baseline Payoff Month: {} months ({:.1} years)", baseline_payoff, baseline_payoff as f64 / 12.0);
    println!("Baseline Nominal Paid: {}", format_currency(baseline_scenario.total_statement.total_paid));
    println!("Baseline Total Int:    {}", format_currency(baseline_scenario.total_statement.total_interest_paid));
    println!("Baseline PV Outflows:  {} (at 3.8% discount)", format_currency(baseline_pv));
    println!("===============================================================================\n");

    // 2. Generate all 715 scenarios
    let mut results: Vec<ScenarioResult> = Vec::with_capacity(715);
    let mut scenario_counter = 0;

    for cash_step in 0..=10 {
        let cash = cash_step as f64 * 100_000.0;
        let max_loc = PROPERTY_PRICE - cash;
        let max_loc_steps = (max_loc / 100_000.0).round() as usize;

        for loc_step in 0..=max_loc_steps {
            let loc = loc_step as f64 * 100_000.0;
            let mtg = max_loc - loc;

            for velocity in &VELOCITY_TIERS {
                scenario_counter += 1;
                let name = format!(
                    "C{:.0}k_L{:.0}k_M{:.0}k_{}",
                    cash / 1_000.0,
                    loc / 1_000.0,
                    mtg / 1_000.0,
                    velocity.name
                );

                let purchase = build_purchase(name.clone(), cash, loc, mtg, velocity.quarterly_amount);
                let scenario = create_scenario(purchase);

                let payoff_month = scenario.monthly_statement.last().map_or(0, |r| r.month);
                let total_nominal_paid = scenario.total_statement.total_paid;
                let total_interest_paid = scenario.total_statement.total_interest_paid;
                let total_tax_savings = scenario.total_statement.total_tax_savings;
                let pv_outflows = calculate_pv_at_rate(&scenario, DISCOUNT_RATE);

                let irr_vs_baseline = calculate_strategy_irr(&baseline_scenario, &scenario);

                results.push(ScenarioResult {
                    id: scenario_counter,
                    name,
                    cash_down: cash,
                    loc_initial: loc,
                    mortgage_initial: mtg,
                    velocity_tier: velocity.name,
                    quarterly_extra: velocity.quarterly_amount,
                    payoff_month,
                    total_nominal_paid,
                    total_interest_paid,
                    total_tax_savings,
                    pv_outflows,
                    irr_vs_baseline,
                });
            }
        }
    }

    println!("Total Scenarios Computed: {}\n", results.len());

    // 3. Export all results to CSV
    let csv_path = Path::new("simulations/results.csv");
    let file = File::create(csv_path)?;
    let mut writer = BufWriter::new(file);

    writeln!(
        writer,
        "Rank,Scenario ID,Name,Cash Down ($),LOC Initial ($),Mortgage Initial ($),Velocity Tier,Quarterly Extra ($),Payoff Month,Payoff Years,Nominal Outflows ($),Total Interest ($),Tax Savings ($),PV Outflows @ 3.8% ($),Delta PV vs Baseline ($),Strategy IRR vs Baseline (%)"
    )?;

    // Sort by PV ascending (lowest cost first)
    let mut sorted_by_pv = results.clone();
    sorted_by_pv.sort_by(|a, b| a.pv_outflows.partial_cmp(&b.pv_outflows).unwrap());

    for (rank, res) in sorted_by_pv.iter().enumerate() {
        let delta_pv = res.pv_outflows - baseline_pv;
        let irr_str = match res.irr_vs_baseline {
            Some(val) => format!("{:.2}%", val * 100.0),
            None => "N/A".to_string(),
        };

        writeln!(
            writer,
            "{},{},{},{:.0},{:.0},{:.0},{},{:.0},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{}",
            rank + 1,
            res.id,
            res.name,
            res.cash_down,
            res.loc_initial,
            res.mortgage_initial,
            res.velocity_tier,
            res.quarterly_extra,
            res.payoff_month,
            res.payoff_month as f64 / 12.0,
            res.total_nominal_paid,
            res.total_interest_paid,
            res.total_tax_savings,
            res.pv_outflows,
            delta_pv,
            irr_str
        )?;
    }
    writer.flush()?;
    println!("✓ Successfully exported all 715 scenario records to `{}`\n", csv_path.display());

    // 4. Output Formatted Tables
    print_top_scenarios(&sorted_by_pv, baseline_pv);
    print_top_moderate_scenarios(&sorted_by_pv, baseline_pv);
    print_insights(&sorted_by_pv, baseline_pv);

    Ok(())
}

fn print_top_scenarios(sorted_by_pv: &[ScenarioResult], baseline_pv: f64) {
    println!("==========================================================================================================================");
    println!("                                   🏆 TOP 15 BEST SCENARIOS (RANKED BY LOWEST PV OF OUTFLOWS)                              ");
    println!("==========================================================================================================================");
    println!(
        "{:<4} | {:<22} | {:<7} | {:<7} | {:<7} | {:<4} | {:<8} | {:<14} | {:<12} | {:<14} | {:<10}",
        "Rank", "Scenario Name", "Cash", "LOC", "Mtg", "Tier", "Payoff", "Nominal Paid", "Total Int", "PV @ 3.8%", "IRR vs Base"
    );
    println!("--------------------------------------------------------------------------------------------------------------------------");

    for (i, res) in sorted_by_pv.iter().take(15).enumerate() {
        let irr_str = match res.irr_vs_baseline {
            Some(val) => format!("{:>8.2}%", val * 100.0),
            None => format!("{:>8}", "N/A"),
        };
        let payoff_str = format!("{:.1} yrs", res.payoff_month as f64 / 12.0);

        println!(
            "#{:<3} | {:<22} | ${:>5.0}k | ${:>5.0}k | ${:>5.0}k | {:<4} | {:<8} | {:>14} | {:>12} | {:>14} | {:<10}",
            i + 1,
            res.name,
            res.cash_down / 1_000.0,
            res.loc_initial / 1_000.0,
            res.mortgage_initial / 1_000.0,
            res.velocity_tier,
            payoff_str,
            format_currency(res.total_nominal_paid),
            format_currency(res.total_interest_paid),
            format_currency(res.pv_outflows),
            irr_str
        );
    }
    println!("--------------------------------------------------------------------------------------------------------------------------");
    let best = &sorted_by_pv[0];
    println!(
        "💡 Top #1 Strategy ({}) saves {} in Present Value (-{:.1}%) vs Baseline ({})\n",
        best.name,
        format_currency(baseline_pv - best.pv_outflows),
        ((baseline_pv - best.pv_outflows) / baseline_pv) * 100.0,
        format_currency(baseline_pv)
    );
}

fn print_top_moderate_scenarios(sorted_by_pv: &[ScenarioResult], baseline_pv: f64) {
    println!("==========================================================================================================================");
    println!("                         🎯 TOP 5 BEST MODERATE VELOCITY SCENARIOS (VELOCITY TIERS V1 / V2: $5k-$15k/QTR)                 ");
    println!("==========================================================================================================================");
    println!(
        "{:<4} | {:<22} | {:<7} | {:<7} | {:<7} | {:<4} | {:<8} | {:<14} | {:<12} | {:<14} | {:<10}",
        "Rank", "Scenario Name", "Cash", "LOC", "Mtg", "Tier", "Payoff", "Nominal Paid", "Total Int", "PV @ 3.8%", "IRR vs Base"
    );
    println!("--------------------------------------------------------------------------------------------------------------------------");

    let moderate: Vec<&ScenarioResult> = sorted_by_pv
        .iter()
        .filter(|s| s.velocity_tier == "V1" || s.velocity_tier == "V2")
        .take(5)
        .collect();

    for (i, res) in moderate.iter().enumerate() {
        let irr_str = match res.irr_vs_baseline {
            Some(val) => format!("{:>8.2}%", val * 100.0),
            None => format!("{:>8}", "N/A"),
        };
        let payoff_str = format!("{:.1} yrs", res.payoff_month as f64 / 12.0);

        println!(
            "#{:<3} | {:<22} | ${:>5.0}k | ${:>5.0}k | ${:>5.0}k | {:<4} | {:<8} | {:>14} | {:>12} | {:>14} | {:<10}",
            i + 1,
            res.name,
            res.cash_down / 1_000.0,
            res.loc_initial / 1_000.0,
            res.mortgage_initial / 1_000.0,
            res.velocity_tier,
            payoff_str,
            format_currency(res.total_nominal_paid),
            format_currency(res.total_interest_paid),
            format_currency(res.pv_outflows),
            irr_str
        );
    }
    println!("--------------------------------------------------------------------------------------------------------------------------");
    if let Some(best_mod) = moderate.first() {
        println!(
            "💡 Top Moderate Strategy ({}) saves {} in PV (-{:.1}%) vs Baseline with achievable ${:.0}k/qtr prepayments.\n",
            best_mod.name,
            format_currency(baseline_pv - best_mod.pv_outflows),
            ((baseline_pv - best_mod.pv_outflows) / baseline_pv) * 100.0,
            best_mod.quarterly_extra / 1_000.0
        );
    }
}

fn print_insights(sorted_by_pv: &[ScenarioResult], baseline_pv: f64) {
    println!("==========================================================================================================================");
    println!("                                             📊 STRATEGIC SIMULATION INSIGHTS                                             ");
    println!("==========================================================================================================================");

    // 1. Tool selection impact (LOC 5.55% vs Mortgage 6.55%)
    let v0_all_mtg = sorted_by_pv.iter().find(|s| s.cash_down == 300_000.0 && s.loc_initial == 0.0 && s.velocity_tier == "V0").unwrap();
    let v0_all_loc = sorted_by_pv.iter().find(|s| s.cash_down == 300_000.0 && s.mortgage_initial == 0.0 && s.velocity_tier == "V0").unwrap();

    let loc_savings = v0_all_mtg.pv_outflows - v0_all_loc.pv_outflows;
    println!("1. SPREAD ARBITRAGE (LOC @ 5.55% vs Mortgage @ 6.55%):");
    println!(
        "   - Pure LOC (C300k, LOC $1.4M, Mtg $0, V0) PV: {} vs Baseline (Mtg $1.4M) PV: {}",
        format_currency(v0_all_loc.pv_outflows), format_currency(v0_all_mtg.pv_outflows)
    );
    println!(
        "   - Replacing $1.4M Mortgage with LOC produces a PV savings of {} (-{:.1}%) due to the 100 bps interest rate advantage and minimum monthly principal amortization.",
        format_currency(loc_savings),
        (loc_savings / v0_all_mtg.pv_outflows) * 100.0
    );

    // 2. Cash Down Efficiency
    println!("\n2. CASH DOWN PAYMENT OPPORTUNITY COST (Yielding 3.8% Compound):");
    println!(
        "   - Baseline (C300k, Mtg $1.4M) PV: {}",
        format_currency(baseline_pv)
    );
    println!(
        "   - Retaining cash in reserve earning 3.8% compound yield significantly reduces net present value outflows compared to large upfront cash down payments at t=0."
    );
    println!(
        "   - Zero/low cash down strategies dominate PV rankings because the unspent cash capital remains fully invested, generating compounding interest that offsets monthly debt servicing."
    );


    // 3. Prepayment Velocity Dominance
    let best = &sorted_by_pv[0];
    let worst = sorted_by_pv.last().unwrap();
    println!("\n3. PREPAYMENT VELOCITY SENSITIVITY:");
    println!(
        "   - Best Scenario:  {} (PV: {}, Payoff: {:.1} yrs, Total Int: {})",
        best.name, format_currency(best.pv_outflows), best.payoff_month as f64 / 12.0, format_currency(best.total_interest_paid)
    );
    println!(
        "   - Worst Scenario: {} (PV: {}, Payoff: {:.1} yrs, Total Int: {})",
        worst.name, format_currency(worst.pv_outflows), worst.payoff_month as f64 / 12.0, format_currency(worst.total_interest_paid)
    );
    println!(
        "   - Total PV Spread across the 715 scenario parameter grid is {}.",
        format_currency(worst.pv_outflows - best.pv_outflows)
    );
    println!("==========================================================================================================================\n");
}
