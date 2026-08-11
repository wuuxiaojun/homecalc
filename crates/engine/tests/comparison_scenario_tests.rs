use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::scenario::Scenario;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use engine::service::comparison::compare_scenarios;
use engine::service::simulation::{aggregate_yearly, compute_metrics, simulate_monthly};
use std::collections::BTreeMap;

fn build_scenario(purchase: Purchase) -> Scenario {
    let monthly_statement = simulate_monthly(&purchase);
    let yearly_statement = aggregate_yearly(&monthly_statement);
    let total_statement = compute_metrics(&yearly_statement);

    Scenario {
        purchase,
        monthly_statement,
        yearly_statement,
        total_statement,
    }
}

#[test]
fn test_scenario_comparison_end_to_end() {
    // Scenario A: Baseline 30-Yr Mortgage
    let purchase_a = Purchase {
        name: "Scenario A: 30-Yr Baseline".to_string(),
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
    };

    // Scenario B: 15-Yr + LOC + Extra Repayments
    let purchase_b = Purchase {
        name: "Scenario B: 15-Yr + LOC + Extra".to_string(),
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
                amount: 1_000_000.0,
                rate: 5.9,
                term: 15,
            }),
            Tool::Loc(Loc {
                amount: 200_000.0,
                rate: 7.5,
            }),
        ],
        mortgage_repay: BTreeMap::from([
            (3, 100_000.0),
            (6, 100_000.0),
            (9, 100_000.0),
            (12, 100_000.0),
            (15, 100_000.0),
        ]),
        loc_repay: BTreeMap::from([
            (6, 50_000.0),
            (12, 50_000.0),
            (18, 50_000.0),
            (24, 50_000.0),
        ]),
    };

    let scenario_a = build_scenario(purchase_a);
    let scenario_b = build_scenario(purchase_b);

    let comparison = compare_scenarios(&scenario_a, &scenario_b);

    assert!(comparison.months_saved > 0);
    assert!(comparison.delta_interest_paid < 0.0);
    assert!(comparison.irr > 0.0);
    assert!(comparison.delta_pv < 0.0);
}
