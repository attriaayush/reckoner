# Reckoner

Finding the right investments for your portfolio can be hard and if we do happen to find a company or a stock that we would like to buy, we want to make sure we are paying a fair price based on that company's financial performance.

AND to do that, we have to scan through company's financial documentation. YUCK.

Why not automate that process?

Reckoner aims to reckon a fair price for an investment i.e. evaluating a company's financials and analyst predictions automatically. Just so we can have an idea whether a certain investment is overvalued or undervalued before we deep dive into that company's business or before we make an investment.

## Evaluation Methods

- [Discounted cash flow](https://www.investopedia.com/terms/d/dcf.asp), a method that _estimates_ the value of an investment based on cash flows. This method is great for a [blue chip company](https://www.investopedia.com/ask/answers/031915/what-qualifies-company-blue-chip.asp) such as Apple, Microsoft etc. Essentially businesses that are stable, profitable and most importantly have a growing cash flow.

- [Price-to-earnings Ratio](https://www.investopedia.com/investing/use-pe-ratio-and-peg-to-tell-stocks-future/), a method to determine whether a company's stock is overvalued or undervalued based on it's [P/E ratio](https://www.investopedia.com/terms/p/price-earningsratio.asp) and bench mark it against an index i.e. [S&P 500](https://en.wikipedia.org/wiki/S%26P_500).

_*Note: P/E ratio evaluation method has not been implemented yet.*_

## Usage

Using [IEX Cloud](https://iexcloud.io/) as financials provider which under the hood uses [Refinitiv](https://www.refinitiv.com/en) for the cash flow projections and estimates.

- Once you have an IEX_API_KEY.
- Run `IEX_API_KEY=<key> cargo run`.
- The `/stock/evaluate` endpoint accepts `ticker_symbol` i.e. company's stock symbol and your `expected_return`. The stock's fair value in other words your buying entry point will be calculated based on your expected return.

Example request:

```bash
curl -d '{"ticker_symbol": "MA", "expected_return": 7.5}' -H "Content-Type: application/json" http://127.0.0.1:8000/stock/evaluate
```

Example response(in USD):

```json
{ "estimated_fair_value": 215 }
```
