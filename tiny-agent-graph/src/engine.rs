#![allow(dead_code)] // We build incrementally — not every field is wired up yet

use crate::flow::{Flow, StepGraph};
use petgraph::algo::toposort;
use rand::{thread_rng, Rng};
use tracing::{info, warn};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Summary of a completed DAG run (used for reporting or persistence)
#[derive(Debug)]
pub struct RunHistory {
    pub run_id: String,
    pub flow_id: String,
    pub status: RunStatus,
    pub step_results: HashMap<String, StepResult>,
}

/// Final result of the DAG execution
#[derive(Debug)]
pub enum RunStatus {
    Success,
    Failed(String), // includes a reason (e.g. “step X failed” or “blocked”)
}

/// Outcome for a single step (used for audit or export)
#[derive(Debug)]
pub struct StepResult {
    pub status: StepStatus,
    pub output: Option<String>,
}

/// Execution status of an individual step
#[derive(Debug)]
pub enum StepStatus {
    Success,
    Failed(String), // failure reason (e.g. timeout, bad input, dependency block)
}

/// Entrypoint: executes a single flow's DAG from top to bottom
///
/// Accepts:
/// - `flow`: the parsed struct from YAML
/// - `graph`: a petgraph DAG with steps as nodes, edges as dependencies
///
/// Returns:
/// - A `RunHistory` with status + step-by-step outcomes
///
/// Notes:
/// - This is a simulation (uses delay + fake handler)
/// - Dependencies are enforced: steps don't run unless all deps succeeded
/// - Real step execution (with retries, idempotency, etc.) would hook in here
pub async fn run_flow(flow: &Flow, graph: StepGraph) -> anyhow::Result<RunHistory> {
    let run_id = uuid::Uuid::new_v4().to_string();
    info!("🚀 Starting run {run_id} for flow '{}'", flow.id);

    // Stores the result for each step as we go
    let mut results: HashMap<String, StepResult> = HashMap::new();

    // Get steps in topological order (dependencies come before dependents)
    let sorted = toposort(&graph, None)
        .map_err(|cycle| anyhow::anyhow!(
            "Cycle detected at step {:?}",
            graph[cycle.node_id()].step.id
        ))?;

    // Execute each step in topological order
    for node_idx in sorted {
        let node = &graph[node_idx];
        let step = &node.step;

        // Enforce dependency rules — don’t run if any parent failed
        let mut all_deps_ok = true;
        for dep_id in &step.depends_on {
            if let Some(dep_result) = results.get(dep_id) {
                if !matches!(dep_result.status, StepStatus::Success) {
                    warn!("⛔ Step '{}' blocked by failed dependency '{}'", step.id, dep_id);
                    all_deps_ok = false;
                }
            } else {
                // This should never happen if DAG is valid
                warn!("⚠️ Missing result for dependency '{}'", dep_id);
                all_deps_ok = false;
            }
        }

        if !all_deps_ok {
            // Mark step as blocked
            results.insert(
                step.id.clone(),
                StepResult {
                    status: StepStatus::Failed("Blocked by failed dependencies".into()),
                    output: None,
                },
            );
            continue;
        }

        // --- Run the actual step (simulated for now) ---
        info!("▶️ Running step '{}': {}", step.id, step.kind);

        match simulate_step_execution(&step.id, &step.kind).await {
            Ok(output) => {
                info!("✅ Step '{}' succeeded", step.id);
                results.insert(
                    step.id.clone(),
                    StepResult {
                        status: StepStatus::Success,
                        output: Some(output),
                    },
                );
            }
            Err(err) => {
                warn!("❌ Step '{}' failed: {err}", step.id);
                results.insert(
                    step.id.clone(),
                    StepResult {
                        status: StepStatus::Failed(err),
                        output: None,
                    },
                );
            }
        }
    }

    // Determine if the flow completed fully or partially failed
    let has_failures = results.values().any(|r| matches!(r.status, StepStatus::Failed(_)));

    let status = if has_failures {
        RunStatus::Failed("At least one step failed".into())
    } else {
        RunStatus::Success
    };

    Ok(RunHistory {
        run_id,
        flow_id: flow.id.clone(),
        status,
        step_results: results,
    })
}

/// Simulates executing a step by sleeping + returning fake output
///
/// In real usage, this is where:
/// - You call external APIs
/// - Access secrets
/// - Enforce retries / backoff
/// - Log to persistent run history
///
/// This is a placeholder to show how the engine behaves.
async fn simulate_step_execution(id: &str, kind: &str) -> Result<String, String> {
    let delay_ms = thread_rng().gen_range(100..300); // Simulate random latency
    sleep(Duration::from_millis(delay_ms)).await;

    // You can trigger a forced failure by setting kind = "fail_test" in YAML
    if kind == "fail_test" {
        Err("Simulated failure".into())
    } else {
        Ok(format!("Simulated output of '{}'", id))
    }
}
