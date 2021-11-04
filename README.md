# Reckoner

Finding the right investments for your portfolio can be hard and if we do happen to find a company or a stock that you would like to buy, we want to make sure we are making those investments at a fair price based on that company's financial performance.

Why not automate that process?

Reckoner provides a collection (only one atm) of evaluation methods to estimate a fair value for an investment i.e. a stock based on the company's financials and analyst predictions automatically. Just so we have an idea whether a certain investment is overvalued or undervalued before you deep dive into that company's business.

## Evaluation Methods

- [Discounted cash flow](https://www.investopedia.com/terms/d/dcf.asp), a method that _estimates_ the value of an investment based on cash flows. This method is great for a [blue chip company](https://www.investopedia.com/ask/answers/031915/what-qualifies-company-blue-chip.asp) such as Apple, Microsoft etc. Essentially businesses that are stable, profitable and most importantly have a growing cash flow.

- [Price-to-earnings Ratio](https://www.investopedia.com/investing/use-pe-ratio-and-peg-to-tell-stocks-future/), a method to determine whether a company's stock is overvalued or undervalued based on it's [P/E ratio](https://www.investopedia.com/terms/p/price-earningsratio.asp) and bench mark it against an index i.e. [S&P 500](https://en.wikipedia.org/wiki/S%26P_500).

_Note: P/E ratio evaluation method has not been implemented yet._

## Usage

This project currently fetches all the finacial information from [IEX Cloud](https://iexcloud.io/) which under the hood uses [Refinitiv](https://www.refinitiv.com/en) for the cash flow projections for the next two years.

- Once you have an IEX_API_KEY
- Run `IEX_API_KEY=<key> cargo run`

Example request:

```bash
    curl -d  \
    '{"ticker_symbol": "MA", "expected_return": 7.5}' \
    -H "Content-Type: application/json" \
    http://127.0.0.1:8000/stock/evaluate
```

Example response(in USD):

```json
{ "estimated_fair_value": 215, "trading_value": 330 }
```

