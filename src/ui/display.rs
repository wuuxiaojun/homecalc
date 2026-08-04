use crate::domain::loc::LocEngine;
use crate::ui::terminal::{BOX_BORDER, BOX_DIVIDER};

/// Formats a floating-point number into a clean currency string (e.g. $1,500,000.00).
fn format_currency(val: f64) -> String {
    let is_negative = val < 0.0;
    let abs_val = val.abs();
    let dollars = abs_val.floor() as u64;
    let cents = ((abs_val - dollars as f64) * 100.0).round() as u64;

    let dollars_str = dollars.to_string();
    let mut result = String::new();
    let len = dollars_str.len();
    for (i, ch) in dollars_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }

    if is_negative {
        format!("-${}.{:02}", result, cents)
    } else {
        format!("${}.{:02}", result, cents)
    }
}

/// Renders a boxed title banner with an emoji prefix.
pub fn print_banner(emoji: &str, title: &str) {
    println!("{}", BOX_BORDER);
    println!("{} {}", emoji, title.to_uppercase());
    println!("{}", BOX_BORDER);
}

/// Displays a 2-column top summary card for the SBLOC line of credit setup.
pub fn print_loc_summary(engine: &LocEngine) {
    print_banner("🏡", &format!("SBLOC HOUSING CALCULATOR SUMMARY - {}", engine.name));

    let initial_draw_str = format_currency(engine.initial_draw);
    let current_balance = engine
        .schedule
        .last()
        .map(|s| s.end_balance)
        .unwrap_or(engine.initial_draw);
    let current_balance_str = format_currency(current_balance);
    let start_date_str = engine.start_date.format("%Y-%m-%d").to_string();

    let month1_interest = engine
        .schedule
        .first()
        .map(|s| s.interest_billed)
        .unwrap_or(0.0);
    let monthly_interest_str = format_currency(month1_interest);

    let annual_rate_str = format!("{:.2}%", engine.annual_rate);
    let property_tax_rate_str = format!("{:.2}%", engine.property_tax_rate);
    let annual_insurance_str = format_currency(engine.annual_insurance);

    let monthly_tax_and_ins = (engine.initial_draw * (engine.property_tax_rate / 100.0) / 12.0)
        + (engine.annual_insurance / 12.0);
    let monthly_tax_and_ins_str = format_currency(monthly_tax_and_ins);

    let total_projected_interest: f64 = engine.schedule.iter().map(|s| s.interest_billed).sum();
    let total_interest_str = format_currency(total_projected_interest);

    println!(
        "  {:24} {:>16}  │  {:26} {:>16}",
        "Initial Draw:", initial_draw_str, "Interest Rate:", annual_rate_str
    );
    println!(
        "  {:24} {:>16}  │  {:26} {:>16}",
        "Current Balance:", current_balance_str, "Property Tax Rate:", property_tax_rate_str
    );
    println!(
        "  {:24} {:>16}  │  {:26} {:>16}",
        "Start Date:", start_date_str, "Annual Insurance:", annual_insurance_str
    );
    println!(
        "  {:24} {:>16}  │  {:26} {:>16}",
        "Monthly Interest Charge:", monthly_interest_str, "Monthly Tax + Insurance:", monthly_tax_and_ins_str
    );
    println!(
        "  {:24} {:>16}  │  {:26} {:>16}",
        "", "", "Total Projected Interest:", total_interest_str
    );
    println!("{}", BOX_DIVIDER);
}

/// Displays the month-by-month statement table with middle truncation for long schedules.
pub fn print_monthly_statement_table(engine: &LocEngine) {
    println!("{}", BOX_BORDER);
    println!("📅 MONTHLY STATEMENT SCHEDULE");
    println!("{}", BOX_BORDER);
    println!(
        "| {:^14} | {:^16} | {:^16} | {:^14} | {:^17} | {:^16} | {:^16} |",
        "Month",
        "Start Balance",
        "Interest Billed",
        "Tax & Ins",
        "Extra Principal",
        "Total Outflow",
        "End Balance"
    );
    println!("{}", BOX_DIVIDER);

    let total_months = engine.schedule.len();
    if total_months > 24 {
        // Print first 12 months
        for stmt in &engine.schedule[0..12] {
            print_monthly_statement_row(stmt);
        }

        // Print truncation row
        println!(
            "| {:^14} | {:^16} | {:^16} | {:^14} | {:^17} | {:^16} | {:^16} |",
            "...", "...", "...", "...", "...", "...", "..."
        );

        // Print last 12 months
        for stmt in &engine.schedule[total_months - 12..total_months] {
            print_monthly_statement_row(stmt);
        }
    } else {
        for stmt in &engine.schedule {
            print_monthly_statement_row(stmt);
        }
    }

    println!("{}", BOX_DIVIDER);
}

fn print_monthly_statement_row(stmt: &crate::domain::loc::LocMonthlyStatement) {
    let month_str = format!("{} (#{})", stmt.date_label, stmt.month_index);
    println!(
        "| {:^14} | {:>16} | {:>16} | {:>14} | {:>17} | {:>16} | {:>16} |",
        month_str,
        format_currency(stmt.start_balance),
        format_currency(stmt.interest_billed),
        format_currency(stmt.tax_and_insurance),
        format_currency(stmt.extra_principal_paid),
        format_currency(stmt.total_outflow),
        format_currency(stmt.end_balance)
    );
}

/// Displays the year-by-year rollup table aggregated by calendar year.
pub fn print_annual_summary_table(engine: &LocEngine) {
    let summaries = engine.annual_summaries();

    println!("{}", BOX_BORDER);
    println!("📊 ANNUAL ROLLUP SUMMARY");
    println!("{}", BOX_BORDER);
    println!(
        "| {:^14} | {:^16} | {:^16} | {:^14} | {:^17} | {:^16} | {:^16} |",
        "Year",
        "Start Balance",
        "Interest Billed",
        "Tax & Ins",
        "Extra Principal",
        "Total Outflow",
        "End Balance"
    );
    println!("{}", BOX_DIVIDER);

    for summary in &summaries {
        println!(
            "| {:^14} | {:>16} | {:>16} | {:>14} | {:>17} | {:>16} | {:>16} |",
            summary.year_label,
            format_currency(summary.start_balance),
            format_currency(summary.total_interest_paid),
            format_currency(summary.total_tax_and_insurance_paid),
            format_currency(summary.total_extra_principal_paid),
            format_currency(summary.total_outflow),
            format_currency(summary.end_balance)
        );
    }

    let total_interest: f64 = summaries.iter().map(|s| s.total_interest_paid).sum();
    let total_tax_ins: f64 = summaries.iter().map(|s| s.total_tax_and_insurance_paid).sum();
    let total_extra: f64 = summaries.iter().map(|s| s.total_extra_principal_paid).sum();
    let total_outflow: f64 = summaries.iter().map(|s| s.total_outflow).sum();

    println!("{}", BOX_DIVIDER);
    println!(
        "| {:^14} | {:^16} | {:>16} | {:>14} | {:>17} | {:>16} | {:^16} |",
        "TOTAL",
        "",
        format_currency(total_interest),
        format_currency(total_tax_ins),
        format_currency(total_extra),
        format_currency(total_outflow),
        ""
    );
    println!("{}", BOX_BORDER);
}
