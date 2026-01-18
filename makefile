# -----------------------------------------
# 26-in-26 Project Makefile Template
# -----------------------------------------

# Default target
.PHONY: all
all: help

# -----------------------------------------
# Help
# -----------------------------------------
.PHONY: help
help:
	@echo "26-in-26 Project Makefile"
	@echo
	@echo "Available commands:"
	@echo "  make run        # Run the project"
	@echo "  make build      # Build / compile the project"
	@echo "  make test       # Run tests"
	@echo "  make clean      # Clean build artifacts"
	@echo "  make poc        # Run POC experiments"
	@echo "  make docs       # Generate / open documentation"

# -----------------------------------------
# Run / Build
# -----------------------------------------
.PHONY: run
run:
	@echo "Running project..."
	@cargo run

.PHONY: build
build:
	@echo "Building project..."
	@cargo build --release

# -----------------------------------------
# Tests
# -----------------------------------------
.PHONY: test
test:
	@echo "Running tests..."
	@cargo test

# -----------------------------------------
# POC / Experiments
# -----------------------------------------
.PHONY: poc
poc:
	@echo "Running POC / experiments..."
	@echo "Customize this target per project"

# -----------------------------------------
# Clean
# -----------------------------------------
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
