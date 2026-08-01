// src/main.rs

mod display;
mod formula;
mod mortgage;

use display::print_mortgage_summary;
use mortgage::Mortgage;

fn main() {
    println!("🚀 Running Full Housing Engine Verification Tests (Phase 3)...\n");

    // $1.5M Home, $300k Down (Loan $1.2M), 5.9% Interest, 15 Years, 1.2% Property Tax, $3,600/yr Insurance
    let price = 1_500_000.0;
    let down = 300_000.0;
    let rate = 5.9;
    let term = 15;
    let tax_rate = 1.2;
    let insurance = 3_600.0;

    println!("--- Scenario 1: Standard Mortgage + Escrow (No Extra Payments) ---");
    match Mortgage::new(price, down, rate, term, tax_rate, insurance) {
        Ok(standard_loan) => {
            print_mortgage_summary(&standard_loan);
        }
        Err(e) => eprintln!("Error creating mortgage: {:?}", e),
    }

    println!("--- Scenario 2: Accelerated Payoff with Escrow ---");
    match Mortgage::new(price, down, rate, term, tax_rate, insurance) {
        Ok(mut custom_loan) => {
            println!("Adding $50,000 extra principal at Month 1...");
            let _ = custom_loan.add_extra_payment(1, 50_000.0);

            println!("Adding $100,000 extra principal at Month 12...");
            let _ = custom_loan.add_extra_payment(12, 100_000.0);

            println!("Adding $200,000 extra principal at Month 24...");
            let _ = custom_loan.add_extra_payment(24, 200_000.0);

            print_mortgage_summary(&custom_loan);
        }
        Err(e) => eprintln!("Error creating mortgage: {:?}", e),
    }
}
