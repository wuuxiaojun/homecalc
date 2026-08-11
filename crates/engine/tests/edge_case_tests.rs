use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::scenario::Scenario;
use engine::domain::tool::{Cash, Mortgage, Tool};
use engine::service::comparison::calculate_strategy_irr;
use engine::service::simulation::{aggregate_yearly, compute_metrics, simulate_monthly};
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

#[test]
fn test_over_repayment_extra_payment_clamped() {
    let mut mortgage_repay = BTreeMap::new();
    mortgage_repay.insert(1, 50_000.0); // $50,000 extra on a $10,000 mortgage

    let purchase = Purchase {
        name: "Small Mortgage Over-Repayment".to_string(),
        house: House {
            purchase_price: 200_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 1_000.0,
            monthly_hoa: 50.0,
        },
        tools: vec![Tool::Mortgage(Mortgage {
            amount: 10_000.0,
            rate: 5.0,
            term: 5,
        })],
        mortgage_repay,
        loc_repay: BTreeMap::new(),
    };

    let monthly = simulate_monthly(&purchase);

    assert_eq!(monthly.len(), 1);
    let m1 = &monthly[0];
    let extra = m1.mortgage.as_ref().unwrap().extra_payment;
    assert!(extra <= 10_000.0);
    assert_eq!(m1.total_remaining_balance, 0.0);
}

#[test]
fn test_zero_debt_setup() {
    let purchase = Purchase {
        name: "Zero Debt Setup".to_string(),
        house: House {
            purchase_price: 500_000.0,
            annual_property_tax_rate: 1.0,
            annual_insurance: 1_200.0,
            monthly_hoa: 100.0,
        },
        tools: vec![Tool::Cash(Cash {
            amount: 500_000.0,
            rate: 4.0,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let monthly = simulate_monthly(&purchase);
    assert!(monthly.is_empty());
}

#[test]
fn test_irr_non_convergence_safety() {
    let purchase = Purchase {
        name: "Baseline Purchase".to_string(),
        house: House {
            purchase_price: 500_000.0,
            annual_property_tax_rate: 1.2,
            annual_insurance: 1_200.0,
            monthly_hoa: 100.0,
        },
        tools: vec![Tool::Mortgage(Mortgage {
            amount: 400_000.0,
            rate: 6.0,
            term: 30,
        })],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let monthly_a = simulate_monthly(&purchase);
    let yearly_a = aggregate_yearly(&monthly_a);
    let total_a = compute_metrics(&yearly_a);
    let scenario_a = Scenario {
        purchase: purchase.clone(),
        monthly_statement: monthly_a,
        yearly_statement: yearly_a,
        total_statement: total_a,
    };

    // Scenario B has strictly higher outflows every month without debt reduction
    // Delta cash flows are all strictly positive (+1000.0), so NPV(r) = 0 has no real root and IRR solver returns None.
    let mut scenario_b = scenario_a.clone();
    for row in &mut scenario_b.monthly_statement {
        row.total_paid += 1000.0;
    }

    let irr_result = calculate_strategy_irr(&scenario_a, &scenario_b);
    assert!(irr_result.is_none());
}
