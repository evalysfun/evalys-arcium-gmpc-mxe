use arcis_imports::*;

/// Confidential curve evaluation for Evalys
/// 
/// Analyzes bonding curve with encrypted user context to provide
/// execution recommendations without exposing user preferences.
#[encrypted]
mod circuits {
    use arcis_imports::*;

    /// User sizing preferences (encrypted)
    pub struct SizingPreferences {
        pub target_size: u64,             // Target position size
        pub min_size: u64,                // Minimum acceptable size
        pub max_size: u64,                // Maximum acceptable size
        pub capital_allocation_pct: u8,    // Capital allocation percentage (0-100)
    }

    /// User constraints (encrypted)
    pub struct UserConstraints {
        pub max_slippage_bps: u16,        // Max slippage in basis points
        pub time_constraint_sec: u32,      // Time constraint for execution
        pub priority_level: u8,            // Priority: 0-255
    }

    /// Curve metrics (plaintext, from on-chain)
    pub struct CurveMetrics {
        pub current_price: u64,           // Current price
        pub price_change_24h: i32,         // 24h price change (can be negative)
        pub liquidity_depth: u64,         // Available liquidity
        pub buy_pressure: u16,             // Buy pressure indicator
        pub sell_pressure: u16,            // Sell pressure indicator
    }

    /// Execution recommendation (encrypted)
    pub struct ExecutionRecommendation {
        pub recommended_size: u64,        // Recommended execution size
        pub entry_price_target: u64,       // Target entry price
        pub execution_urgency: u8,         // Urgency: 0-255 (higher = more urgent)
        pub optimal_timing: u32,          // Optimal timing window in seconds
        pub confidence_score: u8,          // Confidence: 0-255
    }

    #[instruction]
    pub fn confidential_curve_eval(
        sizing: Enc<Shared, SizingPreferences>,
        constraints: Enc<Shared, UserConstraints>,
        curve: CurveMetrics,
    ) -> Enc<Shared, ExecutionRecommendation> {
        let size_prefs = sizing.to_arcis();
        let user_constraints = constraints.to_arcis();

        // Calculate recommended size based on curve liquidity and user preferences
        let recommended_size = if curve.liquidity_depth > size_prefs.max_size * 2 {
            // Plenty of liquidity: can execute full target
            size_prefs.target_size
        } else if curve.liquidity_depth > size_prefs.min_size {
            // Limited liquidity: use available liquidity with buffer
            (curve.liquidity_depth * 3) / 4 // Use 75% of available liquidity
        } else {
            // Very limited liquidity: use minimum
            size_prefs.min_size
        };

        // Ensure within user constraints
        let recommended_size = recommended_size.min(size_prefs.max_size).max(size_prefs.min_size);

        // Entry price target: consider current price and momentum
        let price_adjustment = if curve.price_change_24h > 1000 {
            // Strong upward momentum: slight premium
            (curve.current_price * 101) / 100
        } else if curve.price_change_24h < -1000 {
            // Downward momentum: can get discount
            (curve.current_price * 99) / 100
        } else {
            // Stable: use current price
            curve.current_price
        };
        let entry_price_target = price_adjustment;

        // Execution urgency based on buy/sell pressure and user priority
        let pressure_urgency = if curve.buy_pressure > curve.sell_pressure * 2 {
            200u8 // High buy pressure: urgent
        } else if curve.sell_pressure > curve.buy_pressure * 2 {
            50u8 // High sell pressure: wait
        } else {
            100u8 // Balanced: moderate urgency
        };

        let execution_urgency = ((pressure_urgency as u16 + user_constraints.priority_level as u16) / 2) as u8;

        // Optimal timing: balance user constraints with market conditions
        let optimal_timing = if execution_urgency > 200 {
            user_constraints.time_constraint_sec.min(60) // Urgent: max 60s
        } else if execution_urgency > 150 {
            user_constraints.time_constraint_sec.min(300) // High urgency: max 5min
        } else {
            user_constraints.time_constraint_sec // Normal: use user constraint
        };

        // Confidence score: based on liquidity depth and price stability
        let liquidity_confidence = if curve.liquidity_depth > recommended_size * 3 {
            200u8 // Plenty of liquidity
        } else if curve.liquidity_depth > recommended_size {
            150u8 // Adequate liquidity
        } else {
            100u8 // Limited liquidity
        };

        let stability_confidence = if curve.price_change_24h.abs() < 500 {
            200u8 // Stable price
        } else if curve.price_change_24h.abs() < 2000 {
            150u8 // Moderate movement
        } else {
            100u8 // High volatility
        };

        let confidence_score = ((liquidity_confidence as u16 + stability_confidence as u16) / 2) as u8;

        let recommendation = ExecutionRecommendation {
            recommended_size,
            entry_price_target,
            execution_urgency,
            optimal_timing,
            confidence_score,
        };

        sizing.owner.from_arcis(recommendation)
    }
}

