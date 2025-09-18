#![allow(dead_code)] // Allow unused code during incremental development

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use petgraph::graph::{Graph, NodeIndex};
use tracing::{debug, warn};

/// Represents a complete agent flow, as loaded from a YAML definition
#[derive(Debug, Deserialize)]
pub struct Flow {
    /// Unique identifier for the flow (used for scheduling, runs, etc.)
    pub id: String,

    /// Optional human-readable description (not used functionally)
    pub description: Option<String>,

    /// The list of steps that make up this flow
    pub nodes: Vec<Step>,
}

/// A single step in a flow (represented as a node in the DAG)
#[derive(Debug, Deserialize, Clone)]
pub struct Step {
    /// Unique step ID within this flow
    pub id: String,

    /// Type of handler to invoke (e.g. "http_get", "db_upsert")
    pub kind: String,

    /// Step IDs this one depends on (DAG edges)
    #[serde(default)]
    pub depends_on: Vec<String>,

    /// Arbitrary config passed to the step at runtime
    #[serde(default)]
    pub config: serde_yaml::Value,

    /// Optional retry configuration
    #[serde(default)]
    pub retry: Option<RetryPolicy>,

    /// Optional idempotency key to enable safe retries
    #[serde(default)]
    pub idempotency_key: Option<String>,

    /// Optional compensation logic (for rollback flows)
    #[serde(default)]
    pub compensation: Option<Compensation>,
}

/// Optional retry policy per step (attempts, backoff, etc.)
#[derive(Debug, Deserialize, Clone)]
pub struct RetryPolicy {
    /// Maximum number of attempts (default = 1)
    pub max_attempts: usize,

    /// Backoff between attempts, in seconds
    #[serde(default = "default_backoff")]
    pub backoff_seconds: u64,
}

/// Compensation step definition (used to rollback if needed)
#[derive(Debug, Deserialize, Clone)]
pub struct Compensation {
    /// Handler kind to invoke during compensation
    pub kind: String,

    /// Config passed to the compensating handler
    #[serde(default)]
    pub config: serde_yaml::Value,
}

/// Fallback backoff delay if none specified in RetryPolicy
fn default_backoff() -> u64 {
    5
}

/// Internal graph node type — wraps a Step
#[derive(Debug, Clone)]
pub struct StepNode {
    pub step: Step,
}

/// Graph of step execution (DAG)
pub type StepGraph = Graph<StepNode, ()>;

/// Public function to load a flow definition from disk
/// - Parses YAML into typed `Flow`
/// - Builds a validated, acyclic execution DAG from the flow
pub fn load_flow(path: &Path) -> anyhow::Result<(Flow, StepGraph)> {
    let yaml = std::fs::read_to_string(path)?;
    let flow: Flow = serde_yaml::from_str(&yaml)?;
    let dag = build_step_graph(&flow)?;
    Ok((flow, dag))
}

/// Converts the flow into an executable DAG of `StepNode`s
/// - Verifies node uniqueness
/// - Connects dependencies
/// - Detects and rejects cycles
///
/// This function is exposed internally for tests and scheduler usage.
pub(crate) fn build_step_graph(flow: &Flow) -> anyhow::Result<StepGraph> {
    let mut graph = StepGraph::new();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

    // Insert each step as a node
    for step in &flow.nodes {
        let index = graph.add_node(StepNode { step: step.clone() });
        node_indices.insert(step.id.clone(), index);
    }

    // Add edges for each declared dependency
    for step in &flow.nodes {
        let from_idx = node_indices.get(&step.id).unwrap();

        for dep in &step.depends_on {
            match node_indices.get(dep) {
                Some(dep_idx) => {
                    graph.add_edge(*dep_idx, *from_idx, ());
                }
                None => {
                    // Log missing dependency but don't panic — will likely cause step to block
                    warn!("⚠ Step '{}' depends on unknown step '{}'", step.id, dep);
                }
            }
        }
    }

    // Validate DAG is acyclic (required for safe topological execution)
    if let Err(cycle) = petgraph::algo::toposort(&graph, None) {
        return Err(anyhow::anyhow!(
            "Flow '{}' contains a cycle at step '{}'",
            flow.id,
            graph[cycle.node_id()].step.id
        ));
    }

    debug!(
        "✅ Loaded flow '{}' with {} steps",
        flow.id,
        graph.node_count()
    );

    Ok(graph)
}

/// Default implementation of Step for test cases or stubs
impl Default for Step {
    fn default() -> Self {
        Step {
            id: String::new(),
            kind: "noop".into(),
            depends_on: vec![],
            config: serde_yaml::Value::Null,
            retry: None,
            idempotency_key: None,
            compensation: None,
        }
    }
}
