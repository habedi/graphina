# Variables
PATH            := /snap/bin:$(PATH)
DEBUG_GRAPHINA  := 1
RUST_LOG        := info
RUST_BACKTRACE  := full
WHEEL_DIR       := dist
PYGRAPHINA_DIR  := pygraphina
PY_DEP_MNGR := uv # Use `uv sync --all-extras` to make the environment
TEST_DATA_DIR  := tests/testdata
SHELL           := /bin/bash
MSRV          := 1.85

# Pinned versions for Rust 1.85.0
TARPAULIN_VERSION=0.32.8
NEXTEST_VERSION=0.9.100
AUDIT_VERSION=0.21.2
CAREFUL_VERSION=0.4.8
DENY_VERSION=0.16.4

# Find the latest built Python wheel file
WHEEL_FILE := $(shell ls $(PYGRAPHINA_DIR)/$(WHEEL_DIR)/pygraphina-*.whl 2>/dev/null | head -n 1)

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show the help message for each target
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' Makefile | \
	   awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

########################################################################################
## Rust targets
########################################################################################

.PHONY: audit
audit: ## Run security audit on Rust dependencies
	@echo "Running security audit..."
	@cargo audit

.PHONY: bench
bench: ## Run benchmarks
	@echo "Running benchmarks..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo bench --features all

.PHONY: bench-graphina
bench-graphina: ## Run the graphina vs rustworkx-core comparison harness
	@echo "Running graphina vs rustworkx-core comparison..."
	@cd benchmarks/graphina && cargo run --release

# Undirected real-world datasets used by the dataset comparison targets. The large
# directed graphs (stanford_web_graph, dblp_citation_network) are left out by default;
# pass one explicitly via RUSTWORKX_COMPARE_DATASET / PYGRAPHINA_COMPARE_DATASET to run it.
COMPARE_DATASETS := wikipedia_chameleon wikipedia_squirrel wikipedia_crocodile facebook_page_page

.PHONY: bench-graphina-datasets
bench-graphina-datasets: ## Run the graphina vs rustworkx-core comparison on the real-world datasets (run `make testdata` first)
	@echo "Running graphina vs rustworkx-core comparison on real-world datasets..."
	@for ds in $(COMPARE_DATASETS); do \
		echo ""; echo "########## dataset: $$ds ##########"; \
		(cd benchmarks/graphina && RUSTWORKX_COMPARE_DATASET=$(CURDIR)/$(TEST_DATA_DIR)/graphina-graphs/$$ds.txt cargo run --release) || exit 1; \
	done

.PHONY: bench-pygraphina
bench-pygraphina: develop-py ## Run the PyGraphina vs rustworkx comparison harness
	@echo "Running PyGraphina vs rustworkx comparison..."
	@uv run --with rustworkx python benchmarks/pygraphina/compare.py

.PHONY: bench-pygraphina-datasets
bench-pygraphina-datasets: develop-py ## Run the PyGraphina vs rustworkx comparison on the real-world datasets (run `make testdata` first)
	@echo "Running PyGraphina vs rustworkx comparison on real-world datasets..."
	@for ds in $(COMPARE_DATASETS); do \
		echo ""; echo "########## dataset: $$ds ##########"; \
		PYGRAPHINA_COMPARE_DATASET=$(CURDIR)/$(TEST_DATA_DIR)/graphina-graphs/$$ds.txt uv run --with rustworkx python benchmarks/pygraphina/compare.py || exit 1; \
	done

.PHONY: build
build: format ## Build the binary for the current platform
	@echo "Building the project..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo build --release

.PHONY: careful
careful: ## Run tests under cargo-careful (detects undefined behavior and unsafe misuse)
	@echo "Running tests under cargo-careful..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo careful test --features all

.PHONY: check-module-deps
check-module-deps: ## Check that top-level modules only depend on core (not on each other)
	@echo "Checking module dependencies..."
	@ERROR=0; \
	TOP_MODULES="approximation centrality community links metrics mst parallel subgraphs traversal"; \
	for module in $$TOP_MODULES; do \
		if [ -d "src/$$module" ]; then \
			for other_module in $$TOP_MODULES; do \
				if [ "$$module" != "$$other_module" ]; then \
					VIOLATIONS=$$(grep -r "use crate::$$other_module" src/$$module/ 2>/dev/null || true); \
					if [ -n "$$VIOLATIONS" ]; then \
						echo "ERROR: Module '$$module' has forbidden dependency on '$$other_module':"; \
						echo "$$VIOLATIONS" | sed 's/^/  /'; \
						ERROR=1; \
					fi; \
				fi; \
			done; \
		fi; \
	done; \
	if [ $$ERROR -eq 0 ]; then \
		echo "All module dependencies are valid - only 'core' is used"; \
	else \
		echo "Module dependency violations found!"; \
		echo ""; \
		echo "Rule: Top-level modules can only depend on 'core', not on each other."; \
		echo "Top-level modules: $$TOP_MODULES"; \
		exit 1; \
	fi

