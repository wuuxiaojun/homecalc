use engine::domain::house::House;
use engine::domain::scenario::Scenario;
use engine::domain::tool::{Cash, Mortgage, Tool};
use engine::service::simulate::simulation;
use std::collections::BTreeMap;

fn main() {
    // Sample Scenario with a House, Cash, and Mortgage with scheduled extra payments
    let scenario = Scenario {
        name: "Sample 30-Year Mortgage with Extra Payments".to_string(),
        house: House {
            purchase_price: 1500_000.0,
            annual_property_tax_rate: 1.2, // 1.2%
            annual_insurance: 3_600.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 500_000.0,
                rate: 3.9,
            }),
            Tool::Mortgage(Mortgage {
                amount: 1_000_000.0,
                rate: 5.9,
                term: 15,
            }),
        ],
        mortgage_repay: BTreeMap::from([
            (3, 100_000.0),
            (6, 100_000.0),
            (9, 100_000.0),
            (12, 100_000.0),
            (15, 100_000.0),
        ]),
        loc_repay: BTreeMap::new(),
    };

    println!(
        "========================================================================================="
    );
    println!(" Scenario: {}", scenario.name);
    println!(
        " House Purchase Price: ${:.2}",
        scenario.house.purchase_price
    );
    println!(
        "========================================================================================="
    );

    // Run simulation
    let schedule = simulation(&scenario);

    println!("\nTotal Months Simulated: {}\n", schedule.len());
    println!(
        "{:<6} | {:<12} | {:<12} | {:<12} | {:<12} | {:<15} | {:<15}",
        "Month", "Mortgage PMT", "Principal", "Interest", "Extra Pay", "Rem Balance", "Total Paid"
    );
    println!("{}", "-".repeat(98));

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
        let mortgage_pmt = row.mortgage.as_ref().map_or(0.0, |m| m.monthly_payment);
        let principal_paid = row.mortgage.as_ref().map_or(0.0, |m| m.principal_paid);
        let interest_paid = row.mortgage.as_ref().map_or(0.0, |m| m.interest_paid);
        let extra_payment = row.total_extra_payment;
        let rem_balance = row.total_remaining_balance;

        println!(
            "{:<6} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<14.2} | ${:<14.2}",
            row.month,
            mortgage_pmt,
            principal_paid,
            interest_paid,
            extra_payment,
            rem_balance,
            row.total_paid
        );
    }
}
