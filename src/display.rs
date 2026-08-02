// src/display.rs

use crate::analysis::ComparisonReport;
use crate::mortgage::Mortgage;
use std::io::{self, Write};

// ANSI Color & Formatting Constants
pub const RESET: &str = "\x1B[0m";
pub const BOLD: &str = "\x1B[1m";
pub const DIM: &str = "\x1B[2m";
pub const RED: &str = "\x1B[1;31m";
pub const GREEN: &str = "\x1B[1;32m";
pub const YELLOW: &str = "\x1B[1;33m";
pub const MAGENTA: &str = "\x1B[1;35m";
pub const CYAN: &str = "\x1B[1;36m";
pub const WHITE: &str = "\x1B[1;37m";

/// Clears the terminal screen buffer cleanly
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

/// Helper function to print standardized double-line header banners
pub fn print_banner(emoji: &str, title: &str) {
    let box_border = "===============================================================================================================";
    println!("{}{}{}", CYAN, box_border, RESET);
    println!(" {} {}{}{}", emoji, BOLD, title.to_uppercase(), RESET);
    println!("{}{}{}", CYAN, box_border, RESET);
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

    let l1 = format!("Home Price:       {}{}${:.2}{}", BOLD, GREEN, mortgage.price, RESET);
    let r1 = format!(
        "Down Payment:     {}{}${:.2}{} ({:.1}%)",
        BOLD, GREEN, mortgage.down, RESET,
        (mortgage.down / mortgage.price) * 100.0
    );

    let l2 = format!("Loan Principal:   {}{}${:.2}{}", BOLD, GREEN, mortgage.loan, RESET);
    let r2 = format!("Interest Rate:    {}{}{:.2}%{}", BOLD, CYAN, mortgage.rate, RESET);

    let l3 = format!("Original Term:    {}{}{} yrs{}", BOLD, CYAN, mortgage.term, RESET);
    let r3 = format!(
        "Property Tax Rate: {:.2}%/yr (${:.2}/mo)",
        mortgage.tax_rate,
        mortgage.monthly_tax()
    );

    let l4 = format!("Home Insurance:   ${:.2}/yr", mortgage.annual_insurance);
    let r4 = format!("Monthly Escrow:   ${:.2}/mo", mortgage.monthly_escrow());

    let l5 = format!("Monthly P&I:      {}{}${:.2}{}", BOLD, CYAN, mortgage.base_payment, RESET);
    let r5 = format!(
        "Total Monthly PITI: {}{}${:.2}/mo{}",
        BOLD, GREEN,
        mortgage.base_payment + mortgage.monthly_escrow(),
        RESET
    );

    let l6 = format!(
        "Actual Payoff:    {}{}{} mos ({:.1} yrs){}",
        BOLD, YELLOW, actual_months, actual_years, RESET
    );
    let r6 = format!("Total Interest:   {}{}${:.2}{}", BOLD, RED, total_interest_paid, RESET);

    let l7 = format!("Total Escrow Paid:${:.2}", total_escrow_paid);
    let r7 = format!("Total Housing Cost: {}{}${:.2}{}", BOLD, WHITE, total_housing_outflow, RESET);

    let crossover_str = match mortgage.crossover_month() {
        Some(m) => format!("Month {} (Yr {:.1})", m, m as f64 / 12.0),
        None => "N/A".to_string(),
    };
    let half_equity_str = match mortgage.half_equity_month() {
        Some(m) => format!("Month {} (Yr {:.1})", m, m as f64 / 12.0),
        None => "N/A".to_string(),
    };

    let l8 = format!("Crossover Month:  {}{}{}", BOLD, MAGENTA, crossover_str);
    let r8 = format!("50% Equity Month:  {}{}{}", BOLD, MAGENTA, half_equity_str);

    let box_border = "===============================================================================================================";
    let box_divider = "---------------------------------------------------------------------------------------------------------------";

    println!("\n{}{}{}", CYAN, box_border, RESET);
    println!(" 🏠 {}{}SCENARIO: {}{}", BOLD, WHITE, mortgage.name.to_uppercase(), RESET);
    println!("    {}FULL HOUSING COST ENGINE (PITI + ESCROW SUMMARY){}", DIM, RESET);
    println!("{}{}{}", CYAN, box_border, RESET);
    println!(" {:<53} | {}", l1, r1);
    println!(" {:<53} | {}", l2, r2);
    println!(" {:<53} | {}", l3, r3);
    println!(" {:<44} | {}", l4, r4);
    println!("{}{}{}", DIM, box_divider, RESET);
    println!(" {:<53} | {}", l5, r5);
    println!(" {:<53} | {}", l6, r6);
    println!(" {:<44} | {}", l7, r7);
    println!("{}{}{}", DIM, box_divider, RESET);
    println!(" {:<53} | {}", l8, r8);
    println!("{}{}{}", DIM, box_divider, RESET);
    println!(
        " CLOSING DAY PREPAIDS: Upfront Ins: ${:.2} | Escrow Buffer: ${:.2} | Total Prepaids: {}${:.2}{}",
        upfront_ins, escrow_buffer, BOLD, total_prepaids, RESET
    );
    println!("{}{}{}\n", CYAN, box_border, RESET);

    // Render Table Header
    println!(
        "{}{:<6} | {:<12} | {:<12} | {:<12} | {:<10} | {:<12} | {:<13} | {:<13}{}",
        BOLD, "Month", "P&I Total", "Principal", "Interest", "Extra", "Escrow", "Total Outflow", "Balance", RESET
    );
    println!("{}{}{}", DIM, box_divider, RESET);

    for (&month, entry) in &mortgage.schedule {
        let is_early_month = month <= 6;
        let is_late_month = month > actual_months.saturating_sub(3);
        let has_extra = entry.extra > 0.0;

        if is_early_month || is_late_month || has_extra {
            let extra_str = if entry.extra > 0.0 {
                format!("{}{}${:<9.2}{}", BOLD, GREEN, entry.extra, RESET)
            } else {
                format!("${:<9.2}", entry.extra)
            };

            println!(
                "{:<6} | ${:<11.2} | ${:<11.2} | ${:<11.2} | {} | ${:<11.2} | ${:<12.2} | ${:<12.2}",
                entry.month,
                entry.total_p_i,
                entry.principal,
                entry.interest,
                extra_str,
                entry.escrow,
                entry.total_outflow,
                entry.balance
            );
        } else if month == 7 {
            println!(
                "{}{}  ...  |      ...     |      ...     |      ...     |    ...    |      ...     |      ...      |      ...     {}",
                DIM, RESET, DIM
            );
        }
    }
    println!("{}{}{}\n", CYAN, box_border, RESET);
}

