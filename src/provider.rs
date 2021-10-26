use super::models::*;

use surf::Result;

use crate::{env_var, request};

const MILLION: f64 = 1000000.00;

#[derive(Debug)]
pub struct FinancialsProvider {
    base_url: String,
    token: String,
}

#[derive(Debug)]
pub enum Period {
    Annual,
    // Quarter,
}

fn validate_requested_period(requested_period: &Period) -> &str {
    let period = match requested_period {
        Period::Annual => "annual",
        // Period::Quarter => "quarter",
    };
    period
}

fn normalize_estimates_data(
    estimate_resp: EstimateResponseList,
    shares_outstanding: i64,
) -> (Vec<AnnualCashFlow>, Vec<AnnualIncomeStatement>) {
    let mut cash_flow_estimates = Vec::<AnnualCashFlow>::new();
    let mut revenue_estimates = Vec::<AnnualIncomeStatement>::new();

    for estimate in estimate_resp.estimates.iter() {
        let fiscal_year = estimate.fiscal_period.split("FY ").collect::<Vec<_>>()[1]
            .parse::<u16>()
            .unwrap();
        let net_income = (estimate.consensus_NET * MILLION) as i64;

        cash_flow_estimates.push(AnnualCashFlow {
            capital_expenditures: -(estimate.consensus_CPX * MILLION) as i64,
            cash_flow: ((estimate.consensus_CPS as f64) * (shares_outstanding as f64)) as i64,
            fiscal_year,
            net_income,
        });

        revenue_estimates.push(AnnualIncomeStatement {
            fiscal_year,
            total_revenue: (estimate.consensus_SAL * MILLION) as i64,
            net_income,
        });
    }

    (cash_flow_estimates, revenue_estimates)
}

impl FinancialsProvider {
    pub fn new() -> FinancialsProvider {
        FinancialsProvider {
            base_url: env_var("IEX_API_URL"),
            token: env_var("IEX_API_KEY"),
        }
    }

    pub async fn request_cash_flow(
        &self,
        ticker_symbol: &str,
        period: &Period,
        last: i8,
    ) -> Result<CompanyCashFlowResponse> {
        let period = validate_requested_period(&period);

        let url = format!(
            "{base_url}/stock/{ticker_symbol}/cash-flow?period={period}&last={last}&token={token}",
            base_url = self.base_url,
            token = self.token,
            ticker_symbol = ticker_symbol,
            period = period.to_string(),
            last = last
        );

        let data = request::get(&url).await?;
        let cash_flow = serde_json::from_str(&data)?;

        Ok(cash_flow)
    }

    pub async fn request_income_statement(
        &self,
        ticker_symbol: &str,
        period: &Period,
        last: i8,
    ) -> Result<CompanyIncomeStatementResponse> {
        let period = validate_requested_period(&period);

        let url = format!(
            "{base_url}/stock/{ticker_symbol}/income?period={period}&last={last}&token={token}",
            base_url = self.base_url,
            token = self.token,
            ticker_symbol = ticker_symbol,
            period = period.to_string(),
            last = last
        );

        let data = request::get(&url).await?;
        let income_statement = serde_json::from_str(&data)?;

        Ok(income_statement)
    }

    pub async fn request_outstanding_shares(&self, ticker_symbol: &str) -> Result<i64> {
        let url = format!(
            "{base_url}/stock/{ticker_symbol}/stats/sharesOutstanding?token={token}",
            base_url = self.base_url,
            token = self.token,
            ticker_symbol = ticker_symbol
        );

        let data = request::get(&url).await?;
        let outstanding_shares = serde_json::from_str(&data)?;

        Ok(outstanding_shares)
    }

    pub async fn request_estimates(
        &self,
        ticker_symbol: &str,
        period: &Period,
        last: i8,
        shares_outstanding: i64,
    ) -> Result<Estimate> {
        let period = validate_requested_period(&period);

        let url = format!(
            "{base_url}/stock/{ticker_symbol}/estimates?period={period}&last={last}&token={token}",
            base_url = self.base_url,
            token = self.token,
            ticker_symbol = ticker_symbol,
            period = period.to_string(),
            last = last
        );

        let data = request::get(&url).await?;
        let estimate_resp: EstimateResponseList = serde_json::from_str(&data)?;

        let (mut cash_flow_estimates, mut revenue_estimates) =
            normalize_estimates_data(estimate_resp, shares_outstanding);

        cash_flow_estimates.reverse();
        revenue_estimates.reverse();

        Ok(Estimate {
            cash_flow_estimates,
            revenue_estimates,
        })
    }
}
