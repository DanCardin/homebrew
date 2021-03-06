.PHONY: help lint test build
.DEFAULT_GOAL := help

help:  ## Prints this help text
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

lint:  ## Check the code for formatting and other linter errors
	make -C homebrew lint
	make -C homebrew-web lint
	make -C homebrew-ui lint

test:  ## Run all tests
	make -C homebrew test
	make -C homebrew-web test
	make -C homebrew-ui test

build:  ## Build the project
	make -C homebrew build
	make -C homebrew-web build
	make -C homebrew-ui build
