// Top-level module declarations
mod flow;     // Flow parsing and DAG building
mod engine;   // DAG execution engine

// Standard and third-party imports
use std::path::PathBuf;
use tracing::{info, error};
use clap::{Parser, Subcommand};
use flow::load_flow;
use engine::{run_flow, StepStatus};

/// CLI entrypoint using `clap` to define subcommands
#[derive(Parser)]
#[command(name = "Tiny Agent Graph", version, about = "Durable DAG runner for agent workflows")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available subcommands
#[derive(Subcommand)]
enum Commands {
    /// Load and execute a YAML-based flow definition
    RunFlow {
        /// Path to the flow YAML file (e.g. config/catalog_check.yml)
        config: PathBuf,
    },
}

/// Async entrypoint with Tokio runtime
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up structured logging using the `tracing` crate
    // Logs will go to stderr (important for test output and shell scripts)
    tracing_subscriber::fmt()
        .with_env_filter("tiny_agent_graph=debug")
        .with_writer(std::io::stderr) // ✅ Ensure logs go to stderr
        .init();

    // Parse CLI arguments (e.g. `run-flow config/catalog_check.yml`)
    let cli = Cli::parse();

    match cli.command {
        Commands::RunFlow { config } => {
            info!("📄 Loading flow from {:?}", config);

            match load_flow(&config) {
                Ok((flow, graph)) => {
                    println!("✅ Loaded flow '{}'", flow.id);
                    println!("🔢 Total steps: {}\n", graph.node_count());

                    let result = run_flow(&flow, graph).await?;

                    println!("🎯 Final status: {:?}", result.status);
                    println!("\n📋 Step results:");

                    for (step_id, outcome) in result.step_results.iter() {
                        match &outcome.status {
                            StepStatus::Success => {
                                println!("✅ {} → {}", step_id, outcome.output.as_deref().unwrap_or("✓"));
                            }
                            StepStatus::Failed(err) => {
                                println!("❌ {} → Failed: {}", step_id, err);
                            }
                        }
                    }

                    // Future:
                    // - Export RunHistory to file (JSON/YAML)
                    // - Record to SQLite
                    // - Expose as an API (e.g. via MCP or HTTP)
                }
                Err(err) => {
                    error!("❌ Failed to load flow: {err}");
                    std::process::exit(1); // ❗ exit non-zero for CI/tests
                }
            }
        }
    }

    Ok(())
}