.PHONY: clean
clean: ## Remove generated and temporary files
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf $(WHEEL_DIR) dist/ $(PYGRAPHINA_DIR)/$(WHEEL_DIR)
	@rm -f $(PYGRAPHINA_DIR)/*.so

.PHONY: coverage
coverage: format doctest ## Generate test coverage report (excludes the pygraphina cdylib crate)
	@echo "Generating test coverage report..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo tarpaulin --workspace --exclude pygraphina --features all --out Xml --out Html

.PHONY: deny
deny: ## Check dependencies for advisories, license compliance, and duplicates
	@echo "Running cargo-deny..."
	@cargo deny check

.PHONY: docs
docs: format ## Generate the documentation
	@echo "Generating documentation..."
	@cargo doc --no-deps --document-private-items

.PHONY: doctest
doctest: ## Run documentation tests (Rust code examples in doc comments)
	@echo "Running documentation tests..."
	@cargo test --doc --features all

.PHONY: fix-lint
fix-lint: ## Fix the linter warnings
	@echo "Fixing linter warnings..."
	@cargo clippy --fix --allow-dirty --all-targets --workspace --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used

.PHONY: format
format: ## Format Rust files
	@echo "Formatting Rust files..."
	@cargo fmt

.PHONY: format-check
format-check: ## Check Rust formatting without modifying files (for CI)
	@echo "Checking Rust formatting..."
	@cargo fmt --all --check

.PHONY: install-deps
install-deps: install-snap ## Install development dependencies
	@echo "Installing development dependencies..."
	@rustup component add rustfmt clippy
	# Install each tool with a specific, pinned version
	@cargo install --locked cargo-tarpaulin --version ${TARPAULIN_VERSION}
	@cargo install --locked cargo-nextest --version ${NEXTEST_VERSION}
	@cargo install --locked cargo-audit --version ${AUDIT_VERSION}
	@cargo install --locked cargo-careful --version ${CAREFUL_VERSION}
	@cargo install --locked cargo-deny --version ${DENY_VERSION}
	@sudo apt-get install python3-pip libfontconfig1-dev
	@pip install $(PY_DEP_MNGR)

.PHONY: install-msrv
install-msrv: ## Install the minimum supported Rust version (MSRV) for development
	@echo "Installing the minimum supported Rust version..."
	@rustup toolchain install $(MSRV)
	@rustup default $(MSRV)

.PHONY: install-snap
install-snap: ## Install dependencies using Snapcraft
	@echo "Installing snap dependencies..."
	@sudo apt-get update && sudo apt-get install -y snapd
	@sudo snap refresh
	@sudo snap install rustup --classic

.PHONY: lint
lint: format ## Run linters on Rust files
	@echo "Linting Rust files..."
	@# graphina production code (all features): ban unwrap/expect as well as warnings.
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo clippy --features all -- -D warnings -D clippy::unwrap_used -D clippy::expect_used
	@# graphina all targets (tests, benches, examples): warnings only, since unwrap/expect are allowed in tests.
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo clippy --features all --all-targets -- -D warnings
	@# pygraphina production code: same unwrap/expect ban.
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo clippy -p pygraphina --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used
	@# pygraphina all targets: warnings only.
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo clippy -p pygraphina --all-features --all-targets -- -D warnings

.PHONY: nextest
nextest: ## Run tests using nextest
	@echo "Running tests using nextest..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) RUST_BACKTRACE=$(RUST_BACKTRACE) cargo nextest run --features all

.PHONY: oracle-fixtures
oracle-fixtures: ## Regenerate the NetworkX oracle corpora (for the oracle tests)
	@echo "Generating NetworkX oracle fixtures..."
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_fixtures.py tests/oracle/networkx_oracle.json
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_directed_fixtures.py tests/oracle/networkx_directed_oracle.json
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_directed_centrality_fixtures.py tests/oracle/networkx_directed_centrality_oracle.json
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_centrality_fixtures.py tests/oracle/networkx_centrality_oracle.json
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_spectral_fixtures.py tests/oracle/networkx_spectral_oracle.json
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_links_fixtures.py tests/oracle/networkx_links_oracle.json
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_metrics_fixtures.py tests/oracle/networkx_metrics_oracle.json
	@$(PY_DEP_MNGR) run python scripts/gen_oracle_community_fixtures.py tests/oracle/networkx_community_oracle.json

.PHONY: publish
publish: ## Publish the package to crates.io (requires CARGO_REGISTRY_TOKEN to be set)
	@echo "Publishing package to Cargo registry..."
	@cargo publish --token $(CARGO_REGISTRY_TOKEN)

.PHONY: run-examples
run-examples: ## Run all the scripts in the examples directory one by one
	@echo "Running all example scripts..."
	@for example in examples/*.rs; do \
	   example_name=$$(basename $$example .rs); \
	   echo "Running example: $$example_name"; \
	   cargo run --features all --example $$example_name; \
	done

.PHONY: test
test: format doctest ## Run the tests
	@echo "Running tests..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) RUST_LOG=debug RUST_BACKTRACE=$(RUST_BACKTRACE) cargo test --features all --all-targets \
	--workspace -- --nocapture

.PHONY: testdata
testdata: ## Download the datasets used in tests
	@echo "Downloading test data..."
	@$(SHELL) $(TEST_DATA_DIR)/download_datasets.sh $(TEST_DATA_DIR)

########################################################################################
## Python targets
########################################################################################

.PHONY: develop-py
develop-py: ## Build and install PyGraphina in the current Python environment
	@echo "Building and installing PyGraphina..."
	# Note: Maturin does not work when CONDA_PREFIX and VIRTUAL_ENV are both set
	@(cd $(PYGRAPHINA_DIR) && unset CONDA_PREFIX && maturin develop)

.PHONY: docs-build
docs-build: ## Generate Graphina MkDocs documentation
	@echo "Building Graphina MkDocs..."
	@uv run mkdocs build

.PHONY: docs-py
docs-py: develop-py ## Generate PyGraphina MkDocs documentation
	@echo "Generating MkDocs documentation..."
	@$(PY_DEP_MNGR) run mkdocs build --config-file pygraphina/mkdocs.yml

.PHONY: docs-serve
docs-serve: ## Serve Graphina MkDocs locally
	@echo "Serving Graphina MkDocs..."
	@uv run mkdocs serve

.PHONY: docs-serve-py
docs-serve-py: develop-py ## Serve PyGraphina MkDocs documentation locally
	@echo "Serving MkDocs documentation locally..."
	@$(PY_DEP_MNGR) run mkdocs serve --config-file pygraphina/mkdocs.yml

.PHONY: generate-ci
generate-ci: ## Generate CI configuration files (GitHub Actions workflow)
	@echo "Generating CI configuration files..."
	@(cd $(PYGRAPHINA_DIR) && maturin generate-ci --zig --pytest --platform all -o ../.github/workflows/ci.yml github)

.PHONY: publish-py
publish-py: wheel-manylinux ## Publish the PyGraphina wheel to PyPI (requires PYPI_TOKEN to be set)
	@echo "Publishing PyGraphina to PyPI..."
	@if [ -z "$(WHEEL_FILE)" ]; then \
	   echo "Error: No wheel file found. Please run 'make wheel' first."; \
	   exit 1; \
	fi
	@echo "Found wheel file: $(WHEEL_FILE)"
	@twine upload -u __token__ -p $(PYPI_TOKEN) $(WHEEL_FILE)

.PHONY: rundoc
rundoc: develop-py ## Test all code examples in PyGraphina documentation using rundoc
	@echo "Testing documentation code examples..."
	@failed=0; \
	for f in $(PYGRAPHINA_DIR)/docs/examples/*.md; do \
		echo "=== Testing $$(basename $$f) ==="; \
		if echo | rundoc run "$$f" 2>&1 | grep -q "Failed"; then \
			echo "FAILED: $$f"; \
			failed=$$((failed + 1)); \
		else \
			echo "PASSED: $$f"; \
		fi; \
	done; \
	if [ $$failed -gt 0 ]; then \
		echo "$$failed file(s) had failures"; \
		exit 1; \
	else \
		echo "All documentation examples passed!"; \
	fi

.PHONY: test-py
test-py: develop-py ## Run Python tests
	@echo "Running Python tests..."
	@$(PY_DEP_MNGR) run pytest

.PHONY: wheel
wheel: ## Build the wheel file for PyGraphina
	@echo "Building the PyGraphina wheel..."
	@(cd $(PYGRAPHINA_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check)

.PHONY: wheel-manylinux
wheel-manylinux: ## Build the manylinux wheel file for PyGraphina (using Zig)
	@echo "Building the manylinux PyGraphina wheel..."
	@(cd $(PYGRAPHINA_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check --zig)

########################################################################################
## Additional targets
########################################################################################

.PHONY: setup-hooks
setup-hooks: ## Install Git hooks (pre-commit and pre-push)
	@echo "Installing Git hooks..."
	@pre-commit install --hook-type pre-commit
	@pre-commit install --hook-type pre-push
	@pre-commit install-hooks

.PHONY: test-hooks
test-hooks: ## Test Git hooks on all files
	@echo "Testing Git hooks..."
	@pre-commit run --all-files
