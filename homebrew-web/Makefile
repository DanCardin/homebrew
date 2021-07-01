.PHONY: help setup lint lock fix test run build
.DEFAULT_GOAL := help

help:  ## Prints this help text
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup:  ## Installs dependencies for local development
	cargo install systemfd watchexec-cli
	cargo build

lint:  ## Check the code for formatting and other linter errors
	cargo sqlx prepare --check

lock:
	cargo sqlx prepare

fix:  ## Automatically fix formatting related linter errors
	cargo format

test:  ## Run all tests
	cargo test

run:
	systemfd --no-pid -s http::6273 -- watchexec -r --exts rs,toml cargo run | jq -R 'fromjson?'

build:  ## Build the project
	cargo build
	docker build -t homebrew-web -f Dockerfile ..
