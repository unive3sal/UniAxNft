# AGENTS.md - UniAxNft

This document provides guidelines for AI coding agents working in this Rust/Axum NFT marketplace backend.

## Project Overview

- **Language**: Rust (edition 2024)
- **Framework**: Axum 0.8.7 (async web framework on Tokio)
- **Database**: PostgreSQL via SQLx with compile-time query checking
- **Blockchain**: Solana (NFT minting/management)
- **External Services**: Pinata (IPFS storage)

## Build Commands

```bash
cargo build                    # Development build
cargo build --release          # Optimized release build
cargo run                      # Run the application
cargo check                    # Quick compilation check (no binary output)
```

## Lint Commands

```bash
cargo clippy                   # Run Rust linter
cargo clippy -- -D warnings    # Treat warnings as errors
cargo fmt                      # Format code
cargo fmt -- --check           # Check formatting without modifying
```

## Test Commands

```bash
cargo test                              # Run all tests
cargo test test_name                    # Run specific test by name
cargo test test_name -- --nocapture     # Run test with stdout output
cargo test module::test_name            # Run test in specific module
cargo test -- --test-threads=1          # Run tests sequentially
```

## Database Commands

```bash
sqlx migrate run               # Run pending migrations
sqlx migrate revert            # Revert last migration
sqlx migrate add <name>        # Create new migration files
```

Migrations are in `./migrations/` and run automatically on startup via `sqlx::migrate!()`.

## Project Structure

```
src/
├── main.rs              # Entry point, router setup, middleware layers
├── config.rs            # Configuration structs, environment loading
├── state.rs             # Application state (shared across handlers)
├── error.rs             # Custom error types with HTTP status mapping
├── database/
│   └── connection.rs    # PostgreSQL pool creation
├── middleware/
│   └── auth.rs          # JWT authentication middleware
├── services/
│   ├── pinata.rs        # IPFS/Pinata file upload service
│   └── nft.rs           # Solana NFT service
├── routes/              # HTTP handlers (to be implemented)
└── logging/             # Logging system (to be implemented)
migrations/              # SQLx database migrations
unittest/                # Unit tests directory
```

## Code Style Guidelines

### Import Order

1. Standard library imports (`std::`)
2. External crate imports
3. Internal modules with `crate::` prefix

```rust
use std::sync::Arc;
use std::time::Duration;

use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::config::Config;
use crate::error::{UniAxNftErr, UniAxNftResult};
```

### Import Formatting

- Group related imports with braces
- Multi-line imports for 3+ items:
```rust
use axum::{
    http,
    routing::{get, post, patch, delete},
    Router
};
```

### Naming Conventions

| Type          | Convention             | Example                        |
|---------------|------------------------|--------------------------------|
| Types/Structs | PascalCase             | `UniAxNftState`, `PinataSrv`   |
| Functions     | snake_case             | `create_sql_pool`, `from_env`  |
| Constants     | SCREAMING_SNAKE_CASE   | `JWT_KEYPAIR`                  |
| Modules       | snake_case             | `middleware`, `services`       |
| Error variants| PascalCase + Err suffix| `ConfigErr`, `DatabaseErr`     |
| Services      | PascalCase + Srv suffix| `PinataSrv`, `NftSrv`          |

### Error Handling

Use the custom error type defined in `src/error.rs`:

```rust
use crate::error::{UniAxNftErr, UniAxNftResult};

// Return custom Result type
pub fn example() -> UniAxNftResult<String> {
    some_operation()
        .map_err(|e| UniAxNftErr::ConfigErr(
            format!("operation failed: {}", e)
        ))?;
    Ok("success".to_string())
}
```

Error enum variants map to HTTP status codes:
- `ConfigErr` -> 503 Service Unavailable
- `DatabaseErr` -> 500 Internal Server Error
- `PinataErr` -> 500 Internal Server Error
- `SolanaErr` -> 500 Internal Server Error
- `AuthErr` -> 401 Unauthorized
- `InvalidToken` -> 400 Bad Request

### Struct Patterns

Services use `new()` constructor pattern:
```rust
pub struct ExampleSrv {
    client: Client,
    config: ExampleConfig,
}

impl ExampleSrv {
    pub fn new(config: ExampleConfig) -> Self {
        Self {
            client: Client::new(),
            config: config,
        }
    }
}
```

### Async/State Patterns

- Use `async/await` throughout with Tokio runtime
- Wrap shared state in `Arc<T>`
- Clone state when passing to handlers: `.with_state(state.clone())`

### Configuration

Environment variables loaded in `config.rs`. Required variables:
- `DATABASE_URL` - PostgreSQL connection string
- `SOLANA_SERVICE_WALLET` - Base58 encoded keypair
- `NFT_PROGRAM_ID` - Solana program public key
- `PINATA_GATEWAY`, `PINATA_UPLOAD_URL`, `PINATA_API_URL`
- `PINATA_API_KEY`, `PINATA_API_SECRET`, `PINATA_JWT`
- `SERVER_JWT_SECRET` - JWT signing secret

Optional with defaults:
- `SERVER_HOST` (default: "0.0.0.0")
- `SERVER_PORT` (default: "8080")
- `DATABASE_MIN_CONN` (default: "5")
- `DATABASE_MAX_CONN` (default: "20")
- `SOLANA_RPC_URL` (default: devnet)

### Module Declaration

Inline module declarations in `main.rs`:
```rust
mod config;
mod database {
    pub mod connection;
}
mod services {
    pub mod pinata;
    pub mod nft;
}
```

### HTTP Response Format

JSON error responses follow this structure:
```json
{
    "error": "Error message here",
    "status": 400
}
```

### Database Queries

Use SQLx with compile-time checked queries:
```rust
sqlx::query!("SELECT * FROM users WHERE id = $1", user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| UniAxNftErr::DatabaseErr(format!("query failed: {}", e)))?;
```

### Authentication

Protected routes use `AsyncRequireAuthorizationLayer` with custom `Authorizer`.
JWT tokens contain: `user_id`, `email`, `iat`, `exp` (1 hour expiry).

### API URL Conventions

- Use singular `/user/` prefix for user-related routes (not `/users/`)
- Path parameters use curly braces: `/user/{user_id}/nfts`

## Files to Never Commit

Per `.gitignore`:
- `/target` - Build artifacts
- `uniaxnft_env.sh` - Environment variables
- `.sqls/` - Local SQL files

## Key Dependencies

- `axum` / `tower-http` - Web framework and middleware
- `sqlx` - Async PostgreSQL with compile-time checks
- `jsonwebtoken` - JWT authentication
- `solana-sdk` / `solana-client` - Blockchain integration
- `reqwest` - HTTP client for external services
- `thiserror` - Error derive macros
- `tracing` - Logging/instrumentation
