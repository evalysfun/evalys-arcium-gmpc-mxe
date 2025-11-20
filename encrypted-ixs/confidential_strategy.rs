use arcis_imports::*;

/// Confidential strategy planning for Evalys execution
/// 
/// Takes encrypted user preferences and history, combines with public curve state,
/// and returns an encrypted execution plan.
#[encrypted]
mod circuits {
    use arcis_imports::*;

    /// User preferences (encrypted)
    pub struct UserPreferences {
        pub desired_size: u64,           // Desired trade size in SOL
        pub slippage_tolerance: u16,     // Slippage tolerance in basis points
        pub risk_appetite: u8,           // Risk appetite: 0-255 (0 = conservative, 255 = aggressive)
        pub preferred_hold_time: u32,    // Preferred hold time in seconds
    }

    /// User history (encrypted)
    pub struct UserHistory {
        pub recent_pnl: i64,             // Recent PnL (can be negative)
        pub win_rate: u16,               // Win rate in basis points (0-10000)
        pub avg_hold_time: u32,          // Average hold time in seconds
        pub total_trades: u32,           // Total number of trades
    }

    /// Public curve state (plaintext, passed from on-chain)
    pub struct CurveState {
        pub current_price: u64,          // Current token price
        pub liquidity_depth: u64,         // Available liquidity
        pub volatility: u16,              // Volatility metric
        pub recent_volume: u64,           // Recent trading volume
    }

    /// Strategy plan output (encrypted)
    pub struct StrategyPlan {
        pub recommended_mode: u8,         // 0=Normal, 1=Stealth, 2=Max Ghost
        pub num_slices: u8,              // Recommended number of order slices
        pub slice_size_base: u64,         // Base slice size
        pub timing_window_sec: u32,       // Recommended timing window
        pub risk_level: u8,               // Computed risk level: 0-255
        pub max_notional: u64,            // Maximum notional to commit
    }

    #[instruction]
    pub fn confidential_strategy_plan(
        preferences: Enc<Shared, UserPreferences>,
        history: Enc<Shared, UserHistory>,
        curve_state: CurveState,
    ) -> Enc<Shared, StrategyPlan> {
        let prefs = preferences.to_arcis();
        let hist = history.to_arcis();

        // Risk assessment based on history and preferences
        let risk_score = compute_risk_score(&prefs, &hist, &curve_state);
        
        // Determine recommended privacy mode
        let recommended_mode = if risk_score > 200 {
            2u8 // Max Ghost for high risk
        } else if risk_score > 100 {
            1u8 // Stealth for medium risk
        } else {
            0u8 // Normal for low risk
        };

        // Calculate optimal slicing
        let num_slices = if prefs.desired_size > 10_000_000_000 {
            8u8 // Large orders: more slices
        } else if prefs.desired_size > 1_000_000_000 {
            5u8 // Medium orders
        } else {
            3u8 // Small orders: fewer slices
        };

        // Base slice size (desired_size / num_slices, with some randomization room)
        let slice_size_base = prefs.desired_size / (num_slices as u64);

        // Timing window based on volatility and risk
        let timing_window_sec = if curve_state.volatility > 500 {
            60u32 // High volatility: faster execution
        } else if curve_state.volatility > 200 {
            120u32 // Medium volatility
        } else {
            300u32 // Low volatility: can take more time
        };

        // Max notional: consider user's risk appetite and history
        let max_notional = if prefs.risk_appetite > 200 && hist.win_rate > 6000 {
            prefs.desired_size * 2 // Aggressive with good history: allow 2x
        } else if prefs.risk_appetite > 150 {
            prefs.desired_size * 3 / 2 // Moderate aggressive: 1.5x
        } else {
            prefs.desired_size // Conservative: stick to desired size
        };

        let plan = StrategyPlan {
            recommended_mode,
            num_slices,
            slice_size_base,
            timing_window_sec,
            risk_level: risk_score as u8,
            max_notional,
        };

        preferences.owner.from_arcis(plan)
    }

    /// Compute risk score (0-255) based on user data and curve state
    fn compute_risk_score(
        prefs: &UserPreferences,
        hist: &UserHistory,
        curve: &CurveState,
    ) -> u16 {
        // Base risk from user preferences
        let preference_risk = prefs.risk_appetite as u16;

        // History-based risk adjustment
        let history_risk = if hist.recent_pnl < 0 {
            // Negative PnL increases risk
            (prefs.risk_appetite as u16) + 50
        } else if hist.win_rate < 5000 {
            // Low win rate increases risk
            (prefs.risk_appetite as u16) + 30
        } else {
            // Good history reduces risk
            (prefs.risk_appetite as u16).saturating_sub(20)
        };

        // Curve volatility adds to risk
        let volatility_risk = (curve.volatility as u16) / 10;

        // Combine risks (cap at 255)
        let total_risk = history_risk + volatility_risk;
        if total_risk > 255 {
            255
        } else {
            total_risk as u16
        }
    }
}

