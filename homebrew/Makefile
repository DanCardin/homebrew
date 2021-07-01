.PHONY: help lint fix test build
.DEFAULT_GOAL := help

help:  ## Prints this help text
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

lint:  ## Check the code for formatting and other linter errors
	cargo check

fix:  ## Automatically fix formatting related linter errors
	cargo format

test:  ## Run all tests
	cargo test

build:  ## Build the project
	cargo build
	docker build -t homebrew-web -f Dockerfile ..
