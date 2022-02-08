use super::{models::*, request, IEX_API_KEY, IEX_BASE_URL};
use anyhow::Result;

#[derive(Debug)]
pub enum Period {
    Annual,
    Quarter,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Period::Annual => write!(f, "annual"),
            Period::Quarter => write!(f, "quarter"),
        }
    }
}

#[derive(Debug)]
pub struct Financials<'a> {
    ticker_symbol: &'a str,
    period: &'a Period,
    last: i8,
}

impl<'a> Financials<'a> {
    pub fn new(ticker_symbol: &'a str, period: &'a Period, last: i8) -> Self {
        Financials {
            ticker_symbol,
            period,
            last,
        }
    }

    pub async fn request_cash_flow(&self) -> Result<CompanyCashFlowResponse> {
        let url = format!(
            "{base_url}/stock/{ticker_symbol}/cash-flow?period={period}&last={last}&token={token}",
            base_url = IEX_BASE_URL,
            token = IEX_API_KEY,
            ticker_symbol = self.ticker_symbol,
            period = self.period,
            last = self.last
        );

        let cashflow = request::get::<CompanyCashFlowResponse>(&url).await?;
        Ok(cashflow)
    }

    pub async fn request_estimates(&self) -> Result<EstimateResponseList> {
        let url = format!(
            "{base_url}/stock/{ticker_symbol}/estimates?period={period}&last={last}&token={token}",
            base_url = IEX_BASE_URL,
            token = IEX_API_KEY,
            ticker_symbol = self.ticker_symbol,
            period = self.period,
            last = self.last
        );

        let estimates = request::get::<EstimateResponseList>(&url).await?;
        Ok(estimates)
    }

    pub async fn request_income_statement(&self) -> Result<CompanyIncomeStatementResponse> {
        let url = format!(
            "{base_url}/stock/{ticker_symbol}/income?period={period}&last={last}&token={token}",
            base_url = IEX_BASE_URL,
            token = IEX_API_KEY,
            ticker_symbol = self.ticker_symbol,
            period = self.period,
            last = self.last
        );

        let income_statement = request::get::<CompanyIncomeStatementResponse>(&url).await?;
        Ok(income_statement)
    }

    pub async fn request_company_stats(&self) -> Result<CompanyStats> {
        let url = format!(
            "{base_url}/stock/{ticker_symbol}/stats?token={token}",
            base_url = IEX_BASE_URL,
            token = IEX_API_KEY,
            ticker_symbol = self.ticker_symbol
        );

        let stats = request::get::<CompanyStats>(&url).await.unwrap();
        Ok(stats)
    }

    pub async fn request_balance_sheet(&self) -> Result<BalanceSheetResponse> {
        let url = format!(
            "{base_url}/stock/{ticker_symbol}/balance-sheet?period={period}&token={token}",
            base_url = IEX_BASE_URL,
            token = IEX_API_KEY,
            ticker_symbol = self.ticker_symbol,
            period = self.period
        );

        let balance_sheet = request::get::<BalanceSheetResponse>(&url).await.unwrap();
        Ok(balance_sheet)
    }

    pub async fn request_ten_year_treasury_rate(&self) -> Result<f64> {
        let url = format!(
            "{base_url}/time-series/treasury/DGS10?token={token}",
            base_url = IEX_BASE_URL,
            token = IEX_API_KEY,
        );

        let response = request::get::<Vec<TreasuryRate>>(&url).await.unwrap();
        let rate = response[0].value;
        Ok(rate)
    }
}