/// Prints a year-by-year annual rollup summary table to the terminal.
pub fn print_annual_summary_table(mortgage: &Mortgage) {
    let summaries = mortgage.annual_summaries();
    let box_border = "===============================================================================================================";
    let box_divider = "---------------------------------------------------------------------------------------------------------------";

    print_banner("📅", "ANNUAL AGGREGATION SUMMARY (YEAR-BY-YEAR ROLLUP)");
    println!(
        "{}{:<6} | {:<12} | {:<12} | {:<12} | {:<12} | {:<14} | {:<13}{}",
        BOLD, "Year", "Principal", "Extra Paid", "Interest", "Escrow", "Total Outflow", "End Balance", RESET
    );
    println!("{}{}{}", DIM, box_divider, RESET);

    for s in &summaries {
        let extra_str = if s.extra_principal_paid > 0.0 {
            format!("{}{}${:<11.2}{}", BOLD, GREEN, s.extra_principal_paid, RESET)
        } else {
            format!("${:<11.2}", s.extra_principal_paid)
        };

        println!(
            "{:<6} | ${:<11.2} | {} | ${:<11.2} | ${:<11.2} | ${:<13.2} | ${:<12.2}",
            format!("Yr {}", s.year),
            s.principal_paid,
            extra_str,
            s.interest_paid,
            s.escrow_paid,
            s.total_outflow,
            s.year_end_balance
        );
    }
    println!("{}{}{}\n", CYAN, box_border, RESET);
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

    print_banner("⚖️", "MORTGAGE SCENARIO COMPARISON REPORT (OPTION A vs. OPTION B)");
    println!(
        "{}{:<30} | {:<24} | {:<24} | {:<24}{}",
        BOLD, "Financial Metric", col_a_title, col_b_title, "Delta (Option B - A)", RESET
    );
    println!("{}{}{}", DIM, box_divider, RESET);

    // Section 1: Upfront Cash Needed
    println!(" {}{}--- 1. UPFRONT CASH NEEDED ---{}", BOLD, MAGENTA, RESET);
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
        "{:<30} | ${:<23.2} | ${:<23.2} | {}",
        "Total Cash to Close",
        report.option_a.total_cash_to_close,
        report.option_b.total_cash_to_close,
        delta_cash_fmt
    );
    println!("{}{}{}", DIM, box_divider, RESET);

    // Section 2: Monthly Cash Outflow
    println!(" {}{}--- 2. MONTHLY CASH OUTFLOW ---{}", BOLD, MAGENTA, RESET);
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
        "{:<30} | ${:<23.2} | ${:<23.2} | {}",
        "Total Monthly PITI Outflow",
        report.option_a.monthly_piti,
        report.option_b.monthly_piti,
        delta_piti_fmt
    );
    println!("{}{}{}", DIM, box_divider, RESET);

    // Section 3: Loan Timeline & Equity
    println!(" {}{}--- 3. LOAN TIMELINE & EQUITY ---{}", BOLD, MAGENTA, RESET);
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
            "{}{}+{} mos (+{:.1} yrs){}",
            BOLD, YELLOW,
            report.delta_payoff_months,
            report.delta_payoff_months as f64 / 12.0,
            RESET
        )
    } else if report.delta_payoff_months < 0 {
        format!(
            "{}{}{} mos ({:.1} yrs){}",
            BOLD, GREEN,
            report.delta_payoff_months,
            report.delta_payoff_months as f64 / 12.0,
            RESET
        )
    } else {
        format!("{}0 mos{}", DIM, RESET)
    };
    println!(
        "{:<30} | {:<24} | {:<24} | {}",
        "Payoff Duration", a_months_fmt, b_months_fmt, delta_months_fmt
    );

    let delta_eq_fmt = format_delta_currency(-report.delta_5yr_equity); // Inverted so lower equity is highlighted
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {}",
        "5-Year Equity Built",
        report.option_a.equity_at_5_years,
        report.option_b.equity_at_5_years,
        delta_eq_fmt
    );
    println!("{}{}{}", DIM, box_divider, RESET);

    // Section 4: Lifetime Financial Cost
    println!(" {}{}--- 4. LIFETIME FINANCIAL COST ---{}", BOLD, MAGENTA, RESET);
    let delta_int_fmt = format_delta_currency(report.delta_total_interest);
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {}",
        "Total Interest Paid",
        report.option_a.total_interest_paid,
        report.option_b.total_interest_paid,
        delta_int_fmt
    );
    let delta_outflow_fmt = format_delta_currency(report.delta_lifetime_outflow);
    println!(
        "{:<30} | ${:<23.2} | ${:<23.2} | {}",
        "Total Housing Outflow",
        report.option_a.total_lifetime_outflow,
        report.option_b.total_lifetime_outflow,
        delta_outflow_fmt
    );
    println!("{}{}{}\n", CYAN, box_border, RESET);
}

fn format_delta_currency(delta: f64) -> String {
    if delta > 0.0 {
        format!("{}{}+${:<22.2}{}", BOLD, YELLOW, delta, RESET)
    } else if delta < 0.0 {
        format!("{}{}-${:<22.2}{}", BOLD, GREEN, delta.abs(), RESET)
    } else {
        format!("{}$0.00                   {}", DIM, RESET)
    }
}
