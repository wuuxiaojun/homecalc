//! engine_benchmarks.rs
//! Benchmarks for engine simulation, analysis, and comparison.

use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use engine::service::analysis::analyze_scenario;
use engine::service::comparison::compare_scenarios;
use engine::service::simulation::create_scenario;
use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
    println!("Running engine benchmarks...");

    let house = House {
        purchase_price: 1_200_000.0,
        annual_property_tax_rate: 1.25,
        annual_insurance: 2_400.0,
        monthly_hoa: 150.0,
    };

    let purchase_30yr = Purchase {
        name: "Benchmark 30yr Mortgage".to_string(),
        house: house.clone(),
        tools: vec![
            Tool::Cash(Cash {
                amount: 240_000.0,
                rate: 4.0,
            }),
            Tool::Mortgage(Mortgage {
                amount: 960_000.0,
                rate: 6.5,
                term: 30,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay: BTreeMap::new(),
    };

    let mut loc_repay = BTreeMap::new();
    for m in 1..=60 {
        loc_repay.insert(m, 16_000.0);
    }

    let purchase_loc = Purchase {
        name: "Benchmark 5yr LOC".to_string(),
        house,
        tools: vec![
            Tool::Cash(Cash {
                amount: 240_000.0,
                rate: 4.0,
            }),
            Tool::Loc(Loc {
                amount: 960_000.0,
                rate: 7.0,
            }),
        ],
        mortgage_repay: BTreeMap::new(),
        loc_repay,
    };

    let iterations = 1000;
    let start = Instant::now();

    for _ in 0..iterations {
        let s1 = create_scenario(purchase_30yr.clone());
        let s2 = create_scenario(purchase_loc.clone());
        let _a1 = analyze_scenario(&s1);
        let _a2 = analyze_scenario(&s2);
        let _cmp = compare_scenarios(&s1, &s2);
    }

    let elapsed = start.elapsed();
    println!(
        "Completed {} iterations in {:?} ({:.2} µs/iter)",
        iterations,
        elapsed,
        elapsed.as_micros() as f64 / iterations as f64
    );
}
