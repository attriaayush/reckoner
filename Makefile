.PHONY: release
release: 
	cargo build --release && cp target/release/reckoner ~/.cargo/bin

.PHONY: build
build: 
	cargo build

.PHONY: run-example
run-example: 
	 cargo run --  --tickers "AAPL, GOOGL, MSFT"

