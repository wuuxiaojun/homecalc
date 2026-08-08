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
            purchase_price: 800_000.0,
            annual_property_tax_rate: 1.2, // 1.2%
            annual_insurance: 2_400.0,
            monthly_hoa: 300.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 160_000.0, // Down payment: 20%
                rate: 4.5,         // Cash annual interest rate: 4.5%
            }),
            Tool::Mortgage(Mortgage {
                amount: 640_000.0, // Mortgage amount: 80%
                rate: 6.5,         // Interest rate: 6.5%
                term: 30,          // 30 years
            }),
        ],
        mortgage_repay: BTreeMap::from([
            (6, 5_000.0),  // $5,000 extra principal payment at Month 6
            (12, 10_000.0), // $10,000 extra principal payment at Month 12
        ]),
        loc_repay: BTreeMap::new(),
    };

    println!("=========================================================================================");
    println!(" Scenario: {}", scenario.name);
    println!(" House Purchase Price: ${:.2}", scenario.house.purchase_price);
    println!("=========================================================================================");

    // Run simulation
    let schedule = simulation(&scenario);

    println!("\nTotal Months Simulated: {}\n", schedule.len());
    println!(
        "{:<6} | {:<12} | {:<12} | {:<12} | {:<12} | {:<15} | {:<15}",
        "Month", "Mortgage PMT", "Principal", "Interest", "Extra Pay", "Rem Balance", "Total Paid"
    );
    println!("{}", "-".repeat(98));

    // Print first 12 months of simulation
    for row in schedule.iter().take(12) {
        if let Some(m) = &row.mortgage {
            println!(
                "{:<6} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<14.2} | ${:<14.2}",
                row.month,
                m.monthly_payment,
                m.principal_paid,
                m.interest_paid,
                m.extra_payment,
                m.remaining_balance,
                row.total_paid
            );
        }
    }

    if schedule.len() > 12 {
        println!("... [{} months truncated for preview]", schedule.len() - 12);
        if let Some(last_row) = schedule.last() {
            println!("{}", "-".repeat(98));
            if let Some(m) = &last_row.mortgage {
                println!(
                    "Final Month {:<3}: Mortgage Rem Bal = ${:.2}, Total Debt Paid = ${:.2}",
                    last_row.month, m.remaining_balance, last_row.total_debt_paid
                );
            } else {
                println!(
                    "Final Month {:<3}: Remaining Balance = ${:.2}",
                    last_row.month, last_row.total_remaining_balance
                );
            }
        }
    }
}
