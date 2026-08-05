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
        if i > 0 && (len - i).is_multiple_of(3) {
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

fn format_delta_currency(val: f64) -> String {
    if val > 0.0 {
        format!("+{}", format_currency(val))
    } else if val < 0.0 {
        format_currency(val)
    } else {
        "$0.00".to_string()
    }
}

fn format_delta_pct(val: f64) -> String {
    if val > 0.0 {
        format!("+{:.2}%", val)
    } else {
        format!("{:.2}%", val)
    }
}

fn format_delta_count(val: i64) -> String {
    if val > 0 {
        format!("+{}", val)
    } else {
        val.to_string()
    }
}

/// Renders a boxed title banner with an emoji prefix.
pub fn print_banner(emoji: &str, title: &str) {
    println!("{}", BOX_BORDER);
    println!("|  {} {:<116} |", emoji, title.to_uppercase());
    println!("{}", BOX_BORDER);
}

/// Displays a 2-column top summary card for the LOC line of credit setup.
pub fn print_loc_summary(engine: &LocEngine) {
    print_banner("🏡", &format!("LOC HOUSING CALCULATOR SUMMARY - {}", engine.name));

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

    let monthly_tax_and_ins = engine.monthly_tax_and_insurance();
    let monthly_tax_and_ins_str = format_currency(monthly_tax_and_ins);

    let total_projected_interest: f64 = engine.schedule.iter().map(|s| s.interest_billed).sum();
    let total_interest_str = format_currency(total_projected_interest);

    println!(
        "| {:26} {:>31} | {:26} {:>32} |",
        "Initial Draw:", initial_draw_str, "Interest Rate:", annual_rate_str
    );
    println!(
        "| {:26} {:>31} | {:26} {:>32} |",
        "Current Balance:", current_balance_str, "Property Tax Rate:", property_tax_rate_str
    );
    println!(
        "| {:26} {:>31} | {:26} {:>32} |",
        "Start Date:", start_date_str, "Annual Insurance:", annual_insurance_str
    );
    println!(
        "| {:26} {:>31} | {:26} {:>32} |",
        "Monthly Interest Charge:", monthly_interest_str, "Monthly Tax + Insurance:", monthly_tax_and_ins_str
    );
    println!(
        "| {:26} {:>31} | {:26} {:>32} |",
        "", "", "Total Projected Interest:", total_interest_str
    );
    println!("{}", BOX_BORDER);
}

/// Displays the month-by-month statement table with middle truncation for long schedules.
pub fn print_monthly_statement_table(engine: &LocEngine) {
    println!("{}", BOX_BORDER);
    println!("|  📅 {:<115} |", "MONTHLY STATEMENT SCHEDULE");
    println!("{}", BOX_BORDER);
    println!(
        "| {:^15} | {:^16} | {:^16} | {:^14} | {:^17} | {:^16} | {:^16} |",
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
            "| {:^15} | {:^16} | {:^16} | {:^14} | {:^17} | {:^16} | {:^16} |",
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

    println!("{}", BOX_BORDER);
}

fn print_monthly_statement_row(stmt: &crate::domain::loc::LocMonthlyStatement) {
    let month_str = format!("{} (#{})", stmt.date_label, stmt.month_index);
    println!(
        "| {:^15} | {:>16} | {:>16} | {:>14} | {:>17} | {:>16} | {:>16} |",
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
    println!("|  📊 {:<115} |", "ANNUAL ROLLUP SUMMARY");
    println!("{}", BOX_BORDER);
    println!(
        "| {:^15} | {:^16} | {:^16} | {:^14} | {:^17} | {:^16} | {:^16} |",
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
            "| {:^15} | {:>16} | {:>16} | {:>14} | {:>17} | {:>16} | {:>16} |",
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
        "| {:^15} | {:^16} | {:>16} | {:>14} | {:>17} | {:>16} | {:^16} |",
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

/// Renders a 4-column side-by-side comparison table between two LOC scenarios.
/// Strictly divided into 3 financial sections.
pub fn print_loc_comparison_report(
    report: &crate::analysis::comparison::LocComparisonReport,
    title_a: &str,
    title_b: &str,
) {
    print_banner("⚖️", "LOC SCENARIO COMPARISON REPORT");

    let a = &report.option_a;
    let b = &report.option_b;

    let title_a_fmt = if title_a.len() > 25 {
        format!("{}...", &title_a[..22])
    } else {
        title_a.to_string()
    };

    let title_b_fmt = if title_b.len() > 25 {
        format!("{}...", &title_b[..22])
    } else {
        title_b.to_string()
    };

    println!(
        "| {:^36} | {:^25} | {:^25} | {:^25} |",
        "Financial Metric", title_a_fmt, title_b_fmt, "Delta (Option B - A)"
    );
    println!("{}", BOX_BORDER);

    // --- SECTION 1: BASELINE TERMS ---
    println!(
        "| {:^120} |",
        "--- SECTION 1: BASELINE TERMS ---"
    );
    println!("{}", BOX_DIVIDER);
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Initial Draw ($)",
        format_currency(a.initial_draw),
        format_currency(b.initial_draw),
        format_delta_currency(b.initial_draw - a.initial_draw)
    );
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Interest Rate (%)",
        format!("{:.2}%", a.annual_rate),
        format!("{:.2}%", b.annual_rate),
        format_delta_pct(b.annual_rate - a.annual_rate)
    );
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Annual Property Tax & Insurance ($)",
        format_currency(a.annual_tax_and_insurance),
        format_currency(b.annual_tax_and_insurance),
        format_delta_currency(b.annual_tax_and_insurance - a.annual_tax_and_insurance)
    );
    println!("{}", BOX_BORDER);

    // --- SECTION 2: LUMP-SUM REPAYMENTS ---
    println!(
        "| {:^120} |",
        "--- SECTION 2: LUMP-SUM REPAYMENTS ---"
    );
    println!("{}", BOX_DIVIDER);
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Total Extra Lump-Sums Paid ($)",
        format_currency(a.total_lump_sum_paid),
        format_currency(b.total_lump_sum_paid),
        format_delta_currency(report.delta_lump_sum_paid)
    );
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Number of Lump-Sum Payment Events",
        a.lump_sum_event_count,
        b.lump_sum_event_count,
        format_delta_count(b.lump_sum_event_count as i64 - a.lump_sum_event_count as i64)
    );
    println!("{}", BOX_BORDER);

    // --- SECTION 3: LIFETIME FINANCIAL COSTS ---
    println!(
        "| {:^120} |",
        "--- SECTION 3: LIFETIME FINANCIAL COSTS ---"
    );
    println!("{}", BOX_DIVIDER);
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Total Billed Interest ($)",
        format_currency(a.total_interest_paid),
        format_currency(b.total_interest_paid),
        format_delta_currency(report.delta_total_interest)
    );
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Total Tax & Insurance Paid ($)",
        format_currency(a.total_tax_and_insurance_paid),
        format_currency(b.total_tax_and_insurance_paid),
        format_delta_currency(b.total_tax_and_insurance_paid - a.total_tax_and_insurance_paid)
    );
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Total Lifetime Cash Outflow ($)",
        format_currency(a.total_lifetime_outflow),
        format_currency(b.total_lifetime_outflow),
        format_delta_currency(report.delta_lifetime_outflow)
    );
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Year 5 Balance ($)",
        format_currency(a.balance_at_year_5),
        format_currency(b.balance_at_year_5),
        format_delta_currency(report.delta_year5_balance)
    );
    println!(
        "| {:36} | {:>25} | {:>25} | {:>25} |",
        "Year 5 Equity Built ($)",
        format_currency(a.equity_at_year_5),
        format_currency(b.equity_at_year_5),
        format_delta_currency(report.delta_year5_equity)
    );
    println!("{}", BOX_BORDER);
}
