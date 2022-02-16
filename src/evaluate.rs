use crate::method::DiscountedFreeCashflow;
use anyhow::Result;

#[derive(Debug)]
pub struct FairValue {
    pub fair_value: i64,
    pub stock: String,
}

pub async fn perform_discounted_free_cash_flow(ticker_symbol: String) -> Result<FairValue> {
    let stock = ticker_symbol.clone();
    let estimated_fair_value = DiscountedFreeCashflow::financials(ticker_symbol)
        .await?
        .adjust_projected_estimates()
        .project_fair_value(2.50);

    let value = FairValue {
        fair_value: estimated_fair_value,
        stock,
    };

    Ok(value)
}
