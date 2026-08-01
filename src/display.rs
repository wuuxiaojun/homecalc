// src/display.rs

use crate::mortgage::Mortgage;

/// Prints a high-level summary and formatted amortization schedule to the terminal.
pub fn print_mortgage_summary(mortgage: &Mortgage) {
    let total_months = mortgage.schedule.len() as u32;
    let total_years = total_months as f64 / 12.0;

    // Sum total interest and principal from computed schedule
    let mut total_interest_paid = 0.0;
    let mut total_principal_paid = 0.0;
    let mut total_extra_paid = 0.0;

    for payment in mortgage.schedule.values() {
        total_interest_paid += payment.interest;
        total_principal_paid += payment.principal;
        total_extra_paid += payment.extra;
    }

    let total_paid = total_interest_paid + total_principal_paid + total_extra_paid;

    println!(
        "\n========================================================================================="
    );
    println!(" 🏠 MORTGAGE SUMMARY & AMORTIZATION SCHEDULE");
    println!(
        "========================================================================================="
    );
    println!(
        " Home Price:       ${:<12.2} | Down Payment:     ${:<12.2} ({:.1}%)",
        mortgage.price,
        mortgage.down,
        (mortgage.down / mortgage.price) * 100.0
    );
    println!(
        " Loan Principal:   ${:<12.2} | Interest Rate:    {:.2}%",
        mortgage.loan, mortgage.rate
    );
    println!(
        " Original Term:    {:<12} yrs | Base Monthly P&I: ${:.2}",
        mortgage.term, mortgage.base_payment
    );
    println!(
        "-----------------------------------------------------------------------------------------"
    );
    println!(
        " Actual Payoff:    {} months ({:.1} yrs) | Total Interest:   ${:.2}",
        total_months, total_years, total_interest_paid
    );
    println!(
        " Extra Principal:  ${:<12.2} | Total Cost:       ${:.2}",
        total_extra_paid, total_paid
    );
    println!(
        "========================================================================================="
    );

    // Print Schedule Header
    println!(
        "{:<6} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14}",
        "Month", "Payment", "Principal", "Interest", "Extra Paid", "Rem. Balance"
    );
    println!(
        "-----------------------------------------------------------------------------------------"
    );

    // Sort schedule keys to print months sequentially (1..=N)
    let mut months: Vec<u32> = mortgage.schedule.keys().copied().collect();
    months.sort_unstable();

    for &month in &months {
        if let Some(entry) = mortgage.schedule.get(&month) {
            // Display rule: Print first 6 months, last 3 months, or any month with an extra payment
            let is_early_month = month <= 6;
            let is_late_month = month > total_months.saturating_sub(3);
            let has_extra = entry.extra > 0.0;

            if is_early_month || is_late_month || has_extra {
                println!(
                    "{:<6} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<13.2}",
                    entry.month,
                    entry.total,
                    entry.principal,
                    entry.interest,
                    entry.extra,
                    entry.balance
                );
            } else if month == 7 {
                println!(
                    "  ...  |      ...     |      ...     |      ...     |      ...     |       ...      "
                );
            }
        }
    }
    println!(
        "=========================================================================================\n"
    );
}
