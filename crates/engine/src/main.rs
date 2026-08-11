use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::scenario::Scenario;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use engine::service::analysis::{ScenarioAnalysis, compute_scenario_analysis};
use engine::service::simulation::{aggregate_yearly, compute_metrics, simulate_monthly};
use std::collections::BTreeMap;

fn create_scenario(purchase: Purchase) -> Scenario {
    let monthly_statement = simulate_monthly(&purchase);
    let yearly_statement = aggregate_yearly(&monthly_statement);
    let total_statement = compute_metrics(&yearly_statement);

    Scenario {
        purchase,
        monthly_statement,
        yearly_statement,
        total_statement,
    }
}

fn print_scenario(scenario: &Scenario) {
    println!(
        "\n=================================================================================================================================================="
    );
    println!(" PURCHASE SCENARIO: {}", scenario.purchase.name);
    println!(
        " House Purchase Price: ${:.2}",
        scenario.purchase.house.purchase_price
    );
    println!(
        "=================================================================================================================================================="
    );

    // 1. Monthly Schedule
    println!(
        "\n--- MONTHLY STATEMENT SCHEDULE (Total Months: {}) ---\n",
        scenario.monthly_statement.len()
    );
    println!(
        "{:<5} | {:<12} | {:<11} | {:<11} | {:<12} | {:<10} | {:<10} | {:<11} | {:<12} | {:<12} | {:<13}",
        "Month",
        "Cash Bal",
        "Mortg PMT",
        "Mortg Extra",
        "Mortg Bal",
        "LOC PMT",
        "LOC Extra",
        "LOC Bal",
        "Holding Cost",
        "Total Paid",
        "Total Rem Bal"
    );
    println!("{}", "-".repeat(146));

    let total_len = scenario.monthly_statement.len();
    let mut indices_to_print = Vec::new();

    for (i, row) in scenario.monthly_statement.iter().enumerate() {
        let is_first_6 = i < 6;
        let is_last_3 = i >= total_len.saturating_sub(3);
        let has_extra_payment = row.total_extra_payment > 0.0;

        if is_first_6 || is_last_3 || has_extra_payment {
            indices_to_print.push(i);
        }
    }

    let mut last_printed_idx: Option<usize> = None;

    for &idx in &indices_to_print {
        if let Some(prev) = last_printed_idx {
            if idx > prev + 1 {
                let skipped = idx - prev - 1;
                println!("... [{} month(s) skipped]", skipped);
            }
        }
        last_printed_idx = Some(idx);

        let row = &scenario.monthly_statement[idx];
        let cash_bal = row.cash.as_ref().map_or(0.0, |c| c.cash_now);
        let mortg_pmt = row.mortgage.as_ref().map_or(0.0, |m| {
            (m.principal_paid + m.interest_paid).min(m.monthly_payment)
        });
        let mortg_extra = row.mortgage.as_ref().map_or(0.0, |m| m.extra_payment);
        let mortg_bal = row.mortgage.as_ref().map_or(0.0, |m| m.remaining_balance);

        let loc_pmt = row.loc.as_ref().map_or(0.0, |l| l.monthly_payment);
        let loc_extra = row.loc.as_ref().map_or(0.0, |l| l.extra_payment);
        let loc_bal = row.loc.as_ref().map_or(0.0, |l| l.remaining_balance);

        let holding_cost = row.total_holding_cost;
        let total_paid = row.total_paid;
        let rem_balance = row.total_remaining_balance;

        println!(
            "{:<5} | ${:<11.2} | ${:<10.2} | ${:<10.2} | ${:<11.2} | ${:<9.2} | ${:<9.2} | ${:<10.2} | ${:<11.2} | ${:<11.2} | ${:<12.2}",
            row.month,
            cash_bal,
            mortg_pmt,
            mortg_extra,
            mortg_bal,
            loc_pmt,
            loc_extra,
            loc_bal,
            holding_cost,
            total_paid,
            rem_balance
        );
    }

    // 2. Yearly Statement
    println!("\n--- YEARLY SUMMARY STATEMENT ---");
    println!(
        "{:<5} | {:<13} | {:<14} | {:<12} | {:<12} | {:<13} | {:<12} | {:<15} | {:<15}",
        "Year",
        "Cash Yield",
        "Interest Paid",
        "Debt Paid",
        "Extra Paid",
        "Holding Cost",
        "Tax Savings",
        "Net Annual Paid",
        "Ending Rem Bal"
    );
    println!("{}", "-".repeat(133));

    for y in &scenario.yearly_statement {
        println!(
            "{:<5} | ${:<12.2} | ${:<13.2} | ${:<11.2} | ${:<11.2} | ${:<12.2} | ${:<11.2} | ${:<14.2} | ${:<14.2}",
            y.year,
            y.annual_cash_interest,
            y.annual_interest_paid,
            y.annual_debt_paid,
            y.annual_extra_payment,
            y.annual_holding_cost,
            y.annual_tax_savings,
            y.annual_paid,
            y.ending_remaining_balance
        );
    }

    // 3. Total Statement
    let t = &scenario.total_statement;
    println!("\n--- TOTAL STATEMENT SUMMARY ---");
    println!("Total Cash Yield Earned:  ${:<14.2}", t.total_cash_interest);
    println!("Total Interest Paid:      ${:<14.2}", t.total_interest_paid);
    println!("Total Holding Cost Paid:  ${:<14.2}", t.total_holding_cost);
    println!("Total Tax Savings:        ${:<14.2}", t.total_tax_savings);
    println!("Total Net Cash Paid:      ${:<14.2}", t.total_paid);
}

fn print_analysis(name: &str, analysis: &ScenarioAnalysis) {
    println!("\n--------------------------------------------------");
    println!(" SCENARIO ANALYSIS RESULTS: {}", name);
    println!("--------------------------------------------------");
    println!(" Payoff Month:              {}", analysis.payoff_month);
    println!(
        " Effective Monthly Cost:   ${:.2}",
        analysis.effective_monthly_cost
    );
    println!(
        " Waste Ratio (Int/Borrow):  {:.2}%",
        analysis.waste_ratio * 100.0
    );
    println!(
        " Tax Savings Ratio:        {:.2}%",
        analysis.tax_savings_ratio * 100.0
    );
}

fn main() {
    // Scenario A: Standard 30-Year Mortgage + Cash Down Payment (No extra payments)
    let purchase_a = Purchase {
        name: "Scenario A: Standard 30-Yr Mortgage + Cash Down (Baseline)".to_string(),
        house: House {
            purchase_price: 1_500_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 3_600.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 300_000.0,
                rate: 3.9,
            }),
            Tool::Mortgage(Mortgage {
                amount: 1_200_000.0,
                rate: 6.0,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    // Scenario B: Hybrid Cash + Mortgage + LOC with Accelerated Extra Payments
    let purchase_b = Purchase {
        name: "Scenario B: Cash + Mortgage + LOC (Accelerated Extra Pay)".to_string(),
        house: House {
            purchase_price: 1_500_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 3_600.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 300_000.0,
                rate: 3.9,
            }),
            Tool::Mortgage(Mortgage {
                amount: 1_000_000.0,
                rate: 5.9,
                term: 15,
            }),
            Tool::Loc(Loc {
                amount: 200_000.0,
                rate: 7.5,
            }),
        ],
        mortgage_repay: BTreeMap::from([
            (3, 100_000.0),
            (6, 100_000.0),
            (9, 100_000.0),
            (12, 100_000.0),
            (15, 100_000.0),
        ]),
        loc_repay: BTreeMap::from([
            (6, 50_000.0),
            (12, 50_000.0),
            (18, 50_000.0),
            (24, 50_000.0),
        ]),
    };

    // Create full Scenarios
    let scenario_a = create_scenario(purchase_a);
    let scenario_b = create_scenario(purchase_b);

    // Print Statements for Scenario A and B
    print_scenario(&scenario_a);
    print_scenario(&scenario_b);

    // Compute and print Single Scenario Analysis for A & B
    let analysis_a = compute_scenario_analysis(&scenario_a);
    let analysis_b = compute_scenario_analysis(&scenario_b);

    println!(
        "\n=================================================================================================================================================="
    );
    println!(" SINGLE SCENARIO ANALYSIS COMPARISON");
    println!(
        "=================================================================================================================================================="
    );
    print_analysis(&scenario_a.purchase.name, &analysis_a);
    print_analysis(&scenario_b.purchase.name, &analysis_b);

    // Compute and print Strategy IRR
    let irr = calculate_strategy_irr(&scenario_a, &scenario_b);

    println!(
        "\n=================================================================================================================================================="
    );
    println!(" STRATEGY IRR COMPARISON (Baseline: A, Alternative: B)");
    println!(
        "=================================================================================================================================================="
    );
    match irr {
        Some(rate) => println!(
            " Calculated Strategy IRR (Annualized): {:.2}%",
            rate * 100.0
        ),
        None => println!(" Strategy IRR: Failed to converge (or non-convergent cash flows)"),
    }
}
