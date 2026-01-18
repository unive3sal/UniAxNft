# UniAxNft

A backend API for NFT management built with Rust, Axum, Solana, and IPFS (Pinata).

## Tech Stack

- **Framework**: Axum 0.8
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT with bcrypt password hashing
- **Blockchain**: Solana (devnet by default)
- **Storage**: Pinata (IPFS)
- **Runtime**: Tokio

## Features

- User registration and authentication
- JWT-based authorization
- Password management
- IPFS file uploads via Pinata
- Solana blockchain integration for NFT operations

## Prerequisites

- Rust (2024 edition)
- PostgreSQL
- Solana CLI (optional, for wallet management)
- Pinata account

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `SERVER_HOST` | No | `0.0.0.0` | Server bind address |
| `SERVER_PORT` | No | `8080` | Server port |
| `DATABASE_URL` | Yes | - | PostgreSQL connection string |
| `DATABASE_MIN_CONN` | No | `5` | Minimum database connections |
| `DATABASE_MAX_CONN` | No | `20` | Maximum database connections |
| `SOLANA_RPC_URL` | No | `https://api.devnet.solana.com` | Solana RPC endpoint |
| `SOLANA_SERVICE_WALLET` | Yes | - | Base58-encoded service wallet keypair |
| `NFT_PROGRAM_ID` | Yes | - | NFT program public key |
| `PINATA_GATEWAY` | Yes | - | Pinata gateway URL |
| `PINATA_UPLOAD_URL` | Yes | - | Pinata upload API URL |
| `PINATA_API_URL` | Yes | - | Pinata API URL |
| `PINATA_API_KEY` | Yes | - | Pinata API key |
| `PINATA_API_SECRET` | Yes | - | Pinata API secret |
| `PINATA_JWT` | Yes | - | Pinata JWT token |
| `JWT_PRIVATE_KEY` | Yes | - | RSA private key for JWT signing |
| `JWT_PUBLIC_KEY` | Yes | - | RSA public key for JWT verification |

## API Endpoints

### Public Routes

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `POST` | `/api/v1/auth/register` | User registration |
| `POST` | `/api/v1/auth/login` | User login |

### Protected Routes (require Authorization header)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `PUT` | `/api/v1/user/change_pwd` | Change password |
| `GET` | `/api/v1/user/{user_id}/nfts` | Get user NFT information |

## Running

```bash
# Install dependencies and build
cargo build

# Run database migrations (handled automatically on startup)
# Migrations are in ./migrations/

# Run the server
cargo run
```

## Database Schema

### Users Table

| Column | Type | Description |
|--------|------|-------------|
| `id` | UUID | Primary key |
| `email` | VARCHAR(255) | Unique email |
| `username` | VARCHAR(100) | Unique username |
| `password_hash` | VARCHAR(255) | Bcrypt hashed password |
| `created_at` | TIMESTAMP | Creation timestamp |
| `updated_at` | TIMESTAMP | Last update timestamp |
| `last_login_at` | TIMESTAMP | Last login timestamp |
| `is_active` | BOOLEAN | Account status |

## License

GPL-3.0
