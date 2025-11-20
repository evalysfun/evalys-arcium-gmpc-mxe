# Evalys Arcium gMPC MXE

**Unified Arcium MXE for Confidential Intel and gMPC Strategy Planning**

This repository contains the fused Arcium Multi-Party Execution Environment (MXE) that combines confidential intelligence operations with generalized Multi-Party Computation (gMPC) for encrypted intent processing.

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

## Deployment

### Prerequisites

- Solana CLI tools installed
- Arcium CLI tools installed
- Keypair for deployment

### Deploy to Devnet

```bash
cd evalys-arcium-gmpc-mxe
arcium deploy \
  --cluster-offset 1078779259 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://devnet.helius-rpc.com/?api-key=<your-key>
```

### Deploy to Mainnet

```bash
arcium deploy \
  --cluster-offset <mainnet-cluster-offset> \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://api.mainnet-beta.solana.com
```

## Integration

This MXE is used by:

- **evalys-arcium-bridge-service**: For general confidential computation (strategy, risk, curve)
- **evalys-arcium-gMPC**: For encrypted intent processing and multi-user analytics

Both services can call the same MXE for different use cases.

## Development

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Generate TypeScript Types

```bash
anchor build
anchor idl parse -f target/idl/evalys_arcium_gmpc_mxe.json -o types/
```

## License

MIT License - See LICENSE file for details.

## Repository

`https://github.com/evalysfun/evalys-arcium-gmpc-mxe`

