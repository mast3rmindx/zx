# KnightDAG

A Rust implementation of a directed acyclic graph (DAG) system with web-based visualization.

## Features

- Core DAG implementation in Rust
- PostgreSQL database for persistence
- Web-based visualization using Yew and WebAssembly
- Real-time graph updates and tracking
- Interactive visualization with plotters

## Prerequisites

- Rust 1.70+ and Cargo
- PostgreSQL 13+
- Node.js 16+ (for development tools)
- wasm-pack (install via `cargo install wasm-pack`)

## Project Structure

```
knightdag/
├── src/
│   ├── main.rs           # Backend server entry point
│   ├── lib.rs            # Shared library code
│   ├── dag/              # Core DAG implementation
│   ├── db/               # Database interactions
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
   # Create PostgreSQL database
   createdb knightdag
   # Run migrations
   cargo sqlx migrate run
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

## License

MIT License