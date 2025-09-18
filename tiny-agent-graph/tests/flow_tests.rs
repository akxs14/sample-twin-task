#![allow(dead_code)]

use tiny_agent_graph::flow::load_flow;
use petgraph::algo::is_cyclic_directed;
use tempfile::NamedTempFile;
use std::io::Write;

fn write_yaml(contents: &str) -> NamedTempFile {
    let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
    write!(tmp, "{}", contents).expect("Failed to write YAML");
    tmp
}

#[test]
fn test_loads_valid_linear_flow() {
    let yaml = r#"
id: test-flow
nodes:
  - id: a
    kind: noop
  - id: b
    kind: noop
    depends_on: [a]
  - id: c
    kind: noop
    depends_on: [b]
"#;

    let file = write_yaml(yaml);
    let (flow, graph) = load_flow(file.path()).expect("Failed to load flow");

    assert_eq!(flow.id, "test-flow");
    assert_eq!(flow.nodes.len(), 3);
    assert_eq!(graph.node_count(), 3);
    assert!(!is_cyclic_directed(&graph));
}

#[test]
fn test_loads_branching_flow() {
    let yaml = r#"
id: branch-flow
nodes:
  - id: start
    kind: noop
  - id: a
    kind: noop
    depends_on: [start]
  - id: b
    kind: noop
    depends_on: [start]
  - id: end
    kind: noop
    depends_on: [a, b]
"#;

    let file = write_yaml(yaml);
    let (_flow, graph) = load_flow(file.path()).expect("Failed to load flow");

    assert_eq!(graph.node_count(), 4);
    assert!(!is_cyclic_directed(&graph));
}

#[test]
fn test_detects_cycle() {
    let yaml = r#"
id: bad-flow
nodes:
  - id: a
    kind: noop
    depends_on: [c]
  - id: b
    kind: noop
    depends_on: [a]
  - id: c
    kind: noop
    depends_on: [b]
"#;

    let file = write_yaml(yaml);
    let result = load_flow(file.path());

    assert!(result.is_err());
    let err = result.err().unwrap().to_string();
    assert!(err.contains("cycle"), "Error did not contain 'cycle': {}", err);
}

#[test]
fn test_missing_dependency_warns_but_does_not_crash() {
    let yaml = r#"
id: warn-flow
nodes:
  - id: x
    kind: noop
    depends_on: [nonexistent_step]
"#;

    let file = write_yaml(yaml);
    let result = load_flow(file.path());

    assert!(result.is_ok(), "Missing dep should warn, not error");
    let (_flow, graph) = result.unwrap();
    assert_eq!(graph.node_count(), 1);
}

#[test]
fn test_retry_and_compensation_parsing() {
    let yaml = r#"
id: advanced
nodes:
  - id: step1
    kind: http_post
    retry:
      max_attempts: 5
      backoff_seconds: 10
    compensation:
      kind: http_delete
      config:
        url: "https://example.com/delete"
"#;

    let file = write_yaml(yaml);
    let (flow, _graph) = load_flow(file.path()).expect("Failed to parse flow");

    let step = &flow.nodes[0];

    let retry = step.retry.as_ref().expect("Missing retry");
    assert_eq!(retry.max_attempts, 5);
    assert_eq!(retry.backoff_seconds, 10);

    let comp = step.compensation.as_ref().expect("Missing compensation");
    assert_eq!(comp.kind, "http_delete");
    assert!(comp.config["url"].as_str().unwrap().contains("example.com"));
}
