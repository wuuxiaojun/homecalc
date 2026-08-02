// src/display.rs

use crate::analysis::ComparisonReport;
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

/// Prints a year-by-year annual rollup summary table to the terminal.
pub fn print_annual_summary_table(mortgage: &Mortgage) {
    let summaries = mortgage.annual_summaries();
    let box_border = "===============================================================================================================";
    let box_divider = "---------------------------------------------------------------------------------------------------------------";

    println!("{}", box_border);
    println!(" 📅 ANNUAL AGGREGATION SUMMARY (YEAR-BY-YEAR ROLLUP)");
    println!("{}", box_border);
    println!(
        "{:<6} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} | {:<13}",
        "Year", "Principal", "Extra Paid", "Interest", "Escrow", "Total Outflow", "End Balance"
    );
    println!("{}", box_divider);

    for s in &summaries {
        println!(
            "{:<6} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<11.2} | ${:<13.2} | ${:<12.2}",
            format!("Yr {}", s.year),
            s.principal_paid,
            s.extra_principal_paid,
            s.interest_paid,
            s.escrow_paid,
            s.total_outflow,
            s.year_end_balance
        );
    }
    println!("{}\n", box_border);
}

/// Prints a side-by-side comparison report between two mortgage scenarios to the terminal.
pub fn print_comparison_report(
    report: &ComparisonReport,
    baseline_title: &str,
    accel_title: &str,
) {
    let box_border = "===============================================================================================================";
    let box_divider = "---------------------------------------------------------------------------------------------------------------";

    let b_months_str = format!("{} months ({:.1} yrs)", report.baseline_months, report.baseline_months as f64 / 12.0);
    let a_months_str = format!("{} months ({:.1} yrs)", report.accelerated_months, report.accelerated_months as f64 / 12.0);
    let saved_months_str = format!("{} months ({:.1} yrs) saved", report.months_saved, report.years_saved);

    let b_interest_str = format!("${:.2}", report.baseline_interest);
    let a_interest_str = format!("${:.2}", report.accelerated_interest);
    let saved_interest_str = format!("${:.2} saved", report.interest_saved);

    let b_outflow_str = format!("${:.2}", report.baseline_outflow);
    let a_outflow_str = format!("${:.2}", report.accelerated_outflow);
    let saved_outflow_str = format!("${:.2} saved", report.total_outflow_saved);

    println!("{}", box_border);
    println!(" ⚖️ MORTGAGE SCENARIO COMPARISON REPORT");
    println!("    Baseline:    {}", baseline_title);
    println!("    Accelerated: {}", accel_title);
    println!("{}", box_border);
    println!(
        "{:<30} | {:<25} | {:<25} | {:<22}",
        "Metric", "Baseline", "Accelerated", "Difference / Savings"
    );
    println!("{}", box_divider);
    println!(
        "{:<30} | {:<25} | {:<25} | {:<22}",
        "Payoff Duration", b_months_str, a_months_str, saved_months_str
    );
    println!(
        "{:<30} | {:<25} | {:<25} | {:<22}",
        "Total Interest Paid", b_interest_str, a_interest_str, saved_interest_str
    );
    println!(
        "{:<30} | {:<25} | {:<25} | {:<22}",
        "Total Housing Outflow", b_outflow_str, a_outflow_str, saved_outflow_str
    );
    println!("{}", box_divider);
    println!(
        " 🎉 SUMMARY SAVINGS: Paid off {:.1} years early | Saved ${:.2} in Interest | Saved ${:.2} Total Cash",
        report.years_saved, report.interest_saved, report.total_outflow_saved
    );
    println!("{}\n", box_border);
}
