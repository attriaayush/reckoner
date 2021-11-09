use anyhow::{anyhow, Result};
use iex_provider::{
    models::{EstimateResponse, EstimateResponseList},
    provider::{Financials, Period},
};
use rocket::tokio::try_join;

use crate::evaluate::Stock;

const LAST: i8 = 2;
const MILLION: f64 = 1000000.00;

#[derive(Debug)]
pub struct DiscountedFreeCashflow {
    outstanding_shares: i64,
    projections: EstimateResponseList,
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

        let outstanding_shares = provider.request_outstanding_shares();
        let projections = provider.request_estimates();

        let response = try_join!(outstanding_shares, projections);

        match response {
            Ok((outstanding_shares, projections)) => Ok(DiscountedFreeCashflow {
                outstanding_shares,
                projections,
            }),
            Err(e) => {
                warn!("{}", e);
                Err(anyhow!(e))
            }
        }
    }

    pub fn project_fair_value(&self, expected_return: f64, perpetual_growth: f64) -> i64 {
        let perpetual_growth = perpetual_growth / 100.00;
        let expected_return = expected_return / 100.00;

        let discounted_terminal_value =
            self.discounted_terminal_value(&expected_return, &perpetual_growth);

        let discounted_estimates_total = self.discount_estimates_to_today(&expected_return);

        let fair_value = discounted_estimates_total + discounted_terminal_value;
        fair_value / self.outstanding_shares
    }
}
