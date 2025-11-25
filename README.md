# Evalys Arcium gMPC MXE

**Unified [Arcium](https://arcium.com/) MXE for Confidential Intel and gMPC Strategy Planning**

This repository contains the fused [Arcium Multi-Party Execution Environment (MXE)](https://docs.arcium.com/developers) that combines confidential intelligence operations with generalized Multi-Party Computation (gMPC) for encrypted intent processing.

**Built with Arcium**: This MXE uses [Arcium's Arcis framework](https://docs.arcium.com/developers) to write confidential instructions in Rust. Learn more about Arcium:
- [Arcium Developer Documentation](https://docs.arcium.com/developers)
- [Arcium GitHub Organization](https://github.com/orgs/arcium-hq/)
- [Arcium Discord Community](https://discord.com/invite/arcium)

## Status: v0.1 (Alpha)

### What's Live Now (v0.1)

✅ **Program Structure**: Complete Solana program with Arcium integration
- Anchor + Arcium program structure
- 5 encrypted instruction definitions (Arcis DSL)
- Computation definitions for all operations
- Event emission for results
- Error handling

✅ **Encrypted Instructions**: All 5 instruction types defined:
- `confidential_strategy_plan()` - Strategy generation
- `confidential_risk_score()` - Risk assessment
- `confidential_curve_eval()` - Curve evaluation
- `evalys_gmpc_strategy()` - gMPC intent processing
- `confidential_multi_user_analytics()` - Multi-user aggregation

✅ **Build System**: Arcium.toml and Cargo.toml configured
- Can build with `arcium build`
- Can test with `anchor test`

✅ **Documentation**: MXE spec and crypto docs
- `docs/mxe-spec.md` - Complete MXE specification
- `docs/crypto.md` - Cryptographic operations

✅ **Deployment Script**: `scripts/deploy-devnet.sh` for deployment

✅ **Local Test Harness**: `examples/local-test.ts` for testing

### What's Next

🔜 **v0.2**: Real gMPC Primitive Integration
- Actual Arcium gMPC primitive usage
- Threshold secret sharing implementation
- Performance optimizations

🔜 **v0.3**: Receipt Verification
- Bridge service receipt verification
- Result hash validation
- Timestamp validation

🔜 **v0.4**: Multi-Cluster Redundancy
- Support for multiple Arcium clusters
- Automatic failover
- Load balancing

🔜 **v0.5**: Performance Optimizations
- Caching for repeated computations
- Batch processing support
- Optimized encryption/decryption

## Overview

This MXE provides a unified interface for all Evalys confidential computation needs:

- **Confidential Strategy Planning**: Strategy generation from encrypted user preferences and history
- **Confidential Risk Scoring**: Risk assessment using encrypted portfolio context
- **Confidential Curve Evaluation**: Curve analysis with encrypted user constraints
- **gMPC Strategy Planning**: Encrypted intent processing for execution plan generation
- **Multi-User Analytics**: Aggregated insights across users without exposing individual behavior

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│         Evalys Arcium gMPC MXE (Solana Program)             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Encrypted Instructions:                             │  │
│  │  • confidential_strategy_plan()                      │  │
│  │  • confidential_risk_score()                         │  │
│  │  • confidential_curve_eval()                         │  │
│  │  • evalys_gmpc_strategy()                            │  │
│  │  • confidential_multi_user_analytics()                │  │
│  └──────────────────────────────────────────────────────┘  │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Arcium MPC Cluster (Off-Chain)                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  • Executes encrypted computations                  │  │
│  │  • Returns encrypted results                        │  │
│  │  • Never exposes raw data                           │  │
│  │  • Data encrypted even during computation (gMPC)    │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Encrypted Instructions

### 1. confidential_strategy_plan()

Generates execution strategy from encrypted user preferences, history, and public curve state.

**Inputs**:
- `UserPreferences` (encrypted): Desired size, slippage tolerance, risk appetite, hold time
- `UserHistory` (encrypted): Recent PnL, win rate, average hold time, total trades
- `CurveState` (plaintext): Current price, liquidity depth, volatility, recent volume

**Output**: `StrategyPlan` (encrypted)
- Recommended privacy mode
- Number of order slices
- Slice size base
- Timing window
- Risk level
- Max notional

### 2. confidential_risk_score()

Evaluates trade risk using encrypted portfolio context and performance history.

**Inputs**:
- `PortfolioContext` (encrypted): Total capital, exposure, diversification, leverage
- `PerformanceHistory` (encrypted): Total PnL, Sharpe ratio, max drawdown, consistency
- `MarketConditions` (plaintext): Curve volatility, liquidity risk, market sentiment

**Output**: `RiskAssessment` (encrypted)
- Overall risk score
- Portfolio-specific risk
- Trade-specific risk
- Recommendation (proceed/caution/avoid)

### 3. confidential_curve_eval()

Analyzes bonding curve with encrypted user context for execution recommendations.

**Inputs**:
- `SizingPreferences` (encrypted): Target/min/max size, capital allocation
- `UserConstraints` (encrypted): Max slippage, time constraint, priority
- `CurveMetrics` (plaintext): Current price, price change, liquidity, buy/sell pressure

**Output**: `ExecutionRecommendation` (encrypted)
- Recommended size
- Entry price target
- Execution urgency
- Optimal timing
- Confidence score

### 4. evalys_gmpc_strategy()

gMPC-powered encrypted intent processing for execution plan generation.

**Inputs**:
- `IntentInput` (encrypted): Max size, risk level, privacy priority, market snapshot, historical stats

**Output**: `PlanOutput` (encrypted)
- Recommended size
- Slice count
- Time window
- MEV route
- Privacy mode
- Risk class

### 5. confidential_multi_user_analytics()

Aggregates insights across multiple users without exposing individual behavior.

**Inputs**:
- `UserProfiles` (encrypted): List of anonymized user profiles
- `AggregationType` (plaintext): Type of analytics (curve_insights, market_sentiment, etc.)

**Output**: `AggregatedMetrics` (encrypted)
- Aggregated insights
- Confidence score
- Sample size

## Documentation

- **[MXE Specification](docs/mxe-spec.md)**: Complete specification with inputs, outputs, invariants, and receipt structure
- **[Cryptographic Operations](docs/crypto.md)**: gMPC primitives, encryption scheme, and receipt verification

## Deployment

### Prerequisites

- Solana CLI tools installed
- Arcium CLI tools installed
- Keypair for deployment (generate with `solana-keygen new`)

### Deploy to Devnet

**Using the deployment script (recommended)**:

```bash
# Make script executable (Linux/Mac)
chmod +x scripts/deploy-devnet.sh

# Deploy
./scripts/deploy-devnet.sh

# Or with custom options
./scripts/deploy-devnet.sh \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://devnet.helius-rpc.com/?api-key=<your-key>
```

**Manual deployment**:

```bash
# Build first
arcium build

# Deploy
arcium deploy \
  --cluster-offset 1078779259 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://api.devnet.solana.com
```

### Deploy to Mainnet

```bash
./scripts/deploy-devnet.sh \
  --cluster-offset <mainnet-cluster-offset> \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://api.mainnet-beta.solana.com
```

**Note**: Update the script name or create `scripts/deploy-mainnet.sh` for mainnet-specific deployment.

## Integration

This MXE is used by:

- **evalys-arcium-bridge-service**: For general confidential computation (strategy, risk, curve)
- **evalys-arcium-gMPC**: For encrypted intent processing and multi-user analytics

Both services can call the same MXE for different use cases. See the bridge service documentation for integration details.

## Verification Checklist

To verify this MXE is production-ready:

- ✅ **Can I build it?** - Yes: `arcium build`
- ✅ **Can I run it locally?** - Yes: `npx ts-node examples/local-test.ts`
- ✅ **Can I deploy it?** - Yes: `./scripts/deploy-devnet.sh`
- ✅ **Do I know input/output types?** - Yes: See `docs/mxe-spec.md`
- ✅ **Do I know receipt verification?** - Yes: See `docs/crypto.md`

## Development

This MXE is built using [Arcium's development tools](https://docs.arcium.com/developers/installation). The `arcium` CLI is a wrapper over the `anchor` CLI that adds Arcium-specific functionality.

### Build

```bash
# Using Arcium CLI (recommended)
arcium build

# Or using Anchor directly
anchor build
```

### Test Locally

**Quick Demo (for screen recording)**:

```bash
# Windows PowerShell
.\demo.ps1

# Linux/Mac
chmod +x demo.sh
./demo.sh
```

**Using the test harness**:

```bash
# Install dependencies
npm install  # or yarn install

# Run local test
npx ts-node examples/local-test.ts
```

**Using Anchor tests**:

```bash
anchor test
```

### Generate TypeScript Types

```bash
anchor build
anchor idl parse -f target/idl/evalys_arcium_gmpc_mxe.json -o types/
```

## Project Structure

```
evalys-arcium-gmpc-mxe/
├── programs/
│   └── evalys-arcium-gmpc-mxe/
│       ├── src/
│       │   └── lib.rs          # Main program entry point
│       └── Cargo.toml          # Rust dependencies
├── encrypted-ixs/              # Encrypted instructions (Arcis DSL)
│   ├── confidential_strategy.rs
│   ├── confidential_risk.rs
│   ├── confidential_curve.rs
│   ├── evalys_gmpc_strategy.rs
│   └── confidential_multi_user.rs
├── docs/                       # Documentation
│   ├── mxe-spec.md            # MXE specification
│   └── crypto.md              # Cryptographic operations
├── scripts/                    # Deployment scripts
│   └── deploy-devnet.sh       # Devnet deployment script
├── examples/                   # Example code
│   └── local-test.ts          # Local test harness
├── Arcium.toml                # Arcium workspace configuration
└── README.md
```

### Arcium Development Resources

- [Arcium Installation Guide](https://docs.arcium.com/developers/installation) - Set up Arcium development environment
- [Hello World Tutorial](https://docs.arcium.com/developers/hello-world) - Create your first confidential instruction
- [Computation Lifecycle](https://docs.arcium.com/developers/computation-lifecycle) - Understand how confidential computations work
- [Arcium Examples](https://github.com/orgs/arcium-hq/) - Example Arcium applications
- [TypeScript SDK](https://ts.arcium.com/api) - Client library documentation

## License

MIT License - See LICENSE file for details.

## Repository

`https://github.com/evalysfun/evalys-arcium-gmpc-mxe`

## Arcium Resources

**Learn More About Arcium**:
- [Arcium Website](https://arcium.com/) - The encrypted supercomputer
- [Arcium Developer Documentation](https://docs.arcium.com/developers) - Complete developer guide
- [Arcium GitHub Organization](https://github.com/orgs/arcium-hq/) - Source code, examples, and tools
- [Arcium Discord](https://discord.com/invite/arcium) - Join the community

**Arcium enables**:
- Privacy-preserving applications on Solana
- Processing sensitive data while keeping it encrypted
- Familiar tooling (Arcis extends Anchor)
- Full composability within Solana ecosystem

This MXE demonstrates how to build confidential computation applications using Arcium's technology. For more examples and tutorials, visit the [Arcium Developer Documentation](https://docs.arcium.com/developers).

