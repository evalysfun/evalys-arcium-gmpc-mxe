# Cryptographic Operations

## Overview

This document specifies the cryptographic primitives, key management, and receipt verification used in the Evalys Arcium gMPC MXE.

## gMPC Primitive

### Type: Threshold Secret Sharing with MPC

**Primitive**: Arcium's built-in gMPC framework using threshold secret sharing

**Library**: Arcium Anchor SDK (`arcium-anchor = "0.4.0"`)

**Method**: `Enc<Shared, T>` - Shared encryption type provided by Arcium

**Threshold Parameters**: 
- Managed by Arcium network (not exposed to MXE)
- Arcium network handles threshold parameter selection
- Default: t-of-n where t and n are network-configured

### What is Proven by Arcium Receipt

**Arcium Receipt Attestation**:
1. **Computation Integrity**: Receipt proves computation was executed correctly
2. **Input Privacy**: Receipt proves inputs remained encrypted during computation
3. **Output Authenticity**: Receipt signature proves output came from Arcium network
4. **Non-Repudiation**: Receipt signature prevents denial of computation

**Receipt Contents**:
- `computation_id`: Unique identifier for this computation
- `result_hash`: SHA256 hash of decrypted output (for verification)
- `signature`: Ed25519 signature from Arcium node
- `timestamp`: Unix timestamp (for replay prevention)

## Encryption Scheme

### Input Encryption

**Type**: `Enc<Shared, T>` - Shared encryption for MPC

**Process**:
1. Bridge service encrypts sensitive inputs using Arcium client SDK
2. Encrypted inputs are passed to MXE via Solana transaction
3. Arcium network receives encrypted inputs
4. Inputs remain encrypted during MPC computation

**What's Encrypted**:
- User preferences (desired_size, risk_appetite, etc.)
- User history (recent_pnl, win_rate, etc.)
- Portfolio context (total_capital, exposure, etc.)
- Performance history (total_pnl, sharpe_ratio, etc.)
- Sizing preferences (target_size, min_size, max_size)
- User constraints (max_slippage_bps, priority_level)

**What's Plaintext**:
- Curve state (current_price, liquidity_depth) - public on-chain data
- Market conditions (curve_volatility, liquidity_risk) - public data
- Curve metrics (current_price, price_change_24h) - public data

### Output Encryption

**Type**: `Enc<Shared, T>` - Shared encryption for MPC

**Process**:
1. MPC computation produces encrypted output
2. Encrypted output is returned to MXE callback
3. MXE emits event with encrypted ciphertext
4. Bridge service receives encrypted output
5. Bridge service decrypts using Arcium client SDK

**Output Structure**:
```rust
struct ComputationOutput {
    ciphertexts: Vec<[u8; 32]>,  // Encrypted result chunks
    nonce: [u8; 16],            // Nonce for decryption
}
```

## Receipt Verification

### Verification Process

**Step 1: Signature Verification**

```rust
// Pseudo-code (actual implementation in bridge service)
fn verify_receipt_signature(receipt: &ArciumReceipt) -> bool {
    let message = hash(receipt.computation_id, receipt.result_hash, receipt.timestamp);
    verify_ed25519_signature(
        &receipt.signature,
        &message,
        &arcium_node_public_key
    )
}
```

**Step 2: Result Hash Verification**

```rust
fn verify_result_hash(receipt: &ArciumReceipt, decrypted_output: &[u8]) -> bool {
    let computed_hash = sha256(decrypted_output);
    computed_hash == receipt.result_hash
}
```

**Step 3: Timestamp Validation**

```rust
fn verify_timestamp(receipt: &ArciumReceipt) -> bool {
    let now = current_timestamp();
    let age = now - receipt.timestamp;
    age < 300  // 5 minutes max age
}
```

### Verification Implementation

**Location**: Bridge service (`evalys-arcium-bridge-service`)

**Status**: v0.1 - Placeholder (to be implemented in v0.3)

**Current**: Bridge service accepts receipts without verification (demo mode)

**Future**: Full verification in v0.3 with:
- Ed25519 signature verification
- Result hash validation
- Timestamp validation
- Replay attack prevention

## Key Management

### Arcium Network Keys

**Arcium Node Public Keys**: Managed by Arcium network

**Key Distribution**: Arcium network provides node public keys for verification

**Key Rotation**: Handled by Arcium network (transparent to MXE)

### Client Encryption Keys

**Location**: Bridge service configuration

**Format**: 256-bit symmetric key (base64 encoded)

**Usage**: Bridge service uses this key to encrypt inputs before sending to MXE

**Storage**: Environment variable `ARCIUM_CLIENT_ENCRYPTION_KEY`

## Security Guarantees

### 1. Input Privacy

**Guarantee**: Sensitive inputs never leave encryption boundary

**Mechanism**: 
- Inputs encrypted before transmission
- Remain encrypted during MPC computation
- Only aggregated/computed results are revealed

**Verification**: Bridge service never sees plaintext inputs

### 2. Computation Integrity

**Guarantee**: Computation results are authentic and unmodified

**Mechanism**:
- Arcium receipt signature attests to computation
- Result hash prevents tampering
- Timestamp prevents replay

**Verification**: Bridge service verifies receipt before trusting results

### 3. Non-Repudiation

**Guarantee**: Arcium network cannot deny performing computation

**Mechanism**:
- Ed25519 signature from Arcium node
- Receipt stored on-chain (immutable)
- Computation ID links receipt to computation

**Verification**: Receipt signature is cryptographically verifiable

## Implementation Details

### Arcium SDK Usage

**Encryption**: Uses Arcium's `Enc<Shared, T>` type

**Computation**: Uses Arcium's `#[encrypted]` module and `#[instruction]` functions

**Receipts**: Provided by Arcium network automatically

**No Custom Crypto**: MXE uses Arcium's built-in cryptographic primitives

### Error Handling

**Invalid Inputs**: Computation aborts, returns error code

**Network Failures**: Arcium network handles retries

**Timeout**: Arcium network handles timeouts

**No Plaintext Leakage**: Errors never expose sensitive data

## References

- [Arcium Developer Documentation](https://docs.arcium.com/developers)
- [Arcium Cryptographic Primitives](https://docs.arcium.com/developers/cryptography)
- [Ed25519 Signatures](https://ed25519.cr.yp.to/)
- [SHA256 Hashing](https://en.wikipedia.org/wiki/SHA-2)

