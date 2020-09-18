.PHONY: help setup lint fix test build
.DEFAULT_GOAL := help

help:  ## Prints this help text
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup:  ## Installs dependencies for local development
	npm install
	cargo install watchexec
	cargo build

lint:  ## Check the code for formatting and other linter errors
	npm run lint

fix:  ## Automatically fix formatting related linter errors
	npm run fix

test:  ## Run all tests
	cargo test

build:  ## Build the project
	npm run build
	cargo build
	docker build -t homebrew .
