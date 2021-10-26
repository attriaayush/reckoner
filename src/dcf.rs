use core::f32;
use std::usize;

use anyhow::Result;
use futures::try_join;
use tide::prelude::*;
use tide::Request;

use crate::models::*;
use crate::provider;

const PERIOD_OF_YEARS: i8 = 2;

#[derive(Debug, Deserialize)]
struct EvaluateStock {
    name: String,
    expected_return: f64,
}

#[derive(Debug, Deserialize, Copy, Clone)]
struct CompanyIncome {
    revenue: i64,
    net_income: i64,
    free_cash_flow: i64,
    fiscal_year: u16,
}

fn equity_to_cash_flow(
    cash_flow: &Vec<AnnualCashFlow>,
    income: &Vec<AnnualIncomeStatement>,
) -> (Vec<CompanyIncome>, Result<Option<f32>>) {
    let mut cash_flow_with_income = Vec::<CompanyIncome>::new();

    let equity_to_cash_flow = cash_flow
        .iter()
        .enumerate()
        .map(|(index, annual_flow)| {
            let free_cash_flow = annual_flow.cash_flow + annual_flow.capital_expenditures;
            let free_cash_alignment_to_income =
                (free_cash_flow as f32 / annual_flow.net_income as f32) * 100.00;

            if annual_flow.fiscal_year == income[index].fiscal_year {
                cash_flow_with_income.push(CompanyIncome {
                    revenue: income[index].total_revenue,
                    net_income: annual_flow.net_income,
                    free_cash_flow: free_cash_flow,
                    fiscal_year: annual_flow.fiscal_year,
                });
            }

            free_cash_alignment_to_income
        })
        .min_by(|a, b| a.partial_cmp(&b).unwrap());

    (cash_flow_with_income, Ok(equity_to_cash_flow))
}

fn calc_average_growth(income_statement: &Vec<AnnualIncomeStatement>) -> (f32, i32) {
    let mut least_net_income_margin = Vec::<i32>::with_capacity(PERIOD_OF_YEARS as usize);

    let yearly_revenue_growth_timeline = (0..(PERIOD_OF_YEARS - 1) as usize)
        .map(|index| {
            let average_revenue_growth = (((income_statement[index].total_revenue as f32
                - income_statement[index + 1].total_revenue as f32)
                / income_statement[index + 1].total_revenue as f32)
                * 100.00) as i32;

            least_net_income_margin.push(
                ((income_statement[index].net_income as f64
                    / income_statement[index].total_revenue as f64)
                    * 100.00) as i32,
            );

            average_revenue_growth
        })
        .collect::<Vec<_>>();

    let average_revenue_growth = if yearly_revenue_growth_timeline.len() > 1 {
        yearly_revenue_growth_timeline.iter().sum::<i32>() as f32
            / (yearly_revenue_growth_timeline.len() - 1) as f32
    } else {
        yearly_revenue_growth_timeline[0] as f32
    };

    let least_net_income_margin = least_net_income_margin.iter().min().unwrap();

    (average_revenue_growth, least_net_income_margin.clone())
}

fn project_earnings(
    company_income: &Vec<CompanyIncome>,
    average_revenue_growth: &f32,
    least_net_income_margin: &i32,
    average_annual_equity_to_cash: &f32,
    years: usize,
) -> Result<Vec<CompanyIncome>> {
    let mut projections = Vec::<CompanyIncome>::with_capacity(years);
    projections.push(company_income[0]);

    for index in 1..years {
        let projected_revenue = projections[index - 1].revenue
            + ((projections[index - 1].revenue as f64 * (*average_revenue_growth as f64 / 100.00))
                as i64);

        let projected_net_income =
            (projected_revenue as f64 * (*least_net_income_margin as f64 / 100.00)) as i64;

        let new_fiscal_year = projections[index - 1].fiscal_year + 1;

        projections.push(CompanyIncome {
            revenue: projected_revenue,
            net_income: projected_net_income,
            free_cash_flow: (projected_net_income as f64
                * (*average_annual_equity_to_cash as f64 / 100.00))
                as i64,
            fiscal_year: new_fiscal_year,
        })
    }

    projections.remove(0);
    projections.reverse();
    Ok(projections)
}

pub async fn calculate_dcf(mut req: Request<()>) -> tide::Result {
    let EvaluateStock {
        name,
        expected_return,
    } = req.body_json().await?;

    let perpetual_growth: f32 = 3.5;

    let provider = provider::FinancialsProvider::new();
    let period = provider::Period::Annual;

    let shares_outstanding = provider.request_outstanding_shares(&name).await?;

    let cash_flow = provider.request_cash_flow(&name, &period, PERIOD_OF_YEARS);
    let income_statement = provider.request_income_statement(&name, &period, PERIOD_OF_YEARS);
    let with_estimates =
        provider.request_estimates(&name, &period, PERIOD_OF_YEARS, shares_outstanding);

    let (cash_flow, income_statement, mut with_estimates) =
        try_join!(cash_flow, income_statement, with_estimates)?;

    with_estimates
        .cash_flow_estimates
        .extend(cash_flow.cashflow);
    with_estimates
        .revenue_estimates
        .extend(income_statement.income);

    let (average_revenue_growth, least_net_income_margin) =
        calc_average_growth(&with_estimates.revenue_estimates);

    let (company_income, average_annual_equity_to_cash) = equity_to_cash_flow(
        &with_estimates.cash_flow_estimates,
        &with_estimates.revenue_estimates,
    );

    let projections = project_earnings(
        &company_income,
        &average_revenue_growth,
        &least_net_income_margin,
        &average_annual_equity_to_cash?.unwrap(),
        3,
    )?;

    let projected_discount_free_cash_flow = projections
        .iter()
        .enumerate()
        .map(|(index, projection)| {
            ((projection.free_cash_flow as f64)
                / (1.0 + (expected_return / 100.00)).powf(index as f64 + 1.0)) as i64
        })
        .collect::<Vec<_>>();

    let terminal_value = ((((projections[0].free_cash_flow as f64)
        * (1.0 + (perpetual_growth / 100.00) as f64))
        / ((expected_return / 100.00) - (perpetual_growth / 100.00) as f64))
        / (1.0 + (expected_return / 100.00)).powf(4.0)) as i64;

    let mut total_value_today: i64 = projected_discount_free_cash_flow.iter().sum();
    total_value_today = total_value_today + terminal_value;

    let fair_value = total_value_today / shares_outstanding;

    Ok(format!("{}", fair_value).into())
}
