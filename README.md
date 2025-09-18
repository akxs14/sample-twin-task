# tiny-agent-graph

**A minimal, production-grade DAG engine for executing YAML-defined agent workflows in Rust.**

---

## 🧭 What It Does

`tiny-agent-graph` lets you define structured, multi-step workflows in simple YAML like this:


	id: catalog_sync
	nodes:
	  - id: login
	    kind: http_login

	  - id: fetch_catalog
	    kind: http_get
	    depends_on: [login]

	  - id: validate
	    kind: schema_check
	    depends_on: [fetch_catalog]


It then:

✅ Parses the YAML into a typed Flow

✅ Builds an acyclic execution DAG

✅ Executes each step in topological order

✅ Respects dependencies (depends_on)

✅ Tracks and logs per-step success, failure, and output

✅ Includes full unit and integration test coverage

This is the bare-metal foundation of a resilient agent workflow engine — no bloat, no boilerplate.


🔧 Why It Exists

This repo was built to demonstrate:

⚙️ Executable graph-based agents from YAML

🔁 Safe step orchestration via DAG execution

🧪 Pure, testable design — both CLI and core logic

🧱 Modular structure ready for real-world expansion


📦 Project Structure


tiny-agent-graph/
├── src/
│   ├── main.rs           # CLI entrypoint (run a flow file)
│   ├── lib.rs            # Module exports for testing
│   ├── flow.rs           # Flow parser + DAG builder (petgraph)
│   └── engine.rs         # DAG executor with failure propagation
├── config/
│   └── catalog_check.yml # Sample YAML flow used in Makefile
├── tests/
│   ├── main_tests.rs     # CLI integration tests
│   ├── flow_tests.rs     # YAML and graph parsing tests
│   └── engine_tests.rs   # DAG execution logic tests
├── Makefile              # Dev UX: build, run, test, fmt, help
└── README.md             # You're here


⚙️ How It Works

Parse — flow.rs loads .yml into a strongly typed Flow

Build — constructs a directed acyclic graph (StepGraph) using petgraph

Execute — engine.rs runs each step topologically

Skips or blocks failed steps' dependents

Simulated execution output per step (pluggable)

Log — logs flow and step statuses via tracing

CLI — main.rs provides run-flow subcommand to execute a .yml file


# 🚀 Getting Started

## 1. 📦 Install

	git clone https://github.com/your-username/tiny-agent-graph
	cd tiny-agent-graph
	cargo build

## 2. ▶️ Run a Flow

	make run


Uses the included config/catalog_check.yml.

## 3. 🧪 Run All Tests

	make test

Runs:

Unit tests for DAG + engine

Integration tests for CLI

## 4. 🧼 Format

	make fmt


## 💡 Sample YAML Flow

	id: catalog_check
	nodes:
	  - id: login
	    kind: noop

	  - id: fetch
	    kind: noop
	    depends_on: [login]

	  - id: validate
	    kind: noop
	    depends_on: [fetch]

	  - id: save
	    kind: noop
	    depends_on: [validate]

All steps will be executed in order, respecting the dependencies.


🧠 Design Philosophy

🔁 Determinism — Topological sorting ensures safe, predictable runs

🧱 Modularity — Each step is pluggable (noop, http_get, etc.)

🧪 Testability — Every function is unit or integration test covered

🔩 Extensibility — Retry logic, compensation, scheduling, and real handlers can be added without refactor


# ✨ Possible Improvements
## 🧰 Immediate

- Export RunHistory to JSON or YAML
- Add a --json flag for CLI machine-readable output
- Add real handler types (http_get, shell, db_insert, etc.)

## 💥 Advanced
- Add step-level retries and exponential backoff
- Graph rendering with graphviz (.dot export)
- Web UI to inspect flows and outputs
- Claude / OpenAI plugin support via Model Context Protocol (MCP)

# 🤝 License

MIT — open to use, improve, or fork as needed.

