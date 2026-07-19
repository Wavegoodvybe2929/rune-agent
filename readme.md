# rune-agent

A Rust-based agent engine that parses and executes Runescript — a concise, flow-based **DSL** for automating workflows in React and Tauri development.

## Overview

rune-agent is a minimal, fast Rust agent that:

- Parses Runescript files (a simple declarative DSL)
- Visualizes the execution graph
- Executes shell commands (`RUN(...)`), with optional `OK`/`ERR` branching

Designed as a daily-driver tool for developers building React/Tauri web apps. The AI translates high-level intent (**HLD**) into Runescript, and this Rust binary executes it.

## Runescript Syntax

Runescript is a simple, flow-based DSL:

```
Rune [NodeName][OptionalMetadata] -> [Action1] -> [Action2] -> OK{...} | ERR{...}
```

### Elements

| Element | Example | Meaning |
|---|---|---|
| `Rune` | `Rune BuildWatch[...]` | Declares a node |
| Node Name | `BuildWatch[WATCH src/]` | Unique identifier, may include bracketed metadata |
| `->` | `-> RUN(cargo build)` | Flow arrow between node and action |
| `RUN(...)` | `RUN(cargo build)` | Execute a shell command |
| `OK{...}` | `OK{LOG(Success)}` | Branch on success |
| `ERR{...}` | `ERR{LOG(Failed)}` | Branch on failure |
| `\|` | `OK{...} \| ERR{...}` | Conditional branch separator |

### Example (`demo.runes`)

```
Rune BuildWatch[WATCH src/]
-> RUN(cargo build)
-> OK{LOG(Build Success)} | ERR{LOG(Build Failed)}
```

## Quick Start

### Prerequisites

- Rust (via rustup)
- Git

### Build & Run

```bash
# Clone (or check out)
git clone <repo-url>
cd rune-agent

# Build
cargo build

# Run with default demo
cargo run

# Run with custom runescript
cargo run path/to/your.runes
```

### Output Example

```
▶ Executing: cargo build
    ✅ cargo build succeeded
    📤 Stdout:
🔮 Execution Graph: Rune Flow
────────────────────────────────────────
🔮 BuildWatch[WATCH src/] [Pending]
✅ Flow End [OK]

🔗 Flow Connections:
────────────────────────────────────────
    BuildWatch[WATCH src/] ─> cargo build
    BuildWatch[WATCH src/] ─> OK{LOG(Build Success)} | ERR{LOG(Build Failed)}
```

## Project Structure

```
rune-agent/
├── src/
│   ├── main.rs       # Entry point, CLI parsing
│   ├── runner.rs     # Parses Runescript, builds graph, executes
│   ├── graph.rs      # Graph model + rendering
│   └── utils.rs      # Shell command execution
├── Cargo.toml
└── demo.runes         # Example runescript
```

## Feature Status

| Feature | Status |
|---|---|
| Runescript parser | ✅ Working |
| Graph visualization | ✅ Working |
| Command execution | ✅ Working |
| OK/ERR branching | 🟡 Partial (stubbed) |
| DSL extensions (WAIT, IF, LOOP) | ⬜ Planned |

## Development

```bash
# Watch changes
cargo watch -x run

# Test with different runescript files
cargo run my-workflow.runes
```

## License

MIT
