use arcis_imports::*;

/// Confidential risk scoring for Evalys trades
/// 
/// Evaluates trade risk using encrypted user portfolio context and history,
/// combined with public market conditions.
#[encrypted]
mod circuits {
    use arcis_imports::*;

    /// User portfolio context (encrypted)
    pub struct PortfolioContext {
        pub total_capital: u64,          // Total portfolio value
        pub current_exposure: u64,        // Current exposure in trades
        pub diversification_score: u8,    // How diversified (0-255)
        pub leverage_ratio: u16,         // Leverage ratio in basis points
    }

    /// User performance history (encrypted)
    pub struct PerformanceHistory {
        pub total_pnl: i64,              // Total PnL (can be negative)
        pub sharpe_ratio: i16,            // Sharpe ratio (scaled by 100)
        pub max_drawdown: u16,            // Max drawdown in basis points
        pub consistency_score: u8,         // Consistency: 0-255
    }

    /// Market conditions (plaintext, from on-chain)
    pub struct MarketConditions {
        pub curve_volatility: u16,        // Curve volatility
        pub liquidity_risk: u8,           // Liquidity risk: 0-255
        pub market_sentiment: i8,          // Market sentiment: -128 to 127
    }

    /// Risk assessment output (encrypted)
    pub struct RiskAssessment {
        pub overall_risk_score: u8,      // Overall risk: 0-255
        pub portfolio_risk: u8,           // Portfolio-specific risk
        pub trade_risk: u8,               // Trade-specific risk
        pub recommendation: u8,           // 0=proceed, 1=caution, 2=avoid
    }

    #[instruction]
    pub fn confidential_risk_score(
        portfolio: Enc<Shared, PortfolioContext>,
        performance: Enc<Shared, PerformanceHistory>,
        market: MarketConditions,
    ) -> Enc<Shared, RiskAssessment> {
        let port = portfolio.to_arcis();
        let perf = performance.to_arcis();

        // Portfolio risk: based on exposure and diversification
        let exposure_ratio = if port.total_capital > 0 {
            (port.current_exposure * 255) / port.total_capital
        } else {
            255u64 // No capital = max risk
        };
        
        let portfolio_risk = if exposure_ratio > 200 {
            255u8 // Over-exposed
        } else if exposure_ratio > 150 {
            200u8 // High exposure
        } else if port.diversification_score < 100 {
            (150u64 + (100u64 - port.diversification_score as u64)) as u8 // Low diversification
        } else {
            100u8 // Reasonable exposure
        };

        // Trade risk: based on market conditions
        let trade_risk = if market.curve_volatility > 500 {
            255u8 // Very high volatility
        } else if market.curve_volatility > 300 {
            200u8 // High volatility
        } else if market.liquidity_risk > 200 {
            180u8 // High liquidity risk
        } else {
            100u8 // Moderate risk
        };

        // Performance-based risk adjustment
        let performance_adjustment = if perf.total_pnl < 0 && perf.max_drawdown > 5000 {
            +50i16 // Poor performance increases risk
        } else if perf.sharpe_ratio > 100 && perf.consistency_score > 200 {
            -30i16 // Good performance reduces risk
        } else {
            0i16
        };

        // Overall risk score
        let base_risk = ((portfolio_risk as u16 + trade_risk as u16) / 2) as i16;
        let overall_risk = (base_risk + performance_adjustment).clamp(0, 255) as u8;

        // Recommendation
        let recommendation = if overall_risk > 200 {
            2u8 // Avoid
        } else if overall_risk > 150 {
            1u8 // Caution
        } else {
            0u8 // Proceed
        };

        let assessment = RiskAssessment {
            overall_risk_score: overall_risk,
            portfolio_risk,
            trade_risk,
            recommendation,
        };

        portfolio.owner.from_arcis(assessment)
    }
}

