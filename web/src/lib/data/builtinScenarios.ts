import type { Purchase } from "../state/types";

export interface PresetScenario {
  id: string;
  filename: string;
  name: string;
  description: string;
  purchase: Purchase;
}

export const BUILTIN_SCENARIOS: PresetScenario[] = [
  {
    "filename": "all_cash_starter.json",
    "id": "all_cash_starter",
    "name": "All Cash Starter",
    "purchase": {
      "name": "All Cash Starter",
      "house": {
        "purchase_price": 450000,
        "annual_property_tax_rate": 1.15,
        "annual_insurance": 1200,
        "monthly_hoa": 50
      },
      "tools": [
        {
          "Cash": {
            "amount": 450000,
            "rate": 4.25
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "All-cash property purchase without borrowed debt service or interest friction."
  },
  {
    "filename": "balloon_final_payoff.json",
    "id": "balloon_final_payoff",
    "name": "Balloon Final Payoff",
    "purchase": {
      "name": "Balloon Final Payoff",
      "house": {
        "purchase_price": 900000,
        "annual_property_tax_rate": 1.25,
        "annual_insurance": 2200,
        "monthly_hoa": 100
      },
      "tools": [
        {
          "Cash": {
            "amount": 200000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 700000,
            "rate": 6,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {
        "60": 650000
      },
      "loc_repay": {}
    },
    "description": "Structured amortization strategy featuring scheduled early principal prepayments."
  },
  {
    "filename": "dual_loan_staggered_payoff.json",
    "id": "dual_loan_staggered_payoff",
    "name": "Dual Loan Staggered Payoff",
    "purchase": {
      "name": "Dual Loan Staggered Payoff",
      "house": {
        "purchase_price": 1600000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 3200,
        "monthly_hoa": 120
      },
      "tools": [
        {
          "Cash": {
            "amount": 300000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 1000000,
            "rate": 6,
            "term": 30
          }
        },
        {
          "Loc": {
            "amount": 300000,
            "rate": 7
          }
        }
      ],
      "mortgage_repay": {
        "36": 100000,
        "48": 100000,
        "60": 100000
      },
      "loc_repay": {
        "6": 50000,
        "12": 50000,
        "18": 100000,
        "24": 100000
      }
    },
    "description": "Hybrid multi-instrument financing combining mortgage stability and LOC flexibility."
  },
  {
    "filename": "high_inflation_holding_cost.json",
    "id": "high_inflation_holding_cost",
    "name": "High Inflation Holding Cost",
    "purchase": {
      "name": "High Inflation Holding Cost",
      "house": {
        "purchase_price": 1200000,
        "annual_property_tax_rate": 2.5,
        "annual_insurance": 6000,
        "monthly_hoa": 800
      },
      "tools": [
        {
          "Cash": {
            "amount": 240000,
            "rate": 4.5
          }
        },
        {
          "Mortgage": {
            "amount": 960000,
            "rate": 6.5,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  },
  {
    "filename": "hybrid_mortgage_loc_split.json",
    "id": "hybrid_mortgage_loc_split",
    "name": "Hybrid Mortgage LOC Split",
    "purchase": {
      "name": "Hybrid Mortgage LOC Split",
      "house": {
        "purchase_price": 1500000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 3000,
        "monthly_hoa": 150
      },
      "tools": [
        {
          "Cash": {
            "amount": 300000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 900000,
            "rate": 6,
            "term": 30
          }
        },
        {
          "Loc": {
            "amount": 300000,
            "rate": 6.5
          }
        }
      ],
      "mortgage_repay": {
        "24": 50000,
        "36": 50000
      },
      "loc_repay": {
        "6": 50000,
        "12": 50000,
        "18": 100000,
        "24": 100000
      }
    },
    "description": "Hybrid multi-instrument financing combining mortgage stability and LOC flexibility."
  },
  {
    "filename": "intermittent_seasonal_extra_pay.json",
    "id": "intermittent_seasonal_extra_pay",
    "name": "Intermittent Seasonal Extra Pay",
    "purchase": {
      "name": "Intermittent Seasonal Extra Pay",
      "house": {
        "purchase_price": 850000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 2000,
        "monthly_hoa": 75
      },
      "tools": [
        {
          "Cash": {
            "amount": 170000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 680000,
            "rate": 6.25,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {
        "12": 30000,
        "24": 30000,
        "36": 30000,
        "48": 30000,
        "60": 30000
      },
      "loc_repay": {}
    },
    "description": "Structured amortization strategy featuring scheduled early principal prepayments."
  },
  {
    "filename": "jumbo_mortgage_high_tax.json",
    "id": "jumbo_mortgage_high_tax",
    "name": "Jumbo Mortgage High Tax",
    "purchase": {
      "name": "Jumbo Mortgage High Tax",
      "house": {
        "purchase_price": 2500000,
        "annual_property_tax_rate": 2,
        "annual_insurance": 5000,
        "monthly_hoa": 250
      },
      "tools": [
        {
          "Cash": {
            "amount": 500000,
            "rate": 4.5
          }
        },
        {
          "Mortgage": {
            "amount": 2000000,
            "rate": 6.85,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  },
  {
    "filename": "low_down_fha_style.json",
    "id": "low_down_fha_style",
    "name": "Low Down FHA Style",
    "purchase": {
      "name": "Low Down FHA Style",
      "house": {
        "purchase_price": 400000,
        "annual_property_tax_rate": 1.3,
        "annual_insurance": 1500,
        "monthly_hoa": 60
      },
      "tools": [
        {
          "Cash": {
            "amount": 14000,
            "rate": 3.5
          }
        },
        {
          "Mortgage": {
            "amount": 386000,
            "rate": 6.8,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  },
  {
    "filename": "lump_sum_early_payoff.json",
    "id": "lump_sum_early_payoff",
    "name": "Lump Sum Early Payoff",
    "purchase": {
      "name": "Lump Sum Early Payoff",
      "house": {
        "purchase_price": 1000000,
        "annual_property_tax_rate": 1.25,
        "annual_insurance": 2400,
        "monthly_hoa": 100
      },
      "tools": [
        {
          "Cash": {
            "amount": 200000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 800000,
            "rate": 6.25,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {
        "12": 150000,
        "24": 150000,
        "36": 200000
      },
      "loc_repay": {}
    },
    "description": "Structured amortization strategy featuring scheduled early principal prepayments."
  },
  {
    "filename": "luxury_estate_cash_loc.json",
    "id": "luxury_estate_cash_loc",
    "name": "Luxury Estate Cash LOC",
    "purchase": {
      "name": "Luxury Estate Cash LOC",
      "house": {
        "purchase_price": 5000000,
        "annual_property_tax_rate": 1.25,
        "annual_insurance": 12000,
        "monthly_hoa": 500
      },
      "tools": [
        {
          "Cash": {
            "amount": 3000000,
            "rate": 4.5
          }
        },
        {
          "Loc": {
            "amount": 2000000,
            "rate": 5.5
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {
        "3": 200000,
        "6": 200000,
        "9": 200000,
        "12": 200000,
        "15": 200000,
        "18": 200000,
        "21": 200000,
        "24": 200000,
        "27": 200000,
        "30": 200000
      }
    },
    "description": "Pure line of credit strategy with aggressive principal acceleration."
  },
  {
    "filename": "m1.json",
    "id": "m1",
    "name": "M1",
    "purchase": {
      "name": "M1",
      "house": {
        "purchase_price": 1700000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 3600,
        "monthly_hoa": 100
      },
      "tools": [
        {
          "Cash": {
            "amount": 200000,
            "rate": 3.8
          }
        },
        {
          "Mortgage": {
            "amount": 1200000,
            "rate": 5.9,
            "term": 15
          }
        },
        {
          "Loc": {
            "amount": 300000,
            "rate": 5.5
          }
        }
      ],
      "mortgage_repay": {
        "3": 100000,
        "6": 100000,
        "12": 50000,
        "15": 50000,
        "18": 100000,
        "24": 200000
      },
      "loc_repay": {
        "6": 100000,
        "12": 100000,
        "18": 100000
      }
    },
    "description": "Hybrid multi-instrument financing combining mortgage stability and LOC flexibility."
  },
  {
    "filename": "micro_condo_high_hoa.json",
    "id": "micro_condo_high_hoa",
    "name": "Micro Condo High HOA",
    "purchase": {
      "name": "Micro Condo High HOA",
      "house": {
        "purchase_price": 350000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 1200,
        "monthly_hoa": 950
      },
      "tools": [
        {
          "Cash": {
            "amount": 70000,
            "rate": 3.8
          }
        },
        {
          "Mortgage": {
            "amount": 280000,
            "rate": 6.5,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  },
  {
    "filename": "most_conservative.json",
    "id": "most_conservative",
    "name": "most-conservative",
    "purchase": {
      "name": "most-conservative",
      "house": {
        "purchase_price": 1700000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 2400,
        "monthly_hoa": 100
      },
      "tools": [
        {
          "Cash": {
            "amount": 1000000,
            "rate": 3.8
          }
        },
        {
          "Mortgage": {
            "amount": 700000,
            "rate": 5.9,
            "term": 15
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  },
  {
    "filename": "pure_loc_aggressive_payoff.json",
    "id": "pure_loc_aggressive_payoff",
    "name": "Pure LOC Aggressive Payoff",
    "purchase": {
      "name": "Pure LOC Aggressive Payoff",
      "house": {
        "purchase_price": 800000,
        "annual_property_tax_rate": 1.1,
        "annual_insurance": 1800,
        "monthly_hoa": 0
      },
      "tools": [
        {
          "Loc": {
            "amount": 800000,
            "rate": 7
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {
        "1": 20000,
        "2": 20000,
        "3": 20000,
        "4": 20000,
        "5": 20000,
        "6": 20000,
        "7": 20000,
        "8": 20000,
        "9": 20000,
        "10": 20000,
        "11": 20000,
        "12": 20000,
        "13": 20000,
        "14": 20000,
        "15": 20000,
        "16": 20000,
        "17": 20000,
        "18": 20000,
        "19": 20000,
        "20": 20000,
        "21": 20000,
        "22": 20000,
        "23": 20000,
        "24": 20000,
        "25": 20000,
        "26": 20000,
        "27": 20000,
        "28": 20000,
        "29": 20000,
        "30": 20000,
        "31": 20000,
        "32": 20000,
        "33": 20000,
        "34": 20000,
        "35": 20000,
        "36": 20000,
        "37": 20000,
        "38": 20000,
        "39": 20000,
        "40": 20000
      }
    },
    "description": "Pure line of credit strategy with aggressive principal acceleration."
  },
  {
    "filename": "pure_loc_long.json",
    "id": "pure_loc_long",
    "name": "pure-loc-long-extra",
    "purchase": {
      "name": "pure-loc-long-extra",
      "house": {
        "purchase_price": 1700000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 2400,
        "monthly_hoa": 100
      },
      "tools": [
        {
          "Cash": {
            "amount": 0,
            "rate": 3.8
          }
        },
        {
          "Loc": {
            "amount": 1700000,
            "rate": 5.5
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {
        "1": 10000,
        "2": 10000,
        "3": 50000,
        "4": 10000,
        "5": 10000,
        "6": 50000,
        "7": 10000,
        "8": 10000,
        "9": 50000,
        "10": 10000,
        "11": 10000,
        "12": 50000,
        "13": 10000,
        "14": 10000,
        "15": 50000,
        "18": 50000,
        "21": 50000,
        "24": 50000,
        "27": 50000,
        "30": 50000,
        "33": 50000,
        "36": 50000,
        "39": 50000,
        "42": 50000,
        "45": 50000,
        "48": 50000,
        "60": 800000
      }
    },
    "description": "Pure line of credit strategy with aggressive principal acceleration."
  },
  {
    "filename": "pure_mortgage_long.json",
    "id": "pure_mortgage_long",
    "name": "pure-mortgage-long-extra",
    "purchase": {
      "name": "pure-mortgage-long-extra",
      "house": {
        "purchase_price": 1700000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 2400,
        "monthly_hoa": 100
      },
      "tools": [
        {
          "Cash": {
            "amount": 600000,
            "rate": 3.8
          }
        },
        {
          "Mortgage": {
            "amount": 1100000,
            "rate": 5.9,
            "term": 15
          }
        }
      ],
      "mortgage_repay": {
        "1": 10000,
        "2": 10000,
        "3": 50000,
        "4": 10000,
        "5": 10000,
        "6": 50000,
        "7": 10000,
        "8": 10000,
        "9": 50000,
        "10": 10000,
        "11": 10000,
        "12": 50000,
        "13": 10000,
        "14": 10000,
        "15": 50000,
        "18": 50000,
        "21": 50000,
        "24": 50000,
        "27": 50000,
        "30": 50000,
        "33": 50000,
        "36": 50000,
        "39": 50000,
        "42": 50000,
        "45": 50000,
        "48": 50000,
        "60": 800000
      },
      "loc_repay": {}
    },
    "description": "Structured amortization strategy featuring scheduled early principal prepayments."
  },
  {
    "filename": "rapid_1yr_payoff.json",
    "id": "rapid_1yr_payoff",
    "name": "Rapid 1yr Payoff",
    "purchase": {
      "name": "Rapid 1yr Payoff",
      "house": {
        "purchase_price": 600000,
        "annual_property_tax_rate": 1.1,
        "annual_insurance": 1200,
        "monthly_hoa": 0
      },
      "tools": [
        {
          "Cash": {
            "amount": 100000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 500000,
            "rate": 6,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {
        "1": 42000,
        "2": 42000,
        "3": 42000,
        "4": 42000,
        "5": 42000,
        "6": 42000,
        "7": 42000,
        "8": 42000,
        "9": 42000,
        "10": 42000,
        "11": 42000,
        "12": 42000
      },
      "loc_repay": {}
    },
    "description": "Structured amortization strategy featuring scheduled early principal prepayments."
  },
  {
    "filename": "standard_15yr_fixed.json",
    "id": "standard_15yr_fixed",
    "name": "Standard 15yr Fixed",
    "purchase": {
      "name": "Standard 15yr Fixed",
      "house": {
        "purchase_price": 800000,
        "annual_property_tax_rate": 1.2,
        "annual_insurance": 2000,
        "monthly_hoa": 80
      },
      "tools": [
        {
          "Cash": {
            "amount": 200000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 600000,
            "rate": 5.75,
            "term": 15
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  },
  {
    "filename": "standard_30yr_conventional.json",
    "id": "standard_30yr_conventional",
    "name": "Standard 30yr Conventional",
    "purchase": {
      "name": "Standard 30yr Conventional",
      "house": {
        "purchase_price": 1000000,
        "annual_property_tax_rate": 1.25,
        "annual_insurance": 2400,
        "monthly_hoa": 120
      },
      "tools": [
        {
          "Cash": {
            "amount": 200000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 800000,
            "rate": 6.5,
            "term": 30
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  },
  {
    "filename": "zero_interest_promotional.json",
    "id": "zero_interest_promotional",
    "name": "Zero Interest Promotional",
    "purchase": {
      "name": "Zero Interest Promotional",
      "house": {
        "purchase_price": 300000,
        "annual_property_tax_rate": 1,
        "annual_insurance": 1000,
        "monthly_hoa": 0
      },
      "tools": [
        {
          "Cash": {
            "amount": 50000,
            "rate": 4
          }
        },
        {
          "Mortgage": {
            "amount": 250000,
            "rate": 0,
            "term": 10
          }
        }
      ],
      "mortgage_repay": {},
      "loc_repay": {}
    },
    "description": "Conventional mortgage financing model with standard amortization timeline."
  }
];
