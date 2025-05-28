# ZX - ParityDB Implementation

A blockchain application using ParityDB for efficient state storage.

## Features

- Key-value storage optimized for blockchain state
- Efficient Patricia-Merkle trie support
- Concurrent readers with serialized writes
- Atomic transactions
- OS page cache utilization

## Requirements

- Rust 1.87.0 or later
- Cargo package manager

## Installation

1. Clone the repository
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:
```bash
RUST_LOG=info cargo run
```

The application will:
1. Initialize a ParityDB database in the `db` directory
2. Write example data to the database
3. Read and verify the data
4. Log all operations

## Configuration

The database is configured with:
- 1 column for key-value storage
- Default options for blockchain state storage
- Automatic creation of database if it doesn't exist

## License

This project is licensed under the MIT License.