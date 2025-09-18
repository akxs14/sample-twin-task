# --- Rust build & execution helpers ---

BINARY := tiny-agent-graph
FLOW_YAML := config/catalog_check.yml
DB_URL := sqlite://./data/tasks.db

# Default help target — prints all available commands
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make run         - Run the flow defined in $(FLOW_YAML)"
	@echo "  make build       - Build the Rust project"
	@echo "  make check       - Type-check the project (faster than build)"
	@echo "  make fmt         - Format all source files using rustfmt"
	@echo "  make clean       - Remove build artifacts"
	@echo "  make migrate     - Run database migrations for SQLite ($(DB_URL))"
	@echo "  make scheduler   - Start the DAG scheduler + run a flow in 30s"
	@echo "  make test        - Run all unit and integration tests"

# Run the default YAML flow file
.PHONY: run
run:
	cargo run -- run-flow $(FLOW_YAML)

# Compile the project
.PHONY: build
build:
	cargo build

# Type-check only (faster than full build)
.PHONY: check
check:
	cargo check

# Format all Rust code
.PHONY: fmt
fmt:
	cargo fmt

# Remove target/ build output
.PHONY: clean
clean:
	cargo clean

# Run sqlx database migrations (creates ./data/tasks.db)
.PHONY: migrate
migrate:
	mkdir -p data
	sqlx migrate run --database-url $(DB_URL)

# Run all tests (unit + integration)
.PHONY: test
test:
	cargo test --all -- --nocapture
