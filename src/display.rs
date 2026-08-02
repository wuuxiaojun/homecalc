// src/display.rs

use crate::analysis::ComparisonReport;
use crate::mortgage::Mortgage;
use std::io::{self, Write};

/// Clears the terminal screen buffer cleanly
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

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

    let crossover_str = match mortgage.crossover_month() {
        Some(m) => format!("Month {} (Yr {:.1})", m, m as f64 / 12.0),
        None => "N/A".to_string(),
    };
    let half_equity_str = match mortgage.half_equity_month() {
        Some(m) => format!("Month {} (Yr {:.1})", m, m as f64 / 12.0),
        None => "N/A".to_string(),
    };

    let l8 = format!("Crossover Month:  {}", crossover_str);
    let r8 = format!("50% Equity Month:  {}", half_equity_str);

    let box_border = "===============================================================================================================";
    let box_divider = "---------------------------------------------------------------------------------------------------------------";

    println!("\n{}", box_border);
    println!(" 🏠 SCENARIO: {}", mortgage.name.to_uppercase());
    println!("    FULL HOUSING COST ENGINE (PITI + ESCROW SUMMARY)");
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
    println!(" {:<44} | {}", l8, r8);
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

/// Prints a 4-column side-by-side comparison report between Option A and Option B
pub fn print_comparison_report(
    report: &ComparisonReport,
    title_a: &str,
    title_b: &str,
) {
    let box_border = "===============================================================================================================";
    let box_divider = "---------------------------------------------------------------------------------------------------------------";

    let col_a_title = format!("A: {}", title_a);
    let col_b_title = format!("B: {}", title_b);

    println!("{}", box_border);
    println!(" ⚖️ MORTGAGE SCENARIO COMPARISON REPORT (OPTION A vs. OPTION B)");
    println!("{}", box_border);
    println!(
        "{:<30} | {:<24} | {:<24} | {:<24}",
        "Financial Metric", col_a_title, col_b_title, "Delta (Option B - A)"
    );
    println!("{}", box_divider);

    // Section 1: Upfront Cash Needed
    println!(" --- 1. UPFRONT CASH NEEDED ---");
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | ${:<23.2}",
        "Down Payment",
        report.option_a.down_payment,
        report.option_b.down_payment,
        report.option_b.down_payment - report.option_a.down_payment
    );
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | ${:<23.2}",
        "Closing Prepaids & Reserves",
        report.option_a.upfront_prepaids,
        report.option_b.upfront_prepaids,
        report.option_b.upfront_prepaids - report.option_a.upfront_prepaids
    );
    let delta_cash_fmt = format_delta_currency(report.delta_cash_to_close);
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {:<24}",
        "Total Cash to Close",
        report.option_a.total_cash_to_close,
        report.option_b.total_cash_to_close,
        delta_cash_fmt
    );
    println!("{}", box_divider);

    // Section 2: Monthly Cash Outflow
    println!(" --- 2. MONTHLY CASH OUTFLOW ---");
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | ${:<23.2}",
        "Monthly Principal & Interest",
        report.option_a.monthly_p_i,
        report.option_b.monthly_p_i,
        report.option_b.monthly_p_i - report.option_a.monthly_p_i
    );
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | ${:<23.2}",
        "Monthly Escrow (Tax + Ins)",
        report.option_a.monthly_escrow,
        report.option_b.monthly_escrow,
        report.option_b.monthly_escrow - report.option_a.monthly_escrow
    );
    let delta_piti_fmt = format_delta_currency(report.delta_monthly_piti);
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {:<24}",
        "Total Monthly PITI Outflow",
        report.option_a.monthly_piti,
        report.option_b.monthly_piti,
        delta_piti_fmt
    );
    println!("{}", box_divider);

    // Section 3: Loan Timeline & Equity
    println!(" --- 3. LOAN TIMELINE & EQUITY ---");
    let a_months_fmt = format!(
        "{} mos ({:.1} yrs)",
        report.option_a.actual_payoff_months,
        report.option_a.actual_payoff_months as f64 / 12.0
    );
    let b_months_fmt = format!(
        "{} mos ({:.1} yrs)",
        report.option_b.actual_payoff_months,
        report.option_b.actual_payoff_months as f64 / 12.0
    );
    let delta_months_fmt = if report.delta_payoff_months > 0 {
        format!(
            "+{} mos (+{:.1} yrs)",
            report.delta_payoff_months,
            report.delta_payoff_months as f64 / 12.0
        )
    } else if report.delta_payoff_months < 0 {
        format!(
            "{} mos ({:.1} yrs)",
            report.delta_payoff_months,
            report.delta_payoff_months as f64 / 12.0
        )
    } else {
        "0 mos".to_string()
    };
    println!(
        "{:<30} | {:<24} | {:<24} | {:<24}",
        "Payoff Duration", a_months_fmt, b_months_fmt, delta_months_fmt
    );

    let delta_eq_fmt = format_delta_currency(report.delta_5yr_equity);
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {:<24}",
        "5-Year Equity Built",
        report.option_a.equity_at_5_years,
        report.option_b.equity_at_5_years,
        delta_eq_fmt
    );
    println!("{}", box_divider);

    // Section 4: Lifetime Financial Cost
    println!(" --- 4. LIFETIME FINANCIAL COST ---");
    let delta_int_fmt = format_delta_currency(report.delta_total_interest);
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {:<24}",
        "Total Interest Paid",
        report.option_a.total_interest_paid,
        report.option_b.total_interest_paid,
        delta_int_fmt
    );
    let delta_outflow_fmt = format_delta_currency(report.delta_lifetime_outflow);
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {:<24}",
        "Total Housing Outflow",
        report.option_a.total_lifetime_outflow,
        report.option_b.total_lifetime_outflow,
        delta_outflow_fmt
    );
    println!("{}\n", box_border);
}

fn format_delta_currency(delta: f64) -> String {
    if delta > 0.0 {
        format!("+${:.2}", delta)
    } else if delta < 0.0 {
        format!("-${:.2}", delta.abs())
    } else {
        "$0.00".to_string()
    }
}
