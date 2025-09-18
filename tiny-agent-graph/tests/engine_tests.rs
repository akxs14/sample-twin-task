use tiny_agent_graph::engine::{run_flow, StepStatus, RunStatus};
use tiny_agent_graph::flow::{Flow, Step, StepNode, StepGraph};

/// Helper: build a simple flow + graph manually
fn build_test_flow(steps: Vec<Step>, edges: Vec<(usize, usize)>) -> (Flow, StepGraph) {
    let flow = Flow {
        id: "test-flow".to_string(),
        description: Some("Test flow".into()),
        nodes: steps.clone(),
    };

    let mut graph = StepGraph::new();
    let mut indices = Vec::new();

    for step in steps {
        let idx = graph.add_node(StepNode { step });
        indices.push(idx);
    }

    for (from, to) in edges {
        graph.add_edge(indices[from], indices[to], ());
    }

    (flow, graph)
}

#[tokio::test]
async fn test_successful_linear_flow() {
    let steps = vec![
        Step {
            id: "a".into(),
            kind: "noop".into(),
            ..Default::default()
        },
        Step {
            id: "b".into(),
            kind: "noop".into(),
            depends_on: vec!["a".into()],
            ..Default::default()
        },
        Step {
            id: "c".into(),
            kind: "noop".into(),
            depends_on: vec!["b".into()],
            ..Default::default()
        },
    ];

    let (flow, graph) = build_test_flow(steps, vec![(0, 1), (1, 2)]);

    let result = run_flow(&flow, graph).await.unwrap();
    assert!(matches!(result.status, RunStatus::Success));
    assert_eq!(result.step_results.len(), 3);

    for status in result.step_results.values().map(|r| &r.status) {
        assert!(matches!(status, StepStatus::Success));
    }
}

#[tokio::test]
async fn test_failure_propagation() {
    let steps = vec![
        Step {
            id: "a".into(),
            kind: "fail_test".into(), // Simulated failure
            ..Default::default()
        },
        Step {
            id: "b".into(),
            kind: "noop".into(),
            depends_on: vec!["a".into()],
            ..Default::default()
        },
        Step {
            id: "c".into(),
            kind: "noop".into(),
            depends_on: vec!["b".into()],
            ..Default::default()
        },
    ];

    let (flow, graph) = build_test_flow(steps, vec![(0, 1), (1, 2)]);

    let result = run_flow(&flow, graph).await.unwrap();
    assert!(matches!(result.status, RunStatus::Failed(_)));

    let step_a = result.step_results.get("a").unwrap();
    assert!(matches!(step_a.status, StepStatus::Failed(_)));

    let step_b = result.step_results.get("b").unwrap();
    assert!(matches!(step_b.status, StepStatus::Failed(_)));

    let step_c = result.step_results.get("c").unwrap();
    assert!(matches!(step_c.status, StepStatus::Failed(_)));
}

#[tokio::test]
async fn test_parallel_branching_success() {
    let steps = vec![
        Step {
            id: "a".into(),
            kind: "noop".into(),
            ..Default::default()
        },
        Step {
            id: "b1".into(),
            kind: "noop".into(),
            depends_on: vec!["a".into()],
            ..Default::default()
        },
        Step {
            id: "b2".into(),
            kind: "noop".into(),
            depends_on: vec!["a".into()],
            ..Default::default()
        },
        Step {
            id: "c".into(),
            kind: "noop".into(),
            depends_on: vec!["b1".into(), "b2".into()],
            ..Default::default()
        },
    ];

    let (flow, graph) = build_test_flow(steps, vec![(0, 1), (0, 2), (1, 3), (2, 3)]);

    let result = run_flow(&flow, graph).await.unwrap();
    assert!(matches!(result.status, RunStatus::Success));
}
