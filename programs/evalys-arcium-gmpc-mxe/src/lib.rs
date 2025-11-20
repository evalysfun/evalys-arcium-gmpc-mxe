use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;

declare_id!("EVALYSARCIUMGMPCMXE11111111111111111111111");

#[arcium_program]
pub mod evalys_arcium_gmpc_mxe {
    use super::*;

    // Computation definition offsets for all encrypted instructions
    const COMP_DEF_OFFSET_STRATEGY: u32 = comp_def_offset("confidential_strategy_plan");
    const COMP_DEF_OFFSET_RISK: u32 = comp_def_offset("confidential_risk_score");
    const COMP_DEF_OFFSET_CURVE: u32 = comp_def_offset("confidential_curve_eval");
    const COMP_DEF_OFFSET_GMPC: u32 = comp_def_offset("evalys_gmpc_strategy");
    const COMP_DEF_OFFSET_MULTI_USER: u32 = comp_def_offset("confidential_multi_user_analytics");

    // ========== Confidential Strategy Plan ==========

    /// Initialize the strategy plan computation definition
    pub fn init_strategy_comp_def(ctx: Context<InitStrategyCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, COMP_DEF_OFFSET_STRATEGY, None, None)?;
        Ok(())
    }

    /// Invoke confidential strategy plan computation
    pub fn request_strategy_plan(
        ctx: Context<RequestStrategyPlan>,
        computation_offset: u64,
    ) -> Result<()> {
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            vec![], // Encrypted arguments passed from client
            None,
            vec![StrategyPlanCallback::callback_ix(&[])],
            1,
        )?;
        Ok(())
    }

    /// Callback for strategy plan computation
    #[arcium_callback(encrypted_ix = "confidential_strategy_plan")]
    pub fn strategy_plan_callback(
        ctx: Context<StrategyPlanCallback>,
        output: ComputationOutputs<ConfidentialStrategyPlanOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(ConfidentialStrategyPlanOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(StrategyPlanEvent {
            plan_id: ctx.accounts.computation_account.key(),
            encrypted_plan: result.ciphertexts[0],
            nonce: result.nonce.to_le_bytes(),
        });

        Ok(())
    }

    // ========== Confidential Risk Score ==========

    /// Initialize the risk score computation definition
    pub fn init_risk_comp_def(ctx: Context<InitRiskCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, COMP_DEF_OFFSET_RISK, None, None)?;
        Ok(())
    }

    /// Invoke confidential risk score computation
    pub fn request_risk_score(
        ctx: Context<RequestRiskScore>,
        computation_offset: u64,
    ) -> Result<()> {
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            vec![],
            None,
            vec![RiskScoreCallback::callback_ix(&[])],
            1,
        )?;
        Ok(())
    }

    /// Callback for risk score computation
    #[arcium_callback(encrypted_ix = "confidential_risk_score")]
    pub fn risk_score_callback(
        ctx: Context<RiskScoreCallback>,
        output: ComputationOutputs<ConfidentialRiskScoreOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(ConfidentialRiskScoreOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(RiskScoreEvent {
            assessment_id: ctx.accounts.computation_account.key(),
            encrypted_assessment: result.ciphertexts[0],
            nonce: result.nonce.to_le_bytes(),
        });

        Ok(())
    }

    // ========== Confidential Curve Evaluation ==========

    /// Initialize the curve eval computation definition
    pub fn init_curve_comp_def(ctx: Context<InitCurveCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, COMP_DEF_OFFSET_CURVE, None, None)?;
        Ok(())
    }

    /// Invoke confidential curve evaluation computation
    pub fn request_curve_eval(
        ctx: Context<RequestCurveEval>,
        computation_offset: u64,
    ) -> Result<()> {
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            vec![],
            None,
            vec![CurveEvalCallback::callback_ix(&[])],
            1,
        )?;
        Ok(())
    }

    /// Callback for curve evaluation computation
    #[arcium_callback(encrypted_ix = "confidential_curve_eval")]
    pub fn curve_eval_callback(
        ctx: Context<CurveEvalCallback>,
        output: ComputationOutputs<ConfidentialCurveEvalOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(ConfidentialCurveEvalOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(CurveEvalEvent {
            recommendation_id: ctx.accounts.computation_account.key(),
            encrypted_recommendation: result.ciphertexts[0],
            nonce: result.nonce.to_le_bytes(),
        });

        Ok(())
    }

    // ========== gMPC Strategy ==========

    /// Initialize the gMPC strategy computation definition
    pub fn init_gmpc_comp_def(ctx: Context<InitGmpcCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, COMP_DEF_OFFSET_GMPC, None, None)?;
        Ok(())
    }

    /// Invoke gMPC strategy computation
    pub fn request_gmpc_strategy(
        ctx: Context<RequestGmpcStrategy>,
        computation_offset: u64,
    ) -> Result<()> {
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            vec![],
            None,
            vec![GmpcStrategyCallback::callback_ix(&[])],
            1,
        )?;
        Ok(())
    }

    /// Callback for gMPC strategy computation
    #[arcium_callback(encrypted_ix = "evalys_gmpc_strategy")]
    pub fn gmpc_strategy_callback(
        ctx: Context<GmpcStrategyCallback>,
        output: ComputationOutputs<GmpcStrategyOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(GmpcStrategyOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(GmpcStrategyEvent {
            plan_id: ctx.accounts.computation_account.key(),
            encrypted_plan: result.ciphertexts[0],
            nonce: result.nonce.to_le_bytes(),
        });

        Ok(())
    }

    // ========== Multi-User Analytics ==========

    /// Initialize the multi-user analytics computation definition
    pub fn init_multi_user_comp_def(ctx: Context<InitMultiUserCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, COMP_DEF_OFFSET_MULTI_USER, None, None)?;
        Ok(())
    }

    /// Invoke multi-user analytics computation
    pub fn request_multi_user_analytics(
        ctx: Context<RequestMultiUserAnalytics>,
        computation_offset: u64,
    ) -> Result<()> {
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            vec![],
            None,
            vec![MultiUserAnalyticsCallback::callback_ix(&[])],
            1,
        )?;
        Ok(())
    }

    /// Callback for multi-user analytics computation
    #[arcium_callback(encrypted_ix = "confidential_multi_user_analytics")]
    pub fn multi_user_analytics_callback(
        ctx: Context<MultiUserAnalyticsCallback>,
        output: ComputationOutputs<MultiUserAnalyticsOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(MultiUserAnalyticsOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        emit!(MultiUserAnalyticsEvent {
            analytics_id: ctx.accounts.computation_account.key(),
            encrypted_metrics: result.ciphertexts[0],
            nonce: result.nonce.to_le_bytes(),
        });

        Ok(())
    }
}

