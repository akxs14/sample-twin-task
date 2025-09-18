# 🧠 tiny-agent-graph – Candidate Assignment

Hello, friend.

This repo contains the skeleton of a tiny workflow engine — one that reads YAML, builds a graph, and executes steps in order. Think of it as a "makefile for agents" — but cleaner..

The provided solution should be submitted as a piece of work you intend to hand over to your future colleagues. It should be on the level of quality you would feel proud about and exhibits your professionalism.

---

## 🎯 Your Mission

Your task is to build a **minimal, testable execution engine** that takes a YAML flow like this:

```yaml
id: my-cool-flow
nodes:
  - id: a
    kind: noop

  - id: b
    kind: noop
    depends_on: [a]

  - id: c
    kind: noop
    depends_on: [b]
```

…and does the following:

🛠️ Parses it into a typed structure

📈 Builds a directed acyclic graph (DAG)

🔄 Executes each step in topological order

🧱 Respects depends_on

✅ Logs success/failure/output of each step

🧪 Is fully testable (unit + integration)

That’s it. No database, no web server, no rocket math. Just clean execution logic, clear outputs, and some nice structure.


⏱️ Time Required

1–2 focused evenings should be enough to complete the core assignment.

You have 7 days from the day of reception to submit your solution.

Feel free to overachieve — but we care more about clarity, reasoning, and testability than about features.


💡 Bonus (optional)

If you’re in the mood to show off:

- Add retry policies (max_attempts, backoff_seconds)

- Add support for compensation steps (sagas-style rollback)

- Add a CLI (cargo run -- run-flow file.yml)

- Export RunHistory to .json or .yaml

- Replace "noop" with a real handler (e.g. shell command or HTTP GET)

*But again*: simple and correct wins over complex and buggy.


✅ What We're Looking For

Clear structure

Working DAG traversal

Step-level isolation and traceability

Tests that are not just there, but useful

Bonus: tasteful comments, naming, and edge-case handling

