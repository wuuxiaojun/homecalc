//! tool.rs
//! Financial instruments
use serde::{Deserialize, Serialize};

/// Cash
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Cash {
    pub amount: f64,
    pub rate: f64, // annual cash yield (e.g. 3.9%)
}

/// Mortgage
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Mortgage {
    pub amount: f64,
    pub rate: f64, // annual mortgage interest rate (e.g. 6.0%)
    pub term: u32, // term in years
}

/// Line of Credit (LOC)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Loc {
    pub amount: f64,
    pub rate: f64, // annual loc interest rate (e.g. 5.5%)
}

/// Enum type for financial tools
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tool {
    Mortgage(Mortgage),
    Loc(Loc),
    Cash(Cash),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_instantiation() {
        let cash = Cash {
            amount: 200_000.0,
            rate: 4.5,
        };
        let mortgage = Mortgage {
            amount: 800_000.0,
            rate: 6.25,
            term: 30,
        };
        let loc = Loc {
            amount: 150_000.0,
            rate: 7.0,
        };

        let t_cash = Tool::Cash(cash);
        let t_mort = Tool::Mortgage(mortgage);
        let t_loc = Tool::Loc(loc);

        assert_eq!(t_cash, Tool::Cash(cash));
        assert_eq!(t_mort, Tool::Mortgage(mortgage));
        assert_eq!(t_loc, Tool::Loc(loc));
    }
}
