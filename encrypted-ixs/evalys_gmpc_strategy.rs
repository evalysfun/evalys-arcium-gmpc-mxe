use arcis_imports::*;

/// Evalys gMPC Strategy Planning
/// Confidential computation for execution strategy generation using generalized MPC
/// 
/// All computation happens inside MPC - inputs remain encrypted even during computation
#[encrypted]
mod circuits {
    use arcis_imports::*;

    /// Intent input structure for gMPC computation (encrypted)
    pub struct IntentInput {
        pub max_size_sol: u64,            // Max size in lamports
        pub risk_level: u8,                // 0=low, 1=normal, 2=high
        pub privacy_priority: u8,          // 0=normal, 1=stealth, 2=max_privacy
        pub market_price: u64,             // Current market price
        pub curve_position: u16,           // Curve position (0-10000, basis points)
        pub volatility_score: u16,          // Volatility score (0-10000, basis points)
        pub avg_hold_time: u32,            // Average hold time in seconds
        pub win_rate: u16,                 // Win rate (0-10000, basis points)
        pub max_dd: u16,                   // Max drawdown (0-10000, basis points)
    }

    /// Execution plan output from gMPC (encrypted)
    pub struct PlanOutput {
        pub recommended_size_sol: u64,    // Recommended size in lamports
        pub slice_count: u8,              // Number of order slices
        pub time_window_sec: u32,         // Time window in seconds
        pub mev_route: u8,                // 0=standard, 1=jito_bundle, 2=private_route
        pub privacy_mode: u8,              // 0=normal, 1=stealth, 2=max_ghost
        pub risk_class: u8,               // 0=low, 1=balanced, 2=high
    }

    #[instruction]
    pub fn evalys_gmpc_strategy(
        intent: Enc<Shared, IntentInput>,
    ) -> Enc<Shared, PlanOutput> {
        let input = intent.to_arcis();

        // Base size calculation with risk adjustment
        let base = input.max_size_sol;
        
        // Risk factor based on risk level
        let risk_factor = match input.risk_level {
            0 => 50u64,   // Low risk - reduce to 50%
            1 => 80u64,   // Normal risk - 80%
            _ => 100u64,  // High risk - full size
        };
        
        // Volatility penalty (volatility_score is in basis points, so 7000 = 0.7)
        let vol_penalty = if input.volatility_score > 7000 {
            70u64  // High volatility - reduce to 70%
        } else {
            100u64 // Normal volatility
        };
        
        // Recommended size: base * risk_factor% * vol_penalty%
        let recommended = (base * risk_factor * vol_penalty) / 10000;
        
        // Slice count based on privacy priority
        let slice_count = match input.privacy_priority {
            0 => 3u8,  // Normal privacy
            1 => 5u8,  // Stealth
            _ => 7u8,  // Max privacy
        };
        
        // Time window based on volatility
        let time_window = if input.volatility_score > 7000 {
            60u32  // High volatility - longer window
        } else {
            38u32  // Standard window
        };
        
        // MEV route selection
        let mev_route = if input.privacy_priority >= 1 {
            1u8  // Jito bundle for stealth+
        } else {
            0u8  // Standard route
        };
        
        // Privacy mode selection
        let privacy_mode = match input.privacy_priority {
            0 => 0u8,  // Normal
            1 => 1u8,  // Stealth
            _ => 2u8,  // Max Ghost
        };
        
        // Risk class
        let risk_class = if input.risk_level == 0 && input.volatility_score < 5000 {
            0u8  // Low risk
        } else if input.risk_level == 2 || input.volatility_score > 8000 {
            2u8  // High risk
        } else {
            1u8  // Balanced
        };
        
        let plan = PlanOutput {
            recommended_size_sol: recommended,
            slice_count,
            time_window_sec: time_window,
            mev_route,
            privacy_mode,
            risk_class,
        };

        intent.owner.from_arcis(plan)
    }
}

