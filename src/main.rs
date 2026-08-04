use chrono::NaiveDate;
use loc::domain::loc::LocEngine;

fn main() {
    let start_date = NaiveDate::from_ymd_opt(2026, 8, 1).expect("Valid date");
    match LocEngine::new(
        "Primary SBLOC Home Loan",
        start_date,
        1_500_000.0,
        6.0,
        1.2,
        3600.0,
    ) {
        Ok(engine) => {
            println!("SBLOC Housing Engine initialized for: {}", engine.name);
            println!("Total schedule length: {} months", engine.schedule.len());
            if let Some(first) = engine.schedule.first() {
                println!(
                    "Month 1 ({}) Outflow: ${:.2} (Interest Billed: ${:.2}, Tax: ${:.2}, Insurance: ${:.2})",
                    first.date_label,
                    first.total_outflow,
                    first.interest_billed,
                    first.monthly_property_tax,
                    first.monthly_insurance
                );
            }
        }
        Err(err) => eprintln!("Failed to initialize SBLOC engine: {}", err),
    }
}
