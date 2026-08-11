use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::tool::{Mortgage, Tool};
use engine::service::simulation::simulate_monthly;
use std::collections::BTreeMap;

#[test]
fn test_zero_down_payment_extreme_loan() {
    let purchase = Purchase {
        name: "100% Financing Zero Down".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 2_400.0,
            monthly_hoa: 150.0,
        },
        tools: vec![Tool::Mortgage(Mortgage {
            amount: 1_000_000.0,
            rate: 7.0,
            term: 30,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let monthly = simulate_monthly(&purchase);

    assert_eq!(monthly.len(), 360);
    assert_eq!(monthly.last().unwrap().total_remaining_balance, 0.0);
    assert!(monthly[0].cash.is_none());
}

#[test]
fn test_massive_extra_repayment_early_payoff() {
    let mut mortgage_repay = BTreeMap::new();
    mortgage_repay.insert(1, 2_000_000.0); // Lump sum far exceeding balance in Month 1

    let purchase = Purchase {
        name: "Immediate Lump Sum Payoff".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 2_400.0,
            monthly_hoa: 150.0,
        },
        tools: vec![Tool::Mortgage(Mortgage {
            amount: 1_000_000.0,
            rate: 6.0,
            term: 30,
        })],
        mortgage_repay,
        loc_repay: BTreeMap::new(),
    };

    let monthly = simulate_monthly(&purchase);

    assert_eq!(monthly.len(), 1);
    assert_eq!(monthly[0].month, 1);
    assert_eq!(monthly[0].total_remaining_balance, 0.0);
}

#[test]
fn test_high_inflation_holding_costs() {
    let purchase = Purchase {
        name: "Inflation Verification".to_string(),
        house: House {
            purchase_price: 1_000_000.0,
            annual_property_tax_rate: 1.2, // $1,000/mo initially
            annual_insurance: 12_000.0,    // $1,000/mo initially
            monthly_hoa: 500.0,            // $500/mo initially
        },
        tools: vec![Tool::Mortgage(Mortgage {
            amount: 1_000_000.0,
            rate: 5.0,
            term: 30,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let monthly = simulate_monthly(&purchase);

    // Month 1 (index 0) initial rates
    let m1_tax = monthly[0].house.monthly_property_tax;
    let m1_ins = monthly[0].house.monthly_insurance;
    let m1_hoa = monthly[0].house.monthly_hoa;

    // Month 13 (index 12) escalated rates (Month 13 is start of Year 2)
    let m13_tax = monthly[12].house.monthly_property_tax;
    let m13_ins = monthly[12].house.monthly_insurance;
    let m13_hoa = monthly[12].house.monthly_hoa;

    // Property Tax grows at 2% (1.02)
    assert!((m13_tax - (m1_tax * 1.02)).abs() < 1e-4);

    // Insurance grows at 5% (1.05)
    assert!((m13_ins - (m1_ins * 1.05)).abs() < 1e-4);

    // HOA grows at 4% (1.04)
    assert!((m13_hoa - (m1_hoa * 1.04)).abs() < 1e-4);
}
