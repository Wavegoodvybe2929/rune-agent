use crate::graph::{ExecutionGraph, GraphStatus};
use crate::utils::execute_command;

#[derive(Debug)]
pub struct Runner {
    pub graph: ExecutionGraph,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            graph: ExecutionGraph::new(),
        }
    }

    pub fn run(&mut self, runes: &str) -> Result<String, String> {
        let steps = runes
            .split("Rune ")
            .skip(1)
            .map(|s| s.trim())
            .collect::<Vec<_>>();

        for step_text in steps {
            let parts: Vec<&str> = step_text.split(" -> ").collect();

            if parts.is_empty() {
                continue;
            }

            // Parse node name (preserve brackets)
            let node_name = parts[0].trim();
            self.graph.add_node(node_name.to_string(), GraphStatus::Pending);

            // Process flow relationships
            for part in parts.iter().skip(1) {
                let action = part.trim();

                let flow_action = if action.starts_with("RUN(") {
                    // Extract command: RUN(cargo build) -> cargo build
                    let cmd = action.strip_prefix("RUN(")
                        .unwrap_or(action)
                        .strip_suffix(")")
                        .unwrap_or(action);
                    
                    // EXECUTE the command
                    println!("▶ Executing: {}", cmd);
                    match execute_command(cmd) {
                        Ok(result) => {
                            if result.exit_code == 0 {
                                println!("  ✅ {} succeeded", cmd);
                                println!("  📤 Stdout: {}", result.stdout);
                            } else {
                                println!("  ❌ {} failed (exit code: {})", cmd, result.exit_code);
                                println!("  📤 Stderr: {}", result.stderr);
                            }
                        }
                        Err(e) => {
                            println!("  ❌ Failed to execute {}: {}", cmd, e);
                        }
                    }
                    
                    cmd.to_string()
                } else {
                    action.to_string()
                };

                // Add flow to graph
                self.graph.add_flow(node_name, &flow_action);
            }
        }

        // Add end node
        self.graph.add_node("Flow End".to_string(), GraphStatus::Success);

        Ok(self.graph.render())
    }
}
