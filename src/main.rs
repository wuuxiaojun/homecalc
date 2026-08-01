// src/main.rs

mod display;
mod formula;
mod mortgage;

use display::print_mortgage_summary;
use mortgage::Mortgage;

fn main() {
    println!("🚀 Running Mortgage Engine Verification Tests...\n");

    // Scenario 1: Standard 15-Year Fixed Mortgage ($1.5M Home, $300k Down Payment)
    println!("--- Test 1: Standard Mortgage (No Extra Payments) ---");
    match Mortgage::new(1_500_000.0, 300_000.0, 5.9, 15) {
        Ok(standard_loan) => {
            print_mortgage_summary(&standard_loan);
        }
        Err(e) => eprintln!("Error creating mortgage: {:?}", e),
    }

    // Scenario 2: Dynamic Extra Principal Lump Sums
    println!("--- Test 2: Accelerated Payoff with Extra Principal ---");
    match Mortgage::new(1_500_000.0, 300_000.0, 5.9, 15) {
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

    // Scenario 3: Testing Safeguards & Boundaries
    println!("--- Test 3: Safeguard & Boundary Verification ---");
    match Mortgage::new(1_500_000.0, 300_000.0, 5.9, 15) {
        Ok(mut loan) => {
            println!("\n1. Testing Overpayment Capping:");
            // Attempting to pay $2,000,000 extra on a $1.2M loan
            let _ = loan.add_extra_payment(1, 2_000_000.0);

            println!("\n2. Testing Payment Past Payoff Month:");
            // Attempting to pay extra at Month 100 (loan was paid off in Month 1!)
            let _ = loan.add_extra_payment(100, 10_000.0);
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
