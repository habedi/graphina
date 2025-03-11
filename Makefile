# Variables
PKG             := github.com/habedi/graphina
BINARY_NAME     := $(or $(PROJ_BINARY), $(notdir $(PKG)))
BINARY          := target/release/$(BINARY_NAME)
PATH            := /snap/bin:$(PATH)
DEBUG_GRAPHINA  := 1
RUST_LOG        := info
RUST_BACKTRACE  := full
WHEEL_DIR       := dist
PYGRAPHINA_DIR  := pygraphina
TEST_DATA_DIR  := tests/testdata
SHELL           := /bin/bash
MSRV			:= 1.83

# Find the latest built Python wheel file
WHEEL_FILE := $(shell ls $(PYGRAPHINA_DIR)/$(WHEEL_DIR)/pygraphina-*.whl 2>/dev/null | head -n 1)

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show the list of available targets with their descriptions
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

########################################################################################
## Rust targets
########################################################################################

.PHONY: format
format: ## Format Rust files
	@echo "Formatting Rust files..."
	@cargo fmt

.PHONY: test
test: format ## Run the tests
	@echo "Running tests..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) RUST_LOG=debug RUST_BACKTRACE=$(RUST_BACKTRACE) cargo test -- --nocapture

.PHONY: coverage
coverage: format ## Generate test coverage report
	@echo "Generating test coverage report..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo tarpaulin --out Xml --out Html

.PHONY: build
build: format ## Build the binary for the current platform
	@echo "Building the project..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo build --release

.PHONY: run
run: build ## Build and run the binary
	@echo "Running binary: $(BINARY)"
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) ./$(BINARY)

.PHONY: clean
clean: ## Remove generated and temporary files
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf $(WHEEL_DIR) dist/ $(PYGRAPHINA_DIR)/$(WHEEL_DIR)
	@rm -f $(PYGRAPHINA_DIR)/*.so

.PHONY: install-snap
install-snap: ## Install dependencies using Snapcraft
	@echo "Installing snap dependencies..."
	@sudo apt-get update && sudo apt-get install -y snapd
	@sudo snap refresh
	@sudo snap install rustup --classic

.PHONY: install-deps
install-deps: install-snap ## Install development dependencies
	@echo "Installing development dependencies..."
	@rustup component add rustfmt clippy
	@cargo install cargo-tarpaulin
	@cargo install cargo-audit
	@cargo install cargo-nextest

.PHONY: lint
lint: format ## Run linters on Rust files
	@echo "Linting Rust files..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo clippy -- -D warnings

.PHONY: publish
publish: ## Publish the package to crates.io (requires CARGO_REGISTRY_TOKEN to be set)
	@echo "Publishing package to Cargo registry..."
	@cargo publish --token $(CARGO_REGISTRY_TOKEN)

.PHONY: bench
bench: ## Run benchmarks
	@echo "Running benchmarks..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo bench

.PHONY: audit
audit: ## Run security audit on Rust dependencies
	@echo "Running security audit..."
	@cargo audit

.PHONY: doc
doc: format ## Generate the documentation
	@echo "Generating documentation..."
	@cargo doc --no-deps --document-private-items

.PHONY: fix-lint
fix_lint: ## Fix the linter warnings
	@echo "Fixing linter warnings..."
	@cargo clippy --fix --allow-dirty --allow-staged

.PHONY: nextest
nextest: ## Run tests using nextest
	@echo "Running tests using nextest..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo nextest run

.PHONY: testdata
testdata: ## Download the datasets used in tests
	@echo "Downloading test data..."
	@$(SHELL) $(TEST_DATA_DIR)/download_datasets.sh $(TEST_DATA_DIR)

.PHONY: install-msrv
install-msrv: ## Install the minimum supported Rust version (MSRV) for development
	@echo "Installing the minimum supported Rust version..."
	@rustup toolchain install $(MSRV)
	@rustup default $(MSRV)

.PHONY: run-examples
run-examples: ## Run all the scripts in the examples directory one by one
	@echo "Running all example scripts..."
	@for example in examples/*.rs; do \
		example_name=$$(basename $$example .rs); \
		echo "Running example: $$example_name"; \
		cargo run --example $$example_name; \
	done

########################################################################################
## Python targets
########################################################################################

.PHONY: develop_py
develop_py: ## Build and install PyGraphina in the current Python environment
	@echo "Building and installing PyGraphina..."
	@(cd $(PYGRAPHINA_DIR) && maturin develop)

.PHONY: wheel
wheel: ## Build the wheel file for PyGraphina
	@echo "Building the PyGraphina wheel..."
	@(cd $(PYGRAPHINA_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check)

.PHONY: wheel-manylinux
wheel-manylinux: ## Build the manylinux wheel file for PyGraphina (using Zig)
	@echo "Building the manylinux PyGraphina wheel..."
	@(cd $(PYGRAPHINA_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check --zig)

.PHONY: test_py
test_py: develop_py ## Run Python tests
	@echo "Running Python tests..."
	@poetry run pytest $(PYGRAPHINA_DIR)/tests

publish_py: wheel ## Publish the PyGraphina wheel to PyPI (requires PYPI_TOKEN to be set)
	@echo "Publishing PyGraphina to PyPI..."
	@if [ -z "$(WHEEL_FILE)" ]; then \
		echo "Error: No wheel file found. Please run 'make wheel' first."; \
		exit 1; \
	fi
	@echo "Found wheel file: $(WHEEL_FILE)"
	@twine upload -u __token__ -p $(PYPI_TOKEN) $(WHEEL_FILE)

.PHONY: generate_ci
generate_ci: ## Generate CI configuration files (GitHub Actions workflow)
	@echo "Generating CI configuration files..."
	@(cd $(PYGRAPHINA_DIR) && maturin generate-ci --zig --pytest --platform all -o ../.github/workflows/ci.yml github)
