# KnightDAG

A Rust implementation of a directed acyclic graph (DAG) system with web-based visualization.

## Features

- Core DAG implementation in Rust
- ParityDB for efficient blockchain-optimized storage
- Web-based visualization using Yew and WebAssembly
- Real-time graph updates and tracking
- Interactive visualization with plotters

## Prerequisites

- Rust 1.70+ and Cargo
- Node.js 16+ (for development tools)
- wasm-pack (install via `cargo install wasm-pack`)

## Project Structure

```
knightdag/
├── src/
│   ├── main.rs           # Backend server entry point
│   ├── lib.rs            # Shared library code
│   ├── dag/              # Core DAG implementation
│   ├── db/              # Database interactions
│   ├── api/              # Web API endpoints
│   └── frontend/         # Yew frontend code
└── frontend/             # Frontend assets and styles
```

## Development Setup

1. Install dependencies:
   ```bash
   cargo install wasm-pack
   ```

2. Set up the database:
   ```bash
   # Create data directory
   mkdir -p data/knightdag
   ```

3. Run the development server:
   ```bash
   cargo run
   ```

4. Build and serve the frontend:
   ```bash
   cd frontend
   trunk serve
   ```

## Environment Variables

- `DATABASE_PATH`: Path to the ParityDB database directory (default: "data/knightdag")
- `PORT`: Port to run the server on (default: 3000)

## License

MIT License