# Variables
PKG = github.com/habedi/graphina
BINARY_NAME = $(or $(PROJ_BINARY), $(notdir $(PKG)))
BINARY = target/release/$(BINARY_NAME)
PATH := /snap/bin:$(PATH)
DEBUG_GRAPHINA = 1
RUST_LOG = info
RUST_BACKTRACE = full
WHEEL_DIR = dist
PYGRAPHINA_DIR = pygraphina

# Find the built Python wheel file (the latest one)
WHEEL_FILE = $(shell ls $(PYGRAPHINA_DIR)/$(WHEEL_DIR)/pygraphina-*.whl | head -n 1)

# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show the list of available targets with their descriptions
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

########################################################################################
## Rust targets
########################################################################################

.PHONY: format
format: ## Format Rust files
	@echo "Formatting Rust files..."
	@cargo fmt

.PHONY: test
test: format ## Run tests
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
	@echo "Running the $(BINARY) binary..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) ./$(BINARY)

.PHONY: clean
clean: ## Remove generated and temporary files
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf dist/
	@rm -rf $(PYGRAPHINA_DIR)/$(WHEEL_DIR)
	@rm -f $(PYGRAPHINA_DIR)/*.so

.PHONY: install-snap
install-snap: ## Install a few dependencies using Snapcraft
	@echo "Installing the snap package..."
	@sudo apt-get update
	@sudo apt-get install -y snapd
	@sudo snap refresh
	@sudo snap install rustup --classic

.PHONY: install-deps
install-deps: install-snap ## Install development dependencies
	@echo "Installing dependencies..."
	@rustup component add rustfmt clippy
	@cargo install cargo-tarpaulin
	@cargo install cargo-audit

.PHONY: lint
lint: format ## Run linters on Rust files
	@echo "Linting Rust files..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo clippy -- -D warnings

.PHONY: publish
publish: ## Publish the package to crates.io (needs CARGO_REGISTRY_TOKEN to be set)
	@echo "Publishing the package to Cargo registry..."
	@cargo publish --token $(CARGO_REGISTRY_TOKEN)

.PHONY: bench
bench: ## Run benchmarks
	@echo "Running benchmarks..."
	@DEBUG_GRAPHINA=$(DEBUG_GRAPHINA) cargo bench

.PHONY: audit
audit: ## Run security audit on Rust dependencies
	@echo "Running security audit..."
	@cargo audit

########################################################################################
## Python targets
########################################################################################

.PHONY: develop_py
develop_py: ## Build and install PyGraphina in current Python environment
	@echo "Building and installing PyGraphina in current Python environment..."
	@unset CONDA_PREFIX && cd $(PYGRAPHINA_DIR) && maturin develop && cd ..

.PHONY: wheel
wheel: ## Build the wheel file for PyGraphina
	@echo "Make the Python wheel file..."
	@cd $(PYGRAPHINA_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check && cd ..

.PHONY: wheel-manylinux
wheel-manylinux: ## Build the wheel file for PyGraphina (manylinux version using Zig)
	@echo "Make the Python wheel file..."
	@cd $(PYGRAPHINA_DIR) && maturin build --release --out $(WHEEL_DIR) --auditwheel check --zig && cd ..

.PHONY: test_py
test_py: develop_py ## Run Python tests
	@echo "Running Python tests..."
	@poetry run pytest $(PYGRAPHINA_DIR)/tests

.PHONY: publish_py
publish_py: wheel ## Publish the PyGraphina wheel to PyPI (needs PYPI_TOKEN to be set)
	@echo "Publishing the PyGraphina wheel to PyPI..."
	@echo "Found wheel file: $(WHEEL_FILE)"
	@twine upload -u __token__ -p $(PYPI_TOKEN) $(WHEEL_FILE)

.PHONY: generat_ci
generate_ci: ## Generate CI configuration files (GitHub Actions workflow)
	@echo "Generating CI configuration files..."
	@cd $(PYGRAPHINA_DIR) && maturin generate-ci --zig --pytest --platform all \
	-o ../.github/workflows/ci.yml github && cd ..
