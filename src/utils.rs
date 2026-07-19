use std::process::Command;
use std::error::Error;

#[derive(Debug)]
pub struct CommandResult {
    pub exit_code: u32,
    pub stdout: String,
    pub stderr: String,
}

/// Execute a shell command and return the result.
/// Intentionally unused in current run (stub for execution layer).
#[allow(dead_code)]
pub fn execute_command(command: &str) -> Result<CommandResult, Box<dyn Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    Ok(CommandResult {
        exit_code: output.status.code().map_or(1, |code| code as u32),
        stdout,
        stderr,
    })
}

