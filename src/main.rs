use chrono::NaiveDate;
use loc::domain::loc::LocEngine;
use loc::ui::display::{
    print_annual_summary_table, print_banner, print_loc_summary, print_monthly_statement_table,
};

fn main() {
    let start_date = NaiveDate::from_ymd_opt(2026, 1, 27).expect("Valid start date");

    // =========================================================================
    // SCENARIO 1: Baseline Interest-Only SBLOC (No Extra Principal Payments)
    // =========================================================================
    println!("\n");
    print_banner(
        "📌",
        "SCENARIO 1: BASELINE INTEREST-ONLY SBLOC (NO EXTRA PAYMENTS)",
    );
    println!();

    let baseline_engine = LocEngine::new(
        "Baseline Interest-Only SBLOC",
        start_date,
        1_500_000.0,
        6.0,
        1.2,
        3600.0,
    )
    .expect("Valid SBLOC configuration");

    print_loc_summary(&baseline_engine);
    println!();
    print_monthly_statement_table(&baseline_engine);
    println!();
    print_annual_summary_table(&baseline_engine);

    // =========================================================================
    // SCENARIO 2: Accelerated Payoff ($100,000 Extra Principal Every 3 Months)
    // =========================================================================
    println!("\n\n");
    print_banner(
        "🚀",
        "SCENARIO 2: ACCELERATED PAYOFF ($100k EXTRA PRINCIPAL EVERY 3 MONTHS)",
    );
    println!();

    let mut accelerated_engine = LocEngine::new(
        "Accelerated Payoff SBLOC ($100k Qtrly Extra Principal)",
        start_date,
        1_500_000.0,
        6.0,
        1.2,
        3600.0,
    )
    .expect("Valid SBLOC configuration");

    // Apply $100,000 extra principal every 3 months (Months 3, 6, 9, 12, ...)
    for month_index in (3..=360).step_by(3) {
        accelerated_engine.add_extra_payment(month_index, 100_000.0);
    }

    print_loc_summary(&accelerated_engine);
    println!();
    print_monthly_statement_table(&accelerated_engine);
    println!();
    print_annual_summary_table(&accelerated_engine);
}
