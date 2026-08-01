fn calculate_monthly_payment(principal: f64, interest: f64, years: u32) -> f64 {
    if interest <= 0.0 {
        return principal / (years * 12) as f64;
    }

    let r = interest / 100.0 / 12.0;
    let n = (years * 12) as f64;
    let factor = (1.0 + r).powf(n);

    principal * (r * factor) / (factor - 1.0)
}

fn main() {
    let principal: f64 = 1_500_000.0;
    let interest: f64 = 5.9;
    let years: u32 = 15;

    let monthly_payment = calculate_monthly_payment(principal, interest, years);
    let total_cost = monthly_payment * (years * 12) as f64;
    let total_interest = total_cost - principal;

    println!("--- Phase 1: Mortgage Math Proof of Concept ---");
    println!("Loan Principal:     ${:.2}", principal);
    println!("Annual Interest:    {:.2}%", interest);
    println!("Term Length:        {} years", years);
    println!("-----------------------------------------------");
    println!("Monthly Payment:    ${:.2}", monthly_payment);
    println!("Total Cost:         ${:.2}", total_cost);
    println!("Total Interest:     ${:.2}", total_interest);
}
