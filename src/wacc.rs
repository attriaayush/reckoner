use serde::{Deserialize, Serialize};

// This is derived from the average S&P500 growth
// going back to 1920s
// Instead of polling and calculating this everytime,
// hard-coding it to 10% yearly growth
const PRESUMED_MARKET_GROWTH: f64 = 10.00;

// Weighted Average cost of capital
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wacc {
    pub interest_income: f64,
    pub long_term_debt: f64,
    pub total_current_liabilities: f64,

    pub pre_tax: f64,
    pub income_tax: f64,

    pub ten_year_treasury_rate: f64,
    pub beta: f64,
    pub market_cap: f64,
}

impl Wacc {
    fn effective_tax_rate(&self) -> f64 {
        self.income_tax / self.pre_tax
    }

    fn cost_to_debt(&self) -> (f64, f64) {
        let total_debt = self.long_term_debt + self.total_current_liabilities;
        let cost_to_debt = self.interest_income / total_debt;
        (total_debt, cost_to_debt)
    }

    fn cost_of_equity(&self, total_debt: f64) -> (f64, f64) {
        let updated_debt = self.market_cap + total_debt;
        let total_cap_structure = total_debt / updated_debt;
        let equity = 1.00 - total_cap_structure;

        (equity, total_cap_structure)
    }

    fn capital_asset_pricing_model(&self) -> f64 {
        self.ten_year_treasury_rate
            + (self.beta * (PRESUMED_MARKET_GROWTH - self.ten_year_treasury_rate))
    }

    pub fn generate_wacc(&self) -> f64 {
        let (total_debt, cost_to_debt) = self.cost_to_debt();
        let adjusted_debt = cost_to_debt * self.effective_tax_rate();
        let (equity, total_cap_structure) = self.cost_of_equity(total_debt);

        total_cap_structure * (1.00 - adjusted_debt) + equity * self.capital_asset_pricing_model()
    }
}
