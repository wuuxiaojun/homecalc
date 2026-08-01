// src/display.rs

use crate::mortgage::Mortgage;

/// Prints a high-level summary and formatted amortization schedule to the terminal.
pub fn print_mortgage_summary(mortgage: &Mortgage) {
    // 1. Sort schedule keys to iterate months sequentially (1..=N)
    let mut months: Vec<u32> = mortgage.schedule.keys().copied().collect();
    months.sort_unstable();

    // Actual active months dynamically derived from the schedule length
    let actual_months = months.len() as u32;
    let actual_years = actual_months as f64 / 12.0;

    // 2. Aggregate financial metrics directly from active schedule entries
    let mut total_interest_paid = 0.0;
    let mut total_scheduled_principal = 0.0;
    let mut total_extra_principal = 0.0;

    for payment in mortgage.schedule.values() {
        total_interest_paid += payment.interest;
        total_scheduled_principal += payment.principal;
        total_extra_principal += payment.extra;
    }

    let total_principal_paid = total_scheduled_principal + total_extra_principal;
    let total_cost = total_interest_paid + total_principal_paid;

    // 3. Render Top-Level Summary Box
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
        actual_months, actual_years, total_interest_paid
    );
    println!(
        " Extra Principal:  ${:<12.2} | Total Loan Cost:  ${:.2}",
        total_extra_principal, total_cost
    );
    println!(
        "========================================================================================="
    );

    // 4. Render Table Header
    println!(
        "{:<6} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14}",
        "Month", "Payment", "Principal", "Interest", "Extra Paid", "Rem. Balance"
    );
    println!(
        "-----------------------------------------------------------------------------------------"
    );

    // 5. Render Filtered Table Entries
    for &month in &months {
        if let Some(entry) = mortgage.schedule.get(&month) {
            // Display filter: Show first 6 months, last 3 active months, or any month with extra payment
            let is_early_month = month <= 6;
            let is_late_month = month > actual_months.saturating_sub(3);
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
