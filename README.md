# KnightDAG

A Rust implementation of the KnightDAG protocol with web-based visualization.

## Overview

KnightDAG is a parameterless, self-stabilizing directed acyclic graph (DAG) consensus protocol that adapts to network conditions without relying on hardcoded network latency assumptions. This implementation includes:

- Core DAG implementation in Rust
- PostgreSQL database for graph storage
- Web-based visualization using React/TypeScript
- REST API for frontend-backend communication

## Features

- Parameterless consensus protocol
- Self-stabilizing block confirmation times
- Network condition adaptation
- K-cluster based network delay evaluation
- Web visualization of DAG state
- REST API for graph queries and updates

## Prerequisites

- Rust 1.70+ 
- PostgreSQL 15+
- Node.js 18+
- npm 9+

## Project Structure

```
.
├── README.md
├── backend/                 # Rust backend
│   ├── src/                # Source code
│   │   ├── main.rs         # Entry point
│   │   ├── dag/            # DAG implementation
│   │   ├── api/            # REST API
│   │   └── db/             # Database layer
│   ├── Cargo.toml          # Rust dependencies
│   └── migrations/         # Database migrations
├── frontend/               # React frontend  
│   ├── src/               # Source code
│   ├── package.json       # Node dependencies
│   └── tsconfig.json      # TypeScript config
└── docker/                # Docker configuration
```

## Getting Started

1. Install dependencies:
```bash
# Backend
cd backend
cargo build

# Frontend
cd frontend
npm install
```

2. Set up the database:
```bash
cd backend
cargo run --bin setup-db
```

3. Start the backend:
```bash
cd backend
cargo run
```

4. Start the frontend:
```bash
cd frontend
npm start
```

5. Visit http://localhost:3000 to view the DAG visualization

## Development

- Backend runs on http://localhost:8000
- Frontend runs on http://localhost:3000
- API documentation available at http://localhost:8000/docs

## Testing

```bash
# Run backend tests
cd backend
cargo test

# Run frontend tests
cd frontend
npm test
```

## License

MIT License - see LICENSE file for details