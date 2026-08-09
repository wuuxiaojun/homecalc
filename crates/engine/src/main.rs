use engine::domain::house::House;
use engine::domain::scenario::Scenario;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use engine::service::simulate::simulate_monthly;
use std::collections::BTreeMap;

fn main() {
    // Sample Scenario with House, Cash, Mortgage, and Line of Credit (LOC)
    let scenario = Scenario {
        name: "Sample Purchase with Cash, Mortgage & Line of Credit".to_string(),
        house: House {
            purchase_price: 1_500_000.0,
            annual_property_tax_rate: 1.2, // 1.2%
            annual_insurance: 3_600.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 300_000.0, // $300,000 cash down payment
                rate: 3.9,         // 3.9% annual yield
            }),
            Tool::Mortgage(Mortgage {
                amount: 1_000_000.0, // $1,000,000 mortgage
                rate: 5.9,           // 5.9% interest rate
                term: 15,            // 15 years
            }),
            Tool::Loc(Loc {
                amount: 200_000.0, // $200,000 Line of Credit
                rate: 7.5,         // 7.5% interest rate
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

    println!(
        "=================================================================================================================================================="
    );
    println!(" Scenario: {}", scenario.name);
    println!(
        " House Purchase Price: ${:.2}",
        scenario.house.purchase_price
    );
    println!(" Tools: Cash ($300k @ 3.9%), Mortgage ($1M @ 5.9%, 15yr), LOC ($200k @ 7.5%)");
    println!(
        "=================================================================================================================================================="
    );

    // Run simulation
    let schedule = simulate_monthly(&scenario);

    println!("\nTotal Months Simulated: {}\n", schedule.len());
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

    // Print schedule: first 6 rows, any row with extra payments (mortgage or LOC), and last 3 rows
    let total_len = schedule.len();
    let mut indices_to_print = Vec::new();

    for (i, row) in schedule.iter().enumerate() {
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

        let row = &schedule[idx];
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
}
