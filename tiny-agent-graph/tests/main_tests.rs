use assert_cmd::Command;
use predicates::str::contains;
use tempfile::NamedTempFile;
use std::io::Write;

/// Helper: write a temporary flow YAML file
fn write_flow(contents: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    write!(file, "{}", contents).expect("Failed to write YAML");
    file
}

#[tokio::test]
async fn test_main_executes_valid_flow() {
    let yaml = r#"
id: test-flow
nodes:
  - id: a
    kind: noop
"#;
    let file = write_flow(yaml);

    Command::cargo_bin("tiny-agent-graph")
        .unwrap()
        .arg("run-flow")
        .arg(file.path())
        .assert()
        .success()
        .stdout(contains("✅ Loaded flow"))
        .stdout(contains("✅ a →"))
        .stdout(contains("🎯 Final status: Success"));
}

#[tokio::test]
async fn test_main_handles_cycle_error() {
    let yaml = r#"
id: cyclic-flow
nodes:
  - id: a
    kind: noop
    depends_on: [b]
  - id: b
    kind: noop
    depends_on: [a]
"#;
    let file = write_flow(yaml);

    Command::cargo_bin("tiny-agent-graph")
        .unwrap()
        .arg("run-flow")
        .arg(file.path())
        .assert()
        .failure()
        .stderr(contains("❌ Failed to load flow"));
}

#[tokio::test]
async fn test_main_handles_missing_file() {
    Command::cargo_bin("tiny-agent-graph")
        .unwrap()
        .arg("run-flow")
        .arg("config/does_not_exist.yml")
        .assert()
        .failure()
        .stderr(contains("❌ Failed to load flow"));
}
