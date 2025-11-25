# MXE Specification v0.1

## Purpose

This MXE performs confidential gMPC computation for Evalys trading intents. It provides a unified interface for:

1. **Confidential Strategy Planning**: Strategy generation from encrypted user preferences and history
2. **Confidential Risk Scoring**: Risk assessment using encrypted portfolio context
3. **Confidential Curve Evaluation**: Curve analysis with encrypted user constraints
4. **gMPC Strategy Planning**: Encrypted intent processing for execution plan generation
5. **Multi-User Analytics**: Aggregated insights across users without exposing individual behavior

## Type

**Recurring MXE**: This MXE is designed for multiple invocations. Each computation is stateless and independent. The MXE can handle concurrent requests from multiple Evalys bridge services.

**Rationale**: Trading intents are independent operations. No persistent state is needed between computations. This enables horizontal scaling and reduces complexity.

## Inputs (Encrypted)

### 1. Confidential Strategy Plan

**Encrypted Inputs**:
- `UserPreferences` (Enc<Shared, UserPreferences>):
  - `desired_size: u64` - Desired trade size in lamports
  - `slippage_tolerance: u16` - Slippage tolerance in basis points
  - `risk_appetite: u8` - Risk appetite: 0-255
  - `preferred_hold_time: u32` - Preferred hold time in seconds

- `UserHistory` (Enc<Shared, UserHistory>):
  - `recent_pnl: i64` - Recent PnL (can be negative)
  - `win_rate: u16` - Win rate in basis points (0-10000)
  - `avg_hold_time: u32` - Average hold time in seconds
  - `total_trades: u32` - Total number of trades

**Plaintext Inputs**:
- `CurveState` (CurveState):
  - `current_price: u64` - Current token price
  - `liquidity_depth: u64` - Available liquidity
  - `volatility: u16` - Volatility metric
  - `recent_volume: u64` - Recent trading volume

### 2. Confidential Risk Score

**Encrypted Inputs**:
- `PortfolioContext` (Enc<Shared, PortfolioContext>)
- `PerformanceHistory` (Enc<Shared, PerformanceHistory>)

**Plaintext Inputs**:
- `MarketConditions` (MarketConditions)

### 3. Confidential Curve Evaluation

**Encrypted Inputs**:
- `SizingPreferences` (Enc<Shared, SizingPreferences>)
- `UserConstraints` (Enc<Shared, UserConstraints>)

**Plaintext Inputs**:
- `CurveMetrics` (CurveMetrics)

### 4. gMPC Strategy

**Encrypted Inputs**:
- `IntentInput` (Enc<Shared, IntentInput>):
  - `max_size_sol: u64` - Maximum size in SOL
  - `risk_level: u8` - Risk level: 0-255
  - `privacy_priority: u8` - Privacy priority: 0-255
  - `market_snapshot: MarketSnapshot` - Market state
  - `historical_stats: HistoricalStats` - Historical performance

### 5. Multi-User Analytics

**Encrypted Inputs**:
- `UserProfiles` (Enc<Shared, Vec<UserProfile>>) - List of anonymized user profiles

**Plaintext Inputs**:
- `AggregationType` (u8) - Type of analytics to compute

## Outputs

### 1. Confidential Strategy Plan

**Output**: `StrategyPlan` (Enc<Shared, StrategyPlan>)
- `recommended_mode: u8` - 0=Normal, 1=Stealth, 2=Max Ghost
- `num_slices: u8` - Recommended number of order slices
- `slice_size_base: u64` - Base slice size
- `timing_window_sec: u32` - Recommended timing window
- `risk_level: u8` - Computed risk level: 0-255
- `max_notional: u64` - Maximum notional to commit

**Arcium Receipt**: Standard Arcium receipt with:
- Computation ID
- Result hash (SHA256 of decrypted output)
- Arcium node signature (Ed25519)
- Timestamp

### 2. Confidential Risk Score

**Output**: `RiskAssessment` (Enc<Shared, RiskAssessment>)
- `overall_risk_score: u8` - Overall risk: 0-255
- `portfolio_risk: u8` - Portfolio-specific risk
- `trade_risk: u8` - Trade-specific risk
- `recommendation: u8` - 0=proceed, 1=caution, 2=avoid

### 3. Confidential Curve Evaluation

**Output**: `ExecutionRecommendation` (Enc<Shared, ExecutionRecommendation>)
- `recommended_size: u64` - Recommended execution size
- `entry_price_target: u64` - Target entry price
- `execution_urgency: u8` - Urgency: 0-255
- `optimal_timing: u32` - Optimal timing window in seconds
- `confidence_score: u8` - Confidence: 0-255

