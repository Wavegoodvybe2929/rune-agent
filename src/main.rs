mod runner; // Declare runner module
mod graph;  // Declare graph module
mod utils;  // Declare utils module

use runner::Runner;

fn main() {
    // Determine runescript file from args or default
    let runescript = match std::env::args().nth(1) {
        Some(path) => path.clone(),
        None => "demo.runes".to_string(),
    };

    // Load runescript
    let content = match std::fs::read_to_string(&runescript) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error loading runescript: {}", e);
            return;
        }
    };

    // Initialize Runner
    let mut runner = Runner::new();

    // Execute
    match runner.run(&content) {
        Ok(graph_view) => {
            println!("{}", graph_view);
        }
        Err(e) => {
            eprintln!("Error executing runes: {}", e);
        }
    }
}
