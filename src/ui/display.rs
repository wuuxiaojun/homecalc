// src/ui/display.rs

use crate::analysis::comparison::ComparisonReport;
use crate::domain::mortgage::Mortgage;
use std::io::{self, Write};

// Unified table border and divider constants (127 width)
pub const BOX_BORDER: &str = "===========================================================================================================================";
pub const BOX_DIVIDER: &str = "---------------------------------------------------------------------------------------------------------------------------";

/// Clears the terminal screen buffer cleanly
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

/// Helper function to print standardized double-line header banners
pub fn print_banner(emoji: &str, title: &str) {
    println!("{}", BOX_BORDER);
    println!(" {} {}", emoji, title.to_uppercase());
    println!("{}", BOX_BORDER);
}

/// Prints a high-level summary and formatted amortization schedule to the terminal.
pub fn print_mortgage_summary(mortgage: &Mortgage) {
    let actual_months = mortgage.schedule.len() as u32;
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

    let crossover_str = match mortgage.crossover_month() {
        Some(m) => format!("Month {} (Yr {:.1})", m, m as f64 / 12.0),
        None => "N/A".to_string(),
    };
    let half_equity_str = match mortgage.half_equity_month() {
        Some(m) => format!("Month {} (Yr {:.1})", m, m as f64 / 12.0),
        None => "N/A".to_string(),
    };

    // Plain text values for left column
    let l1_val = format!("${:.2}", mortgage.price);
    let l2_val = format!("${:.2}", mortgage.loan);
    let l3_val = format!("{} yrs", mortgage.term);
    let l4_val = format!("${:.2}/yr", mortgage.annual_insurance);
    let l5_val = format!("${:.2}", mortgage.base_payment);
    let l6_val = format!("{} mos ({:.1} yrs)", actual_months, actual_years);
    let l7_val = format!("${:.2}", total_escrow_paid);
    let l8_val = crossover_str;

    // Plain text values for right column
    let r1_val = format!(
        "${:.2} ({:.1}%)",
        mortgage.down,
        (mortgage.down / mortgage.price) * 100.0
    );
    let r2_val = format!("{:.2}%", mortgage.rate);
    let r3_val = format!(
        "{:.2}%/yr (${:.2}/mo)",
        mortgage.tax_rate,
        mortgage.monthly_tax()
    );
    let r4_val = format!("${:.2}/mo", mortgage.monthly_escrow());
    let r5_val = format!(
        "${:.2}/mo",
        mortgage.base_payment + mortgage.monthly_escrow()
    );
    let r6_val = format!("${:.2}", total_interest_paid);
    let r7_val = format!("${:.2}", total_housing_outflow);
    let r8_val = half_equity_str;

    // Formatted Left Columns (Exact 20-char label + 37-char value = 57 visual chars)
    let l1 = format!("{:<20} {:<37}", "Home Price:", l1_val);
    let l2 = format!("{:<20} {:<37}", "Loan Principal:", l2_val);
    let l3 = format!("{:<20} {:<37}", "Original Term:", l3_val);
    let l4 = format!("{:<20} {:<37}", "Home Insurance:", l4_val);
    let l5 = format!("{:<20} {:<37}", "Monthly P&I:", l5_val);
    let l6 = format!("{:<20} {:<37}", "Actual Payoff:", l6_val);
    let l7 = format!("{:<20} {:<37}", "Total Escrow Paid:", l7_val);
    let l8 = format!("{:<20} {:<37}", "Crossover Month:", l8_val);

    // Formatted Right Columns (Exact 22-char label + 44-char value)
    let r1 = format!("{:<22} {}", "Down Payment:", r1_val);
    let r2 = format!("{:<22} {}", "Interest Rate:", r2_val);
    let r3 = format!("{:<22} {}", "Property Tax Rate:", r3_val);
    let r4 = format!("{:<22} {}", "Monthly Escrow:", r4_val);
    let r5 = format!("{:<22} {}", "Total Monthly PITI:", r5_val);
    let r6 = format!("{:<22} {}", "Total Interest:", r6_val);
    let r7 = format!("{:<22} {}", "Total Housing Cost:", r7_val);
    let r8 = format!("{:<22} {}", "50% Equity Month:", r8_val);

    println!("\n{}", BOX_BORDER);
    println!(" 🏠 SCENARIO: {}", mortgage.name.to_uppercase());
    println!("    FULL HOUSING COST ENGINE (PITI + ESCROW SUMMARY)");
    println!("{}", BOX_BORDER);
    println!(" {} | {}", l1, r1);
    println!(" {} | {}", l2, r2);
    println!(" {} | {}", l3, r3);
    println!(" {} | {}", l4, r4);
    println!("{}", BOX_DIVIDER);
    println!(" {} | {}", l5, r5);
    println!(" {} | {}", l6, r6);
    println!(" {} | {}", l7, r7);
    println!("{}", BOX_DIVIDER);
    println!(" {} | {}", l8, r8);
    println!("{}", BOX_DIVIDER);
    let year1_interest = mortgage
        .annual_summaries()
        .first()
        .map(|s| s.interest_paid)
        .unwrap_or(0.0);
    let (year1_annual_savings, year1_monthly_savings) =
        crate::domain::formula::calculate_annual_tax_savings(mortgage.loan, year1_interest);

    println!(
        " CLOSING DAY PREPAIDS: Upfront Ins: ${:.2} | Escrow Buffer: ${:.2} | Total Prepaids: ${:.2}",
        upfront_ins, escrow_buffer, total_prepaids
    );
    println!("{}", BOX_DIVIDER);
    println!(
        " TAX DEDUCTION ESTIMATE: Year 1 Tax Savings: ${:.2}/yr (${:.2}/mo) [Assumes CA MFJ 24% Fed / 9.3% CA]",
        year1_annual_savings, year1_monthly_savings
    );
    println!("{}\n\n", BOX_BORDER);

    // Render Table Header (Unified 14-width columns)
    println!(
        "{:<8} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14}",
        "Month",
        "P&I Total",
        "Principal",
        "Interest",
        "Extra",
        "Escrow",
        "Total Outflow",
        "Balance"
    );
    println!("{}", BOX_DIVIDER);

    for (&month, entry) in &mortgage.schedule {
        let is_early_month = month <= 6;
        let is_late_month = month > actual_months.saturating_sub(3);
        let has_extra = entry.extra > 0.0;

        if is_early_month || is_late_month || has_extra {
            println!(
                "{:<8} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2}",
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
                "{:<8} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14}",
                "...", "...", "...", "...", "...", "...", "...", "..."
            );
        }
    }
    println!("{}\n\n", BOX_BORDER);
}

