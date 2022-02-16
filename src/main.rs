mod evaluate;
mod method;
mod wacc;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ticker symbols or symbol separated by comma e.g. "AAPL,GOOGL"
    #[clap(long, short)]
    tickers: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse().tickers;
    let tasks: Vec<_> = args
        .split(',')
        .map(|sub| sub.trim().to_owned())
        .map(|ticker| tokio::task::spawn(evaluate::perform_discounted_free_cash_flow(ticker)))
        .collect();

    for task in tasks {
        let result = task.await.unwrap().unwrap();
        println!("Fair value for {} is {}", result.stock, result.fair_value);
    }
}
