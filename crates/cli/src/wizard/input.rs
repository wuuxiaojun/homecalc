//! input.rs
//! Interactive Purchase creation form with strict validation and tool balancing.

use anyhow::Result;
use engine::domain::house::House;
use engine::domain::purchase::Purchase;
use engine::domain::tool::{Cash, Loc, Mortgage, Tool};
use inquire::validator::Validation;
use inquire::{Confirm, CustomType, Select, Text};
use std::collections::BTreeMap;

/// Prompts the user interactively to create a fully configured `Purchase` struct.
pub fn prompt_create_purchase() -> Result<Purchase> {
    println!("\n================================================================================");
    println!(" CREATE NEW PURCHASE SCENARIO");
    println!("================================================================================");

    // 1. Basic Info & House Parameters
    println!("\n--- Purchase Information ---");

    let name = Text::new("Scenario Name:")
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(Validation::Invalid("Scenario name cannot be empty.".into()))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()?;

    let purchase_price = CustomType::<f64>::new("Purchase Price ($):")
        .with_validator(|val: &f64| {
            if *val > 0.0 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Purchase price must be > 0.0".into()))
            }
        })
        .prompt()?;

    let annual_property_tax_rate = CustomType::<f64>::new("Annual Property Tax Rate (%):")
        .with_validator(|val: &f64| {
            if (0.0..=20.0).contains(val) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Tax rate must be between 0.0% and 20.0%".into(),
                ))
            }
        })
        .prompt()?;

    let annual_insurance = CustomType::<f64>::new("Annual Insurance ($):")
        .with_validator(|val: &f64| {
            if *val >= 0.0 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Insurance must be >= 0.0".into()))
            }
        })
        .prompt()?;

    let monthly_hoa = CustomType::<f64>::new("Monthly HOA ($):")
        .with_validator(|val: &f64| {
            if *val >= 0.0 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("HOA must be >= 0.0".into()))
            }
        })
        .prompt()?;

    let house = House {
        purchase_price,
        annual_property_tax_rate,
        annual_insurance,
        monthly_hoa,
    };

    // 2. Financial Tool Allocation & Dynamic Cash Balancing
    println!("\n--- Financial Tools Allocation ---");

    let mut mortgage_amount: f64;
    let mut mortgage_rate: f64 = 0.0;
    let mut mortgage_term: u32 = 30;

    let mut loc_amount: f64;
    let mut loc_rate: f64 = 0.0;

    loop {
        mortgage_amount = CustomType::<f64>::new("Mortgage Loan Principal Amount ($):")
            .with_validator(|val: &f64| {
                if *val >= 0.0 {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("Mortgage amount must be >= 0.0".into()))
                }
            })
            .prompt()?;

        if mortgage_amount > 0.0 {
            mortgage_rate = CustomType::<f64>::new("Mortgage Interest Rate (%):")
                .with_validator(|val: &f64| {
                    if (0.0..=20.0).contains(val) {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid(
                            "Mortgage rate must be between 0.0% and 20.0%".into(),
                        ))
                    }
                })
                .prompt()?;

            mortgage_term = CustomType::<u32>::new("Mortgage Term (Years):")
                .with_validator(|val: &u32| {
                    if (1..=30).contains(val) {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid(
                            "Term must be between 1 and 30 years".into(),
                        ))
                    }
                })
                .prompt()?;
        }

        loc_amount = CustomType::<f64>::new("Line of Credit (LOC) Amount ($):")
            .with_validator(|val: &f64| {
                if *val >= 0.0 {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("LOC amount must be >= 0.0".into()))
                }
            })
            .prompt()?;

        if loc_amount > 0.0 {
            loc_rate = CustomType::<f64>::new("LOC Interest Rate (%):")
                .with_validator(|val: &f64| {
                    if (0.0..=20.0).contains(val) {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid(
                            "LOC rate must be between 0.0% and 20.0%".into(),
                        ))
                    }
                })
                .prompt()?;
        }

        let total_borrowed = mortgage_amount + loc_amount;
        if total_borrowed <= purchase_price {
            break;
        }

        println!(
            "\n[!] Error: Total borrowed amount (${:.2}) exceeds Purchase Price (${:.2}). Please re-enter tool amounts.\n",
            total_borrowed, purchase_price
        );
    }

    let cash_amount = purchase_price - (mortgage_amount + loc_amount);
    let mut tools = Vec::new();

    if cash_amount > 0.0 {
        println!(
            "\n[+] Calculated required Cash Down Payment: ${:.2}",
            cash_amount
        );
        let cash_rate = CustomType::<f64>::new("Cash Interest / Yield Rate (%):")
            .with_validator(|val: &f64| {
                if (0.0..=20.0).contains(val) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(
                        "Cash rate must be between 0.0% and 20.0%".into(),
                    ))
                }
            })
            .prompt()?;

        tools.push(Tool::Cash(Cash {
            amount: cash_amount,
            rate: cash_rate,
        }));
    }

    if mortgage_amount > 0.0 {
        tools.push(Tool::Mortgage(Mortgage {
            amount: mortgage_amount,
            rate: mortgage_rate,
            term: mortgage_term,
        }));
    }

    if loc_amount > 0.0 {
        tools.push(Tool::Loc(Loc {
            amount: loc_amount,
            rate: loc_rate,
        }));
    }

    // 3. Interactive Extra Repayment Builder
    let mut mortgage_repay = BTreeMap::new();
    let mut loc_repay = BTreeMap::new();

    let add_extra = Confirm::new("Do you want to add extra principal repayments?").prompt()?;

    if add_extra {
        loop {
            let tool_choice = Select::new("Target Tool:", vec!["Mortgage", "LOC"]).prompt()?;
            let month = CustomType::<u32>::new("Month Number (1 - 360):")
                .with_validator(|val: &u32| {
                    if (1..=360).contains(val) {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid("Month must be between 1 and 360".into()))
                    }
                })
                .prompt()?;

            let amount = CustomType::<f64>::new("Extra Principal Repayment Amount ($):")
                .with_validator(|val: &f64| {
                    if *val > 0.0 {
                        Ok(Validation::Valid)
                    } else {
                        Ok(Validation::Invalid("Amount must be > 0.0".into()))
                    }
                })
                .prompt()?;

            match tool_choice {
                "Mortgage" => {
                    mortgage_repay.insert(month, amount);
                    println!(
                        "[+] Added Mortgage extra repayment: ${:.2} at Month {}",
                        amount, month
                    );
                }
                "LOC" => {
                    loc_repay.insert(month, amount);
                    println!(
                        "[+] Added LOC extra repayment: ${:.2} at Month {}",
                        amount, month
                    );
                }
                _ => unreachable!(),
            }

            let continue_adding = Confirm::new("Add another extra repayment rule?").prompt()?;

            if !continue_adding {
                break;
            }
        }
    }

    Ok(Purchase {
        name,
        house,
        tools,
        mortgage_repay,
        loc_repay,
    })
}
