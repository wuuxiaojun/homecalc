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

    let l1 = format!("Home Price:       ${:.2}", mortgage.price);
    let r1 = format!(
        "Down Payment:     ${:.2} ({:.1}%)",
        mortgage.down,
        (mortgage.down / mortgage.price) * 100.0
    );

    let l2 = format!("Loan Principal:   ${:.2}", mortgage.loan);
    let r2 = format!("Interest Rate:    {:.2}%", mortgage.rate);

    let l3 = format!("Original Term:    {} yrs", mortgage.term);
    let r3 = format!(
        "Property Tax Rate: {:.2}%/yr (${:.2}/mo)",
        mortgage.tax_rate,
        mortgage.monthly_tax()
    );

    let l4 = format!("Home Insurance:   ${:.2}/yr", mortgage.annual_insurance);
    let r4 = format!("Monthly Escrow:   ${:.2}/mo", mortgage.monthly_escrow());

    let l5 = format!("Monthly P&I:      ${:.2}", mortgage.base_payment);
    let r5 = format!(
        "Total Monthly PITI: ${:.2}/mo",
        mortgage.base_payment + mortgage.monthly_escrow()
    );

    let l6 = format!(
        "Actual Payoff:    {} months ({:.1} yrs)",
        actual_months, actual_years
    );
    let r6 = format!("Total Interest:   ${:.2}", total_interest_paid);

    let l7 = format!("Total Escrow Paid:${:.2}", total_escrow_paid);
    let r7 = format!("Total Housing Cost: ${:.2}", total_housing_outflow);

    let box_border = "===============================================================================================================";
    let box_divider = "---------------------------------------------------------------------------------------------------------------";

    println!("\n{}", box_border);
    println!(" 🏠 FULL HOUSING COST ENGINE (PITI + ESCROW SUMMARY)");
    println!("{}", box_border);
    println!(" {:<44} | {}", l1, r1);
    println!(" {:<44} | {}", l2, r2);
    println!(" {:<44} | {}", l3, r3);
    println!(" {:<44} | {}", l4, r4);
    println!("{}", box_divider);
    println!(" {:<44} | {}", l5, r5);
    println!(" {:<44} | {}", l6, r6);
    println!(" {:<44} | {}", l7, r7);
    println!("{}", box_divider);
    println!(
        " CLOSING DAY PREPAIDS: Upfront Ins: ${:.2} | Escrow Buffer: ${:.2} | Total Prepaids: ${:.2}",
        upfront_ins, escrow_buffer, total_prepaids
    );
    println!("{}\n", box_border);

    // Render Table Header
    println!(
        "{:<6} | {:<12} | {:<12} | {:<12} | {:<10} | {:<12} | {:<13} | {:<13}",
        "Month", "P&I Total", "Principal", "Interest", "Extra", "Escrow", "Total Outflow", "Balance"
    );
    println!("{}", box_divider);

    for &month in &months {
        if let Some(entry) = mortgage.schedule.get(&month) {
            let is_early_month = month <= 6;
            let is_late_month = month > actual_months.saturating_sub(3);
            let has_extra = entry.extra > 0.0;

            if is_early_month || is_late_month || has_extra {
                println!(
                    "{:<6} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<9.2} | ${:<11.2} | ${:<12.2} | ${:<12.2}",
                    entry.month,
                    entry.total_p_i,
                    entry.principal,
                    entry.interest,
                    entry.extra,
                    entry.escrow,
                    entry.total_outflow,
                    entry.balance
                );
            } else if month == 7 {
                println!(
                    "  ...  |      ...     |      ...     |      ...     |    ...    |      ...     |      ...      |      ...     "
                );
            }
        }
    }
    println!("{}\n", box_border);
}
