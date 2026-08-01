// src/display.rs

use crate::mortgage::Mortgage;

/// Prints a high-level summary and formatted amortization schedule to the terminal.
pub fn print_mortgage_summary(mortgage: &Mortgage) {
    let mut months: Vec<u32> = mortgage.schedule.keys().copied().collect();
    months.sort_unstable();

    let actual_months = months.len() as u32;
    let actual_years = actual_months as f64 / 12.0;

    let mut total_interest_paid = 0.0;
    let mut total_scheduled_principal = 0.0;
    let mut total_extra_principal = 0.0;
    let mut total_escrow_paid = 0.0;

    for payment in mortgage.schedule.values() {
        total_interest_paid += payment.interest;
        total_scheduled_principal += payment.principal;
        total_extra_principal += payment.extra;
        total_escrow_paid += payment.escrow;
    }

    let total_principal_paid = total_scheduled_principal + total_extra_principal;
    let total_loan_cost = total_interest_paid + total_principal_paid;
    let total_housing_outflow = total_loan_cost + total_escrow_paid;

    let (upfront_ins, escrow_buffer, total_prepaids) = mortgage.closing_prepaids();

    println!(
        "\n========================================================================================="
    );
    println!(" 🏠 FULL HOUSING COST ENGINE (PITI + ESCROW SUMMARY)");
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
        " Original Term:    {:<12} yrs | Property Tax Rate:{:.2}%/yr (${:.2}/mo)",
        mortgage.term,
        mortgage.tax_rate,
        mortgage.monthly_tax()
    );
    println!(
        " Home Insurance:   ${:<12.2}/yr| Monthly Escrow:   ${:.2}/mo",
        mortgage.annual_insurance,
        mortgage.monthly_escrow()
    );
    println!(
        "-----------------------------------------------------------------------------------------"
    );
    println!(
        " Monthly P&I:      ${:<12.2} | Total Monthly PITI:${:.2}/mo",
        mortgage.base_payment,
        mortgage.base_payment + mortgage.monthly_escrow()
    );
    println!(
        " Actual Payoff:    {} months ({:.1} yrs) | Total Interest:   ${:.2}",
        actual_months, actual_years, total_interest_paid
    );
    println!(
        " Total Escrow Paid:${:<12.2} | Total Housing Cost:${:.2}",
        total_escrow_paid, total_housing_outflow
    );
    println!(
        "-----------------------------------------------------------------------------------------"
    );
    println!(
        " CLOSING DAY PREPAIDS: Upfront Ins: ${:.2} | Escrow Buffer: ${:.2} | Total Prepaids: ${:.2}",
        upfront_ins, escrow_buffer, total_prepaids
    );
    println!(
        "========================================================================================="
    );

    // Render Table Header
    println!(
        "{:<6} | {:<12} | {:<12} | {:<12} | {:<10} | {:<12} | {:<12}",
        "Month", "P&I Total", "Principal", "Interest", "Extra", "Escrow", "Total Outflow"
    );
    println!(
        "-----------------------------------------------------------------------------------------"
    );

    for &month in &months {
        if let Some(entry) = mortgage.schedule.get(&month) {
            let is_early_month = month <= 6;
            let is_late_month = month > actual_months.saturating_sub(3);
            let has_extra = entry.extra > 0.0;

            if is_early_month || is_late_month || has_extra {
                println!(
                    "{:<6} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<9.2} | ${:<11.2} | ${:<11.2}",
                    entry.month,
                    entry.total_p_i,
                    entry.principal,
                    entry.interest,
                    entry.extra,
                    entry.escrow,
                    entry.total_outflow
                );
            } else if month == 7 {
                println!(
                    "  ...  |      ...     |      ...     |      ...     |    ...    |      ...     |      ...     "
                );
            }
        }
    }
    println!(
        "=========================================================================================\n"
    );
}
