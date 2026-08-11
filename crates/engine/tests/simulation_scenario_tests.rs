use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::tool::{Cash, Mortgage, Tool};
use engine::service::simulation::{aggregate_yearly, compute_metrics, simulate_monthly};
use std::collections::BTreeMap;

fn create_standard_purchase() -> Purchase {
    Purchase {
        name: "Standard 30-Year Baseline".to_string(),
        house: House {
            purchase_price: 1_500_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 3_600.0,
            monthly_hoa: 100.0,
        },
        tools: vec![
            Tool::Cash(Cash {
                amount: 300_000.0,
                rate: 3.9,
            }),
            Tool::Mortgage(Mortgage {
                amount: 1_200_000.0,
                rate: 6.0,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    }
}

#[test]
fn test_30yr_baseline_amortization_schedule() {
    let purchase = create_standard_purchase();
    let monthly_statement = simulate_monthly(&purchase);
    let yearly_statement = aggregate_yearly(&monthly_statement);
    let total_statement = compute_metrics(&yearly_statement);

    assert_eq!(monthly_statement.len(), 360);
    assert_eq!(monthly_statement.last().unwrap().total_remaining_balance, 0.0);
    assert_eq!(yearly_statement.len(), 30);
    assert!(total_statement.total_interest_paid > 0.0);
}

#[test]
fn test_accelerated_payoff_schedule() {
    let mut purchase = create_standard_purchase();
    purchase.name = "Accelerated Payoff".to_string();

    // Schedule aggressive extra payments
    for yr in 1..=5 {
        purchase.mortgage_repay.insert(yr * 12, 100_000.0);
    }

    let baseline_monthly = simulate_monthly(&create_standard_purchase());
    let baseline_yearly = aggregate_yearly(&baseline_monthly);
    let baseline_total = compute_metrics(&baseline_yearly);

    let acc_monthly = simulate_monthly(&purchase);
    let acc_yearly = aggregate_yearly(&acc_monthly);
    let acc_total = compute_metrics(&acc_yearly);

    let payoff_month = acc_monthly.last().unwrap().month;
    assert!(payoff_month < 360);
    assert!(acc_total.total_interest_paid < baseline_total.total_interest_paid);
}

#[test]
fn test_irs_mortgage_tax_deduction_cap() {
    let purchase = create_standard_purchase();
    let monthly_statement = simulate_monthly(&purchase);
    let yearly_statement = aggregate_yearly(&monthly_statement);

    // Year 1 average mortgage balance is > $750,000 (starting at $1.2M)
    // So annual_tax_savings must be strictly less than (annual_interest_paid * 24%)
    let year1 = &yearly_statement[0];
    let uncapped_tax_savings = year1.annual_interest_paid * 0.24;
    assert!(year1.annual_tax_savings < uncapped_tax_savings);
    assert!(year1.annual_tax_savings > 0.0);
}
