//! scenario_comparison_differential_test.rs
//! Pairwise scenario comparison differential tests, PV monotonicity, and IRR solver accuracy.

use cli::storage::serialize::{get_scenarios_dir_path, load_purchase_from_path};
use engine::service::comparison::{compare_scenarios, solve_irr_newton_raphson};
use engine::service::simulation::create_scenario;
use std::fs;

#[test]
fn test_pairwise_scenario_comparisons_differential_metrics() {
    let dir = get_scenarios_dir_path();
    let mut scenarios = Vec::new();

    for entry in fs::read_dir(&dir).expect("Failed to read scenarios dir") {
        let entry = entry.expect("Valid entry");
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            let purchase = load_purchase_from_path(&path).unwrap();
            let scenario = create_scenario(purchase);
            scenarios.push(scenario);
        }
    }

    assert!(
        scenarios.len() >= 8,
        "Need at least 8 scenarios for pairwise tests"
    );

    for (i, base) in scenarios.iter().enumerate() {
        for (j, alt) in scenarios.iter().enumerate() {
            if i == j {
                // Identity comparison
                let cmp = compare_scenarios(base, alt);
                assert_eq!(cmp.months_saved, 0);
                assert_eq!(cmp.delta_extra_payment, 0.0);
                assert_eq!(cmp.delta_interest_paid, 0.0);
                assert_eq!(cmp.delta_cash_interest, 0.0);
                assert_eq!(cmp.delta_tax_savings, 0.0);
                assert_eq!(cmp.delta_gross_paid, 0.0);
                assert_eq!(cmp.delta_pv, 0.0);
                assert!(cmp.irr.is_none());
                continue;
            }

            let cmp = compare_scenarios(base, alt);

            // Timeline delta
            let expected_months_saved =
                cmp.baseline_payoff_month as i32 - cmp.alternative_payoff_month as i32;
            assert_eq!(cmp.months_saved, expected_months_saved);

            // Extra payment delta
            let expected_delta_extra = cmp.alternative_extra_payment - cmp.baseline_extra_payment;
            assert!((cmp.delta_extra_payment - expected_delta_extra).abs() < 1e-4);

            // Interest paid delta
            let expected_delta_interest =
                cmp.alternative_interest_paid - cmp.baseline_interest_paid;
            assert!((cmp.delta_interest_paid - expected_delta_interest).abs() < 1e-4);

            // Cash interest delta
            let expected_delta_cash = cmp.alternative_cash_interest - cmp.baseline_cash_interest;
            assert!((cmp.delta_cash_interest - expected_delta_cash).abs() < 1e-4);

            // Tax savings delta
            let expected_delta_tax = cmp.alternative_tax_savings - cmp.baseline_tax_savings;
            assert!((cmp.delta_tax_savings - expected_delta_tax).abs() < 1e-4);

            // Gross paid delta
            let expected_delta_gross = cmp.alternative_gross_paid - cmp.baseline_gross_paid;
            assert!((cmp.delta_gross_paid - expected_delta_gross).abs() < 1e-4);

            // Present value delta
            let expected_delta_pv = cmp.alternative_pv - cmp.baseline_pv;
            assert!((cmp.delta_pv - expected_delta_pv).abs() < 1e-4);

            // IRR verification: if IRR exists, NPV at that monthly rate should be near zero
            if let Some(annual_irr) = cmp.irr {
                assert!(annual_irr.is_finite());
                let monthly_rate = (1.0 + annual_irr).powf(1.0 / 12.0) - 1.0;
                let max_len = base
                    .monthly_statement
                    .len()
                    .max(alt.monthly_statement.len());

                let mut npv = 0.0;
                let base_factor = 1.0 + monthly_rate;
                for m_idx in 0..max_len {
                    let flow_base =
                        engine::service::comparison::extract_monthly_outflow(base, m_idx);
                    let flow_alt = engine::service::comparison::extract_monthly_outflow(alt, m_idx);
                    let delta_flow = flow_alt - flow_base;
                    if m_idx == 0 {
                        npv += delta_flow;
                    } else {
                        npv += delta_flow / base_factor.powi(m_idx as i32);
                    }
                }

                assert!(
                    npv.abs() < 1.0 || (npv / cmp.baseline_pv.max(1.0)).abs() < 1e-3,
                    "IRR NPV residual too high: {} for {} vs {}",
                    npv,
                    base.purchase.name,
                    alt.purchase.name
                );
            }
        }
    }
}

#[test]
fn test_irr_solver_direct_known_streams() {
    // 1. Simple 1-month investment: Pay 100 now, receive 105 in 1 month (5% monthly -> ~79.58% annual)
    let flows = vec![-100.0, 105.0];
    let irr =
        solve_irr_newton_raphson(&flows).expect("Should converge for standard 1-month stream");
    let expected_annual = (1.05_f64).powi(12) - 1.0;
    assert!((irr - expected_annual).abs() < 1e-5);

    // 2. 12-month loan stream: +10,000 upfront, -1,000/month for 12 months
    let mut loan_flows = vec![10000.0];
    for _ in 1..=12 {
        loan_flows.push(-1000.0);
    }
    let irr_loan = solve_irr_newton_raphson(&loan_flows).expect("Should converge for loan stream");
    assert!(irr_loan > 0.0);

    // 3. All positive stream (no solution possible)
    let positive_flows = vec![100.0, 200.0, 300.0];
    assert!(solve_irr_newton_raphson(&positive_flows).is_none());

    // 4. All negative stream (no solution possible)
    let negative_flows = vec![-100.0, -200.0, -300.0];
    assert!(solve_irr_newton_raphson(&negative_flows).is_none());
}
