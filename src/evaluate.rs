use anyhow::Result;
use rocket::serde::{Deserialize, Serialize};

use crate::method::DiscountedFreeCashflow;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Evaluate {
    pub ticker_symbol: String,
    pub expected_return: f64,
}

impl Evaluate {
    pub fn new(self) -> Self {
        Evaluate {
            ticker_symbol: self.ticker_symbol,
            expected_return: self.expected_return,
        }
    }

    pub async fn perform_discounted_free_cash_flow(&self) -> Result<()> {
        // TODO:
        // a. Validate whether discounted free cash flow can be performed on the requested stock
        // b. Have clearer error messages
        let estimated_fair_value = DiscountedFreeCashflow::financials(&self)
            .await?
            .adjust_projected_estimates()
            .project_fair_value(self.expected_return, 2.50);

        println!("Fair value is {}", estimated_fair_value);

        Ok(())
    }
}
