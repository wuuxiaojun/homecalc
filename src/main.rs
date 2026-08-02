// src/main.rs

mod analysis;
mod display;
mod formula;
mod mortgage;

use analysis::compare_mortgages;
use display::{print_annual_summary_table, print_comparison_report, print_mortgage_summary};
use mortgage::Mortgage;

fn main() {
    println!("🚀 Running Full Housing Engine Verification Tests (Phase 4.2)...\n");

    // $1.5M Home, $300k Down (Loan $1.2M), 5.9% Interest, 15 Years, 1.2% Property Tax, $3,600/yr Insurance
    let price = 1_500_000.0;
    let down = 300_000.0;
    let rate = 5.9;
    let term = 15;
    let tax_rate = 1.2;
    let insurance = 3_600.0;

    let standard_loan = match Mortgage::new(price, down, rate, term, tax_rate, insurance) {
        Ok(loan) => loan,
        Err(e) => {
            eprintln!("Error creating baseline mortgage: {:?}", e);
            return;
        }
    };

    let mut accelerated_loan = match Mortgage::new(price, down, rate, term, tax_rate, insurance) {
        Ok(loan) => loan,
        Err(e) => {
            eprintln!("Error creating accelerated mortgage: {:?}", e);
            return;
        }
    };

    println!("--- Scenario 1: Standard Mortgage + Escrow (No Extra Payments) ---");
    print_mortgage_summary(&standard_loan);
    print_annual_summary_table(&standard_loan);

    println!("--- Scenario 2: Accelerated Payoff with Escrow ---");
    println!("Adding $50,000 extra principal at Month 1...");
    let _ = accelerated_loan.add_extra_payment(1, 50_000.0);

    println!("Adding $100,000 extra principal at Month 12...");
    let _ = accelerated_loan.add_extra_payment(12, 100_000.0);

    println!("Adding $200,000 extra principal at Month 24...");
    let _ = accelerated_loan.add_extra_payment(24, 200_000.0);

    print_mortgage_summary(&accelerated_loan);
    print_annual_summary_table(&accelerated_loan);

    println!("--- Scenario 3: Side-by-Side Comparison Analysis ---");
    let report = compare_mortgages(&standard_loan, &accelerated_loan);
    print_comparison_report(
        &report,
        "Standard 15-Year Mortgage (No Extra Payments)",
        "Accelerated 15-Year Mortgage ($350k Extra Principal)",
    );
}
