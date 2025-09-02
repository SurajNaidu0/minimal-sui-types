# Minimal Sui Types

This is a minimal, SGX-compatible version of the `sui-types` crate that provides only the essential types needed for basic Sui transaction handling.

## What's Included

This minimal version includes only the types you specifically requested:

- `sui_types::{message_envelope::Envelope, transaction::SenderSignedData, crypto::EmptySignInfo}`
- `sui_types::{base_types::SuiAddress, crypto::SuiKeyPair, transaction::{Transaction, TransactionData}}`

## What's Removed

All networking-related dependencies that cause SGX compilation issues have been removed:
- `socket2` and other networking crates
- `anemo` (network protocol)
- `tonic` (gRPC)
- `mysten-network` 
- Complex consensus and authority types
- Database and storage dependencies

## Usage

Replace your `Cargo.toml` dependency:

```toml
# Instead of:
# sui-types = { git = "https://github.com/mystenlabs/sui", package = "sui-types" }

# Use this local minimal version:
minimal-sui-types = { path = "./minimal-sui-types" }
```

Then in your code:

```rust
use minimal_sui_types::{
    message_envelope::Envelope, 
    transaction::SenderSignedData, 
    crypto::EmptySignInfo
};

use minimal_sui_types::{
    base_types::SuiAddress, 
    crypto::SuiKeyPair,
    transaction::{Transaction, TransactionData}
};
```

## Features

- **SGX Compatible**: No networking or system-specific dependencies
- **Minimal Dependencies**: Only essential crates like `serde`, `fastcrypto`, `move-core-types`
- **Core Functionality**: Basic transaction types, addresses, and crypto primitives
- **Serialization**: Full serde support for JSON and BCS

## Limitations

This is a simplified implementation and may not support all advanced Sui features:
- No authority consensus logic
- No network protocol handling
- Simplified transaction validation
- Basic signature verification only

## Building

```bash
cd minimal-sui-types
cargo build
```

## Testing

```bash
cargo test
```

## License

Apache-2.0 (same as original sui-types)
