use anyhow::Result;
use rocket::serde::{Deserialize, Serialize};

use crate::method::DiscountedFreeCashflow;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Stock {
    pub ticker_symbol: String,
    pub expected_return: f64,
}

impl Stock {
    pub fn new(self) -> Self {
        Stock {
            ticker_symbol: self.ticker_symbol,
            expected_return: self.expected_return,
        }
    }

    pub async fn perform_discounted_free_cash_flow(&self) -> Result<i64> {
        // TODO:
        // Validate whether discounted free cash flow can be performed on the requested stock
        let estimated_fair_value = DiscountedFreeCashflow::financials(&self)
            .await?
            .adjust_projected_estimates()
            .project_fair_value(self.expected_return, 2.50);

        Ok(estimated_fair_value)
    }
}
