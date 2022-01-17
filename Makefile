help:
	@grep -E '^\w+:.*?##\s.*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

run: ## Run
	cargo run main

test: ## Tests
	cargo test
