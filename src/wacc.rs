use serde::{Deserialize, Serialize};

// Weighted Average cost of capital
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Wacc {
    pub interest_income: f64,
    pub long_term_debt: f64,
    pub total_current_liabilities: f64,
    pub pre_tax: f64,
    pub income_tax: f64,
}

impl Wacc {
    fn effective_tax_rate(&self) -> f64 {
        self.income_tax / self.pre_tax
    }

    fn cost_to_debt(&self) -> f64 {
        let total_debt = self.long_term_debt + self.total_current_liabilities;
        let cost_to_debt = self.interest_income / total_debt;
        cost_to_debt
    }
}
