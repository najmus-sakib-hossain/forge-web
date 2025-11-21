# DX Forge

**Production-ready VCS and orchestration engine for DX tools ecosystem**

[![Crates.io](https://img.shields.io/crates/v/dx-forge.svg)](https://crates.io/crates/dx-forge)
[![Documentation](https://docs.rs/dx-forge/badge.svg)](https://docs.rs/dx-forge)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

Forge is the orchestration backbone for the DX tools ecosystem, providing Git-like version control, intelligent file watching, traffic branch safety systems, and zero-bloat component injection for modern development workflows.

## Features

- 🎼 **Tool Orchestration** - Priority-based execution with dependency resolution
- 📡 **Dual-Watcher Architecture** - LSP + File System monitoring
- 🌳 **Traffic Branch System** - Green/Yellow/Red merge safety classification
- 📦 **Version Control** - Git-like snapshots, branching, and merging
- 🔧 **Component Injection** - Zero node_modules bloat with R2 caching
- 🔄 **Lifecycle Management** - Start/stop control with event notifications
- 📊 **Code Generation Tracking** - Track and manage generated files
- ⚡ **Performance** - Parallel execution, content-addressable storage
- 🛡️ **Type Safety** - Full Rust type safety and error handling

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dx-forge = "0.0.2"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

### Basic Usage

```rust
use dx_forge::{Forge, DxTool, ExecutionContext, ToolOutput};
use anyhow::Result;

// Define your tool
struct MyTool;

impl DxTool for MyTool {
    fn name(&self) -> &str { "my-tool" }
    fn version(&self) -> &str { "1.0.0" }
    fn priority(&self) -> u32 { 50 }
    
    fn execute(&mut self, ctx: &ExecutionContext) -> Result<ToolOutput> {
        println!("Running my tool in: {:?}", ctx.repo_root);
        Ok(ToolOutput::success())
    }
}

fn main() -> Result<()> {
    // Initialize Forge
    let forge = Forge::new(".")?;
    println!("Forge initialized at: {:?}", forge.project_root());
    
    // Use orchestrator for tool execution
    use dx_forge::Orchestrator;
    let mut orch = Orchestrator::new(".")?;
    orch.register_tool(Box::new(MyTool))?;
    let results = orch.execute_all()?;
    
    println!("Executed {} tools successfully", results.len());
    Ok(())
}
```

## Documentation

- [API Reference](docs/API_REFERENCE.md) - Complete API documentation
- [Architecture](docs/Architecture) - System design and architecture
- [How to Run Forge](docs/HOW_TO_RUN_FORGE.md) - Setup and running guide
- [DX Tools Integration](docs/DX_TOOLS_INTEGRATION_GUIDE.md) - Integration guide
- [Troubleshooting](docs/troubleshooting.md) - Common issues and solutions

## Examples

See the `examples/` directory for complete working examples:

- [`simple.rs`](examples/simple.rs) - Basic tool registration
- [`traffic_branch_and_lsp.rs`](examples/traffic_branch_and_lsp.rs) - Traffic branch analysis
- [`complete_dx_workflow.rs`](examples/complete_dx_workflow.rs) - Full workflow

Run an example:

```bash
cargo run --example simple
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
