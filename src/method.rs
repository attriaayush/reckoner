use anyhow::{anyhow, Result};
use iex_provider::{
    models::{EstimateResponse, EstimateResponseList},
    provider::{Financials, Period},
};
use rocket::tokio::try_join;

use crate::{evaluate::Stock, wacc::Wacc};

const LAST: i8 = 2;
const MILLION: f64 = 1000000.00;

#[derive(Debug)]
pub struct DiscountedFreeCashflow {
    outstanding_shares: i64,
    projections: EstimateResponseList,
    required_rate_of_return: f64
}

impl DiscountedFreeCashflow {
    fn discount_estimates_to_today(&self, expected_return: &f64) -> i64 {
        let total_discounted_value_today: i64 = self
            .projections
            .estimates
            .iter()
            .enumerate()
            .map(|(index, estimate)| {
                (estimate.consensus_CPS / (1.0 + expected_return).powf((index + 1) as f64)) as i64
            })
            .sum();

        total_discounted_value_today
    }

    fn discounted_terminal_value(&self, expected_return: &f64, perpetual_growth: &f64) -> i64 {
        let mut terminal_value = self.projections.estimates.last().unwrap().consensus_CPS
            * (1.0 + perpetual_growth)
            / (expected_return - perpetual_growth);

        terminal_value = terminal_value / (1.0 + expected_return).powf(LAST as f64);
        terminal_value as i64
    }

    pub fn adjust_projected_estimates(self) -> Self {
        DiscountedFreeCashflow {
            projections: EstimateResponseList {
                estimates: self
                    .projections
                    .estimates
                    .iter()
                    .map(|estimate| EstimateResponse {
                        consensus_CPS: estimate.consensus_CPS * (self.outstanding_shares as f64),
                        consensus_NET: estimate.consensus_NET * MILLION,
                        consensus_SAL: estimate.consensus_SAL * MILLION,
                        consensus_CPX: -(estimate.consensus_CPX * MILLION),
                        fiscal_period: estimate.fiscal_period.clone(),
                    })
                    .collect(),
                ..self.projections
            },
            ..self
        }
    }

    pub async fn financials(e: &Stock) -> Result<Self> {
        let provider = Financials::new(&e.ticker_symbol, &Period::Annual, LAST);

        let response = try_join!(
            provider.request_income_statement(),
            provider.request_balance_sheet(),
            provider.request_ten_year_treasury_rate(),
            provider.request_company_stats(),
            provider.request_estimates()
        );

        match response {
            Ok((income_statement, balance, treasury_rate, stats, projections)) => {
                let income_statement = income_statement.income.first().unwrap();
                let balance_sheet = balance.balancesheet.first().unwrap();

                let required_rate_of_return = Wacc {
                    interest_income: income_statement.interest_income,
                    long_term_debt: balance_sheet.long_term_debt,
                    total_current_liabilities: balance_sheet.total_current_liabilities,
                    pre_tax: income_statement.pretax_income,
                    income_tax: income_statement.income_tax,
                    ten_year_treasury_rate: treasury_rate,
                    beta: stats.beta,
                    market_cap: stats.marketcap,
                }.generate_wacc();

                Ok(DiscountedFreeCashflow {
                    outstanding_shares: stats.shares_outstanding,
                    projections,
                    required_rate_of_return
                })
            }
            Err(e) => {
                warn!("{}", e);
                Err(anyhow!(e))
            }
        }
    }

    pub fn project_fair_value(&self, perpetual_growth: f64) -> i64 {
        let perpetual_growth = perpetual_growth / 100.00;
        let expected_return = self.required_rate_of_return / 100.00;

        let discounted_terminal_value =
            self.discounted_terminal_value(&expected_return, &perpetual_growth);

        let discounted_estimates_total = self.discount_estimates_to_today(&expected_return);

        let fair_value = discounted_estimates_total + discounted_terminal_value;
        fair_value / self.outstanding_shares
    }
}
