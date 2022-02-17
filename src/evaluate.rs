use crate::method::DiscountedFreeCashflow;
use anyhow::Result;

#[derive(Debug)]
pub struct Stock {
    pub ticker: String,
    pub fair_value: Option<i64>,
}

impl Stock {
    pub fn new(ticker: String) -> Self {
        Stock {
            ticker,
            fair_value: None,
        }
    }

    pub async fn perform_discounted_free_cash_flow(self) -> Result<Stock> {
        let ticker = self.ticker.clone();
        let estimated_fair_value = DiscountedFreeCashflow::financials(self.ticker)
            .await?
            .adjust_projected_estimates()
            .project_fair_value(2.50);

        let value = Stock {
            ticker,
            fair_value: Some(estimated_fair_value),
        };

        Ok(value)
    }
}
