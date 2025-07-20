# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Sigma is a Rust-based MCP (Model Context Protocol) server for building and simulating Verilog and SystemVerilog files. It provides an interface for EDA (Electronic Design Automation) tools through the MCP protocol.

## Development Commands

```bash
# Check compilation
cargo check

# Build the project
cargo build

# Run the MCP server
cargo run

# Run tests (when implemented)
cargo test

# Generate documentation
cargo doc --open
```

## Project Architecture

The codebase is organized into several key modules:

- **`src/main.rs`** - Entry point for the MCP server
- **`src/tools/`** - MCP tool implementations for Verilog operations
- **`src/verilog/`** - Core Verilog/SystemVerilog functionality
  - `parser.rs` - Verilog file parsing
  - `toolchain.rs` - Tool detection and management (Icarus, Verilator, etc.)
  - `simulator.rs` - Simulation execution interface
- **`src/utils/`** - Utility functions for file operations

## Key Dependencies

- **rmcp** - Rust MCP SDK for protocol implementation
- **tokio** - Async runtime
- **serde/serde_json** - Serialization for MCP messages
- **anyhow** - Error handling

## MCP Tools (Planned)

- `build_verilog` - Compile Verilog/SystemVerilog files
- `run_simulation` - Execute testbenches and simulations
- `list_modules` - Extract module information from Verilog files
- `analyze_design` - Static analysis of designs

## Simulator Support

The project is designed to support multiple simulation tools:
- Icarus Verilog (open source)
- Verilator (open source) 
- ModelSim/QuestaSim (commercial)

## Development Notes

This is an early-stage project. The MCP server structure is in place but individual tools are not yet implemented. When implementing new functionality, follow the existing module structure and use the `rmcp` crate for MCP protocol handling.