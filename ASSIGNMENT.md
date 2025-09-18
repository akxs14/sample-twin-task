# 🧠 Tiny Agent Graph – A Durable DAG Runner for Agents

Welcome! 👋  
This is a small Rust project that explores how to **run agent workflows as typed, durable graphs**—exactly the kind of thing you’d need when your agents start doing real-world operations like fetching invoices, checking catalogs, or syncing data between systems.

---

## 💡 The Problem

Many agent-based systems (like the ones Twin Labs is building) don’t just run a single function—they execute **multi-step flows**:  
`Login → Fetch → Parse → Verify → Store`

These steps often depend on each other, might need to **retry** if something fails (like a flaky API), and must be **idempotent** so we don’t accidentally upload the same invoice twice. Sometimes we even want to **undo** things if a flow fails halfway—think "saga" patterns or compensation hooks.

We also want to schedule flows in the future:  
“Run this invoice fetch every morning at 04:00, and retry it 3 times if it fails.”

This project tackles both sides of that puzzle:
- A **tiny graph engine** that runs flows described in YAML, persists step state, retries on failure, and supports compensation steps.
- A **durable timer + resumer**, so we can schedule agent tasks with at-least-once guarantees (even across crashes).

---

## 🎯 What This Project Does

- Loads YAML-based flows into a typed **DAG** (via `petgraph`)
- Runs them step-by-step with:
  - **Idempotency keys** to avoid duplicate effects
  - **Retry policies** (with backoff/jitter)
  - **Compensation hooks** for safe rollback
- Stores all runs, step statuses, and outputs in a local **SQLite** database
- Provides a basic **cron-style scheduler**:
  - You can `schedule()` a task for later
  - A background loop wakes it up and runs it
  - Survives restarts and avoids double execution

---

## 🤖 Example Use Case

Let’s say we want to:
1. Log into an API
2. Fetch a list of invoices
3. Parse them
4. Validate that each invoice is from this month
5. Store the data into a database

You describe that as a YAML flow (see `flows/quick_catalog_check.yml`), and this runner will:
- Build a DAG
- Resolve dependencies
- Enforce retries + idempotency
- Record exactly what happened, with timestamps and result logs

You can even schedule this to run every day at 04:00 using `schedule(task, when)`.

---

## 🔧 Why This Exists

Twin Labs mentions **graph-based agents**, **secure automation**, **vertical operators**, and **high-reliability infra**. That means you care about:
- Clear state machines
- Durable scheduling
- Idempotent side effects
- Debuggable run history
- Being able to sleep at night 🛏️

This project shows a minimal—but production-minded—approach to that.

---

## 🧱 Technologies Used

- `petgraph` for the DAG
- `sqlx` (or `rusqlite`) for persistence
- `serde_yaml` to describe flows
- `tracing` for logs and spans
- `tokio` for async task execution
- `ulid` for unique IDs
- MCP (Model Context Protocol) for tool exposure

---

## 🧪 What’s Next

- Try running the demo: `cargo run -- run-flow flows/quick_catalog_check.yml`
- Schedule a task: `cargo run -- schedule flows/quick_catalog_check.yml "2025-09-20T04:00:00Z"`
- View the run history
- Or plug it into **Claude Desktop** via MCP and ask it to "run the quick catalog check flow"

---

## ❤️ Why It Matters

Real-world agents need more than clever prompts.  
They need **durability**, **accountability**, and **the ability to recover gracefully**.

This repo is my take on that.

Hope you enjoy it,  
Angelos