### 4. gMPC Strategy

**Output**: `PlanOutput` (Enc<Shared, PlanOutput>)
- `recommended_size_sol: u64` - Recommended size in SOL
- `slice_count: u8` - Number of slices
- `time_window_sec: u32` - Time window in seconds
- `mev_route: u8` - MEV route: 0=direct, 1=Jito bundle
- `privacy_mode: u8` - Privacy mode: 0=Normal, 1=Stealth, 2=Max Ghost
- `risk_class: u8` - Risk class: 0=low, 1=balanced, 2=high

### 5. Multi-User Analytics

**Output**: `AggregatedMetrics` (Enc<Shared, AggregatedMetrics>)
- Aggregated insights (structure depends on aggregation type)
- `confidence_score: u8` - Confidence: 0-255
- `sample_size: u32` - Number of users in aggregation

## Error Codes

- `AbortedComputation`: Computation was aborted (invalid inputs, timeout, etc.)

## Invariants

### 1. Plaintext Never Exits MXE

**Rule**: No plaintext sensitive data ever leaves the MXE boundary.

**Enforcement**:
- All sensitive inputs are encrypted (Enc<Shared, T>)
- All outputs are encrypted (Enc<Shared, T>)
- Only public/plaintext data (curve state, market conditions) is passed unencrypted
- Events emitted contain only encrypted ciphertexts and nonces

**Verification**: Bridge service verifies that all outputs are encrypted before decryption.

### 2. Output is Valid Only if Receipt Verifies

**Rule**: Bridge service must verify Arcium receipt before trusting computation results.

**Enforcement**:
- Arcium network provides receipt with signature
- Bridge verifies receipt signature using Arcium node public key
- Bridge verifies result hash matches decrypted output
- Bridge verifies timestamp is recent (replay prevention)

**Verification**: See `docs/crypto.md` for receipt verification process.

### 3. No Persistent State

**Rule**: MXE is stateless. Each computation is independent.

**Enforcement**:
- No account state is modified between computations
- Each computation uses fresh accounts
- No cross-computation data sharing

**Rationale**: Arcium network is stateless. This enables horizontal scaling and simplifies security model.

### 4. Deterministic Computation

**Rule**: Same encrypted inputs → same encrypted output (within MPC randomness bounds).

**Enforcement**:
- Computation logic is deterministic
- Randomness is bounded (for privacy, not for correctness)
- Output structure is fixed

**Verification**: Local tests verify deterministic behavior.

## Computation Lifecycle

1. **Initialization**: Bridge service calls `init_*_comp_def()` to initialize computation definition
2. **Request**: Bridge service calls `request_*()` with encrypted inputs
3. **Execution**: Arcium MPC cluster executes encrypted computation
4. **Callback**: MXE callback receives encrypted output
5. **Event**: MXE emits event with encrypted result
6. **Verification**: Bridge service verifies receipt and decrypts result

## Receipt Structure

**Arcium Receipt** (from Arcium network):
```rust
struct ArciumReceipt {
    receipt_id: [u8; 32],
    computation_id: Pubkey,
    result_hash: [u8; 32],  // SHA256 of decrypted output
    signature: [u8; 64],    // Ed25519 signature from Arcium node
    timestamp: i64,         // Unix timestamp
    status: u8,             // 0=completed, 1=failed
}
```

**Verification**: Bridge service verifies:
1. Signature is valid (Ed25519)
2. Result hash matches decrypted output
3. Timestamp is recent (< 5 minutes old)
4. Status is "completed"

## Versioning

### Current Version: v0.1

**Status**: Alpha - Basic structure and encrypted instructions defined

**Implemented**:
- ✅ Encrypted instruction definitions (Arcis DSL)
- ✅ Solana program structure (Anchor + Arcium)
- ✅ Computation definitions for all 5 operations
- ✅ Event emission for results
- ✅ Basic error handling

**Not Yet Implemented**:
- ❌ Actual gMPC primitive integration (using Arcium's built-in primitives)
- ❌ Receipt verification in bridge service
- ❌ Multi-cluster redundancy
- ❌ Performance optimizations

### Future Versions

- **v0.2**: Real gMPC primitive integration
- **v0.3**: Receipt verification and proof validation
- **v0.4**: Multi-cluster redundancy
- **v0.5**: Performance optimizations and caching

## References

- [Arcium Developer Documentation](https://docs.arcium.com/developers)
- [Arcium GitHub Organization](https://github.com/orgs/arcium-hq/)
- [Arcium Discord](https://discord.com/invite/arcium)

