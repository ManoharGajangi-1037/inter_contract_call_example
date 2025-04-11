# Inter-Contract Call Example (CosmWasm)

This repository demonstrates how to implement **inter-contract calls** in CosmWasm using two example contracts: `contract_a` and `contract_b`.

## 📁 Project Structure

```

## Overview

This project showcases a simple message passing system between two CosmWasm smart contracts:

- **Contract A**: Acts as the sender contract that can initiate messages to Contract B
- **Contract B**: Acts as the receiver contract that stores messages and maintains an authorization system

### Features

- Message passing between contracts
- Authorization system for message reception
- Message storage implementation
- Query functionality for stored messages

## Contract Details

### Contract A
- Implements `instantiate`, `execute`, and `query` entry points
- Can send messages to Contract B using `SendMessage` execute message
- Forwards messages through CosmWasm's message passing system

### Contract B
- Implements secure message reception with authorization checks
- Stores received messages in contract state
- Provides query functionality to retrieve stored messages
- Uses `cw-storage-plus` for state management

## Message Types

### Contract A Execute Messages
```rust
pub enum ExecuteMsg {
    SendMessage { message: String, target: String },
    ReceiveCall { message: String },
}
```

### Contract B Execute Messages
```rust
pub enum ExecuteMsg {
    ReceiveCall { message: String },
}
```

### Contract B Query Messages
```rust
pub enum QueryMsg {
    GetMessages {},
}
```

## Building and Testing

To build the contracts:

```bash
cargo build --release --target wasm32-unknown-unknown
```

## Usage Example

1. Deploy both contracts to your CosmWasm-enabled blockchain
2. Initialize Contract B
3. Set Contract B's address as the target in Contract A's messages
4. Use Contract A's `SendMessage` execute message to send messages to Contract B
5. Query Contract B using `GetMessages` to retrieve stored messages

## State Management

Contract B maintains two main state items:
- `MESSAGES`: Stores all received messages
- `AUTHORIZED_CONTRACT`: Stores the address of the authorized sender contract

## Security

The contract implements several security measures:
- Authorization checks for message reception
- Proper error handling and input validation
- Use of CosmWasm's standard security practices

## Dependencies

- `cosmwasm-std`: Core CosmWasm dependencies
- `cosmwasm-schema`: For schema generation
- `cw-storage-plus`: For state management
- `serde`: For serialization/deserialization
- `thiserror`: For error handling

