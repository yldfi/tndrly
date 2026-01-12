# tndrly

Unofficial Rust client library for the Tenderly API.

## Build Commands

```bash
# Build library
cargo build

# Build release
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Project Structure

```
src/
├── lib.rs            # Library entry point and Client impl
├── client.rs         # Core HTTP client (reqwest)
├── error.rs          # Error types (thiserror)
├── utils.rs          # Address validation utilities
├── simulation/
│   ├── mod.rs        # Simulation module exports
│   ├── api.rs        # Simulation API client
│   └── types.rs      # SimulationRequest, SimulationResponse
├── vnets/
│   ├── mod.rs        # Virtual TestNets module exports
│   ├── api.rs        # Virtual TestNets API client
│   ├── admin_rpc.rs  # Admin RPC client (time, balance, storage, snapshots)
│   └── types.rs      # VNet, CreateVNetRequest, etc.
├── alerts/
│   ├── mod.rs        # Alerts module exports
│   ├── api.rs        # Alerts API client
│   └── types.rs      # AlertType, CreateAlertRequest, etc.
├── contracts/
│   ├── mod.rs        # Contracts module exports
│   ├── api.rs        # Contracts API client
│   └── types.rs      # Contract, AddContractRequest, etc.
├── actions/
│   ├── mod.rs        # Web3 Actions module exports
│   ├── api.rs        # Web3 Actions API client
│   └── types.rs      # ActionTrigger, CreateActionRequest, etc.
└── wallets/
    ├── mod.rs        # Wallets module exports
    ├── api.rs        # Wallets API client
    └── types.rs      # Wallet, AddWalletRequest, etc.
```

## Key Dependencies

- **reqwest**: HTTP client with rustls-tls
- **tokio**: Async runtime (rt-multi-thread, macros)
- **serde/serde_json**: Serialization
- **secrecy**: Secret protection for API keys
- **thiserror**: Error handling

## Environment Variables

- `TENDERLY_ACCESS_KEY` - API access key
- `TENDERLY_ACCOUNT` - Account slug
- `TENDERLY_PROJECT` - Project slug

## Git Hooks

Install pre-commit hooks to run fmt, clippy, and tests before each commit:

```bash
./.githooks/install.sh
```

## Code Style

- Use `#[must_use]` on all builder methods
- Use `#[non_exhaustive]` on all public enums
- Implement `FromStr`/`Display` for enums
- Use `SecretString` for sensitive values
- Validate addresses with `utils::is_valid_address()`

## Testing

```bash
# Run unit tests
cargo test

# Run examples (requires credentials)
cargo run --example test_connection
```

## Usage

```rust
use tndrly::Client;
use tndrly::simulation::SimulationRequest;

// From environment variables
let client = Client::from_env()?;

// Or construct directly
let client = Client::new(Config::new("access_key", "account", "project"))?;

// Use APIs
let result = client.simulation().simulate(&request).await?;
```

## Admin RPC (Virtual TestNets)

The Admin RPC client provides JSON-RPC methods for manipulating VNet state:

```rust
// Get admin RPC client for a VNet
let admin = client.vnets().admin_rpc("vnet-id").await?;

// Time manipulation
admin.increase_time(3600).await?;              // Advance 1 hour
admin.set_next_block_timestamp(1234567890).await?;
admin.increase_blocks(100).await?;

// Balance management
admin.set_balance("0x...", "1000000000000000000").await?;  // 1 ETH
admin.add_balance("0x...", "1000000000000000000").await?;
admin.set_erc20_balance("0xtoken", "0xwallet", "1000000").await?;

// Storage manipulation
admin.set_storage_at("0x...", "0x0", "0x1").await?;
admin.set_code("0x...", "0x6080...").await?;

// Snapshots
let snapshot_id = admin.snapshot().await?;
// ... do stuff ...
admin.revert(&snapshot_id).await?;
```