/// Prints a year-by-year annual rollup summary table to the terminal.
pub fn print_annual_summary_table(mortgage: &Mortgage) {
    let summaries = mortgage.annual_summaries();

    print_banner("📅", "ANNUAL AGGREGATION SUMMARY (YEAR-BY-YEAR ROLLUP)");
    println!(
        "{:<8} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14} | {:<14}",
        "Year",
        "Principal",
        "Extra Paid",
        "Interest",
        "Escrow",
        "Total Outflow",
        "Tax Savings",
        "Net Outlay"
    );
    println!("{}", BOX_DIVIDER);

    for s in &summaries {
        println!(
            "{:<8} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2} | ${:<13.2}",
            format!("Yr {}", s.year),
            s.principal_paid,
            s.extra_principal_paid,
            s.interest_paid,
            s.escrow_paid,
            s.total_outflow,
            s.tax_savings,
            s.net_effective_outlay
        );
    }
    println!("{}\n", BOX_BORDER);
}

/// Prints a 4-column side-by-side comparison report between Option A and Option B
pub fn print_comparison_report(report: &ComparisonReport, title_a: &str, title_b: &str) {
    let col_a_title = format!("A: {}", title_a);
    let col_b_title = format!("B: {}", title_b);

    print_banner(
        "⚖️",
        "MORTGAGE SCENARIO COMPARISON REPORT (OPTION A vs. OPTION B)",
    );
    println!(
        "{:<33} | {:<28} | {:<28} | {:<28}",
        "Financial Metric", col_a_title, col_b_title, "Delta (Option B - A)"
    );
    println!("{}", BOX_DIVIDER);

    // Section 1: Upfront Cash Needed
    println!(" --- 1. UPFRONT CASH NEEDED ---");
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | ${:<27.2}",
        "Down Payment",
        report.option_a.down_payment,
        report.option_b.down_payment,
        report.option_b.down_payment - report.option_a.down_payment
    );
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | ${:<27.2}",
        "Closing Prepaids & Reserves",
        report.option_a.upfront_prepaids,
        report.option_b.upfront_prepaids,
        report.option_b.upfront_prepaids - report.option_a.upfront_prepaids
    );
    let delta_cash_fmt = format_delta_currency(report.delta_cash_to_close);
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | {}",
        "Total Cash to Close",
        report.option_a.total_cash_to_close,
        report.option_b.total_cash_to_close,
        delta_cash_fmt
    );
    println!("{}", BOX_DIVIDER);

    // Section 2: Monthly Cash Outflow
    println!(" --- 2. MONTHLY CASH OUTFLOW ---");
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | ${:<27.2}",
        "Monthly Principal & Interest",
        report.option_a.monthly_p_i,
        report.option_b.monthly_p_i,
        report.option_b.monthly_p_i - report.option_a.monthly_p_i
    );
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | ${:<27.2}",
        "Monthly Escrow (Tax + Ins)",
        report.option_a.monthly_escrow,
        report.option_b.monthly_escrow,
        report.option_b.monthly_escrow - report.option_a.monthly_escrow
    );
    let delta_piti_fmt = format_delta_currency(report.delta_monthly_piti);
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | {}",
        "Total Monthly PITI Outflow",
        report.option_a.monthly_piti,
        report.option_b.monthly_piti,
        delta_piti_fmt
    );
    println!("{}", BOX_DIVIDER);

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
        "{:<33} | {:<28} | {:<28} | {}",
        "Payoff Duration", a_months_fmt, b_months_fmt, delta_months_fmt
    );

    let delta_eq_fmt = format_delta_currency(-report.delta_5yr_equity);
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | {}",
        "5-Year Equity Built",
        report.option_a.equity_at_5_years,
        report.option_b.equity_at_5_years,
        delta_eq_fmt
    );
    println!("{}", BOX_DIVIDER);

    // Section 4: Lifetime Financial Cost
    println!(" --- 4. LIFETIME FINANCIAL COST ---");
    let delta_int_fmt = format_delta_currency(report.delta_total_interest);
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | {}",
        "Total Interest Paid",
        report.option_a.total_interest_paid,
        report.option_b.total_interest_paid,
        delta_int_fmt
    );
    let delta_outflow_fmt = format_delta_currency(report.delta_lifetime_outflow);
    println!(
        "{:<33} | ${:<27.2} | ${:<27.2} | {}",
        "Total Housing Outflow",
        report.option_a.total_lifetime_outflow,
        report.option_b.total_lifetime_outflow,
        delta_outflow_fmt
    );
    println!("{}\n", BOX_BORDER);
}

fn format_delta_currency(delta: f64) -> String {
    if delta > 0.0 {
        format!("+${:<27.2}", delta)
    } else if delta < 0.0 {
        format!("-${:<27.2}", delta.abs())
    } else {
        "$0.00".to_string()
    }
}
