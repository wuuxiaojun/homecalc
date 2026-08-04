use chrono::NaiveDate;
use loc::domain::loc::LocEngine;
use loc::ui::display::{print_annual_summary_table, print_loc_summary, print_monthly_statement_table};

fn main() {
    let start_date = NaiveDate::from_ymd_opt(2026, 1, 27).expect("Valid start date");
    let engine = LocEngine::new(
        "Primary Residence SBLOC",
        start_date,
        1_500_000.0,
        6.0,
        1.2,
        3600.0,
    )
    .expect("Valid SBLOC configuration");

    print_loc_summary(&engine);
    println!();
    print_monthly_statement_table(&engine);
    println!();
    print_annual_summary_table(&engine);
}