// ========== Account Structures ==========

#[derive(Accounts)]
pub struct InitStrategyCompDef<'info> {
    // Account structure auto-generated by Arcium macros
}

#[derive(Accounts)]
pub struct RequestStrategyPlan<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Other required accounts auto-generated
}

#[derive(Accounts)]
pub struct StrategyPlanCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    // Other required accounts
}

#[derive(Accounts)]
pub struct InitRiskCompDef<'info> {}

#[derive(Accounts)]
pub struct RequestRiskScore<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct RiskScoreCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
}

#[derive(Accounts)]
pub struct InitCurveCompDef<'info> {}

#[derive(Accounts)]
pub struct RequestCurveEval<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CurveEvalCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
}

#[derive(Accounts)]
pub struct InitGmpcCompDef<'info> {}

#[derive(Accounts)]
pub struct RequestGmpcStrategy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GmpcStrategyCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
}

#[derive(Accounts)]
pub struct InitMultiUserCompDef<'info> {}

#[derive(Accounts)]
pub struct RequestMultiUserAnalytics<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct MultiUserAnalyticsCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
}

// ========== Events ==========

#[event]
pub struct StrategyPlanEvent {
    pub plan_id: Pubkey,
    pub encrypted_plan: [u8; 32],
    pub nonce: [u8; 16],
}

#[event]
pub struct RiskScoreEvent {
    pub assessment_id: Pubkey,
    pub encrypted_assessment: [u8; 32],
    pub nonce: [u8; 16],
}

#[event]
pub struct CurveEvalEvent {
    pub recommendation_id: Pubkey,
    pub encrypted_recommendation: [u8; 32],
    pub nonce: [u8; 16],
}

#[event]
pub struct GmpcStrategyEvent {
    pub plan_id: Pubkey,
    pub encrypted_plan: [u8; 32],
    pub nonce: [u8; 16],
}

#[event]
pub struct MultiUserAnalyticsEvent {
    pub analytics_id: Pubkey,
    pub encrypted_metrics: [u8; 32],
    pub nonce: [u8; 16],
}

// ========== Error Codes ==========

#[error_code]
pub enum ErrorCode {
    #[msg("Computation was aborted")]
    AbortedComputation,
}

