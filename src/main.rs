// src/main.rs

mod analysis;
mod display;
mod formula;
mod mortgage;

use analysis::compare_mortgages;
use display::{print_annual_summary_table, print_comparison_report, print_mortgage_summary};
use mortgage::Mortgage;

fn main() {
    println!("🚀 Running Full Housing Engine Verification Tests...\n");

    let price = 1_500_000.0;
    let tax_rate = 1.2;
    let insurance = 3_600.0;

    // Option A: 15-Year Fixed @ 5.5% ($300k Down)
    let option_a = match Mortgage::new(price, 300_000.0, 5.5, 15, tax_rate, insurance) {
        Ok(loan) => loan,
        Err(e) => {
            eprintln!("Error creating Option A mortgage: {:?}", e);
            return;
        }
    };

    // Option B: 30-Year Fixed @ 6.25% ($150k Down)
    let option_b = match Mortgage::new(price, 150_000.0, 6.25, 30, tax_rate, insurance) {
        Ok(loan) => loan,
        Err(e) => {
            eprintln!("Error creating Option B mortgage: {:?}", e);
            return;
        }
    };

    println!("--- Option A: 15-Year Fixed @ 5.5% ($300k Down) ---");
    print_mortgage_summary(&option_a);
    print_annual_summary_table(&option_a);

    println!("--- Option B: 30-Year Fixed @ 6.25% ($150k Down) ---");
    print_mortgage_summary(&option_b);
    print_annual_summary_table(&option_b);

    println!("--- Product Comparison: 15-Year Fixed vs. 30-Year Fixed ---");
    let report = compare_mortgages(&option_a, &option_b);
    print_comparison_report(
        &report,
        "15-Yr / $300k Down",
        "30-Yr / $150k Down",
    );
}
