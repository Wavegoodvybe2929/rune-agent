use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphStatus {
    Pending,
    Success,
}

/// Represents a step in the execution graph.
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub name: String,
    pub status: GraphStatus,
}

impl GraphNode {
    pub fn display(&self) -> String {
        match self.status {
            GraphStatus::Pending => format!("🔮 {} [Pending]", self.name.bold()),
            GraphStatus::Success => format!("✅ {} [OK]", self.name.green()),
        }
    }
}

/// Simple Graph structure for linear or branching flows.
#[derive(Debug, Clone)]
pub struct ExecutionGraph {
    pub nodes: Vec<GraphNode>,
    pub flows: Vec<(String, String)>, // (from_node, action)
}

impl ExecutionGraph {
    pub fn new() -> Self {
        ExecutionGraph {
            nodes: Vec::new(),
            flows: Vec::new(),
        }
    }

    pub fn add_node(&mut self, name: String, status: GraphStatus) {
        self.nodes.push(GraphNode { name, status });
    }

    pub fn add_flow(&mut self, from: &str, action: &str) {
        self.flows.push((from.to_string(), action.to_string()));
    }

    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("🔮 Execution Graph: {}", "Rune Flow".bold()));
        lines.push("─".repeat(40));

        // Display nodes
        for node in &self.nodes {
            lines.push(node.display());
        }
        
        // Display flows
        if !self.flows.is_empty() {
            lines.push("\n🔗 Flow Connections:".to_string());
            lines.push("─".repeat(40));
            for (from, action) in &self.flows {
                lines.push(format!("  {} ─> {}", from, action.yellow()));
            }
        } else {
            lines.push("\n🔗 No flows defined.".to_string());
        }

        lines.join("\n")
    }
}
