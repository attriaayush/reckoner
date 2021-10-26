use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnualCashFlow {
    pub capital_expenditures: i64,
    pub cash_flow: i64,
    pub fiscal_year: u16,
    pub net_income: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AnnualIncomeStatement {
    pub fiscal_year: u16,
    pub total_revenue: i64,
    pub net_income: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyCashFlowResponse {
    symbol: String,
    pub cashflow: Vec<AnnualCashFlow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyIncomeStatementResponse {
    symbol: String,
    pub income: Vec<AnnualIncomeStatement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct EstimateResponse {
    pub consensus_CPX: f64,
    pub consensus_CPS: f32,
    pub consensus_NET: f64,
    pub consensus_SAL: f64,
    pub fiscal_period: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateResponseList {
    symbol: String,
    pub estimates: Vec<EstimateResponse>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estimate {
    pub cash_flow_estimates: Vec<AnnualCashFlow>,
    pub revenue_estimates: Vec<AnnualIncomeStatement>
}
