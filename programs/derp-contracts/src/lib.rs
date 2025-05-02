use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod math;
pub mod pyth;
pub mod state;

declare_id!("GZeLk7wqD1MUk2ELdMw4KskogNQsckoVqCTCbepxT1h3");

#[program]
pub mod derp_contracts {
    use super::*;

    pub fn initialize(
        ctx: Context<instructions::initialize::Initialize>,
        gold_pyth_account: Pubkey,
        sol_pyth_account: Pubkey,
        fartcoin_pyth_account: Pubkey,
    ) -> Result<()> {
        instructions::initialize::handler(
            ctx,
            gold_pyth_account,
            sol_pyth_account,
            fartcoin_pyth_account,
        )
    }

    pub fn create_user_account(
        ctx: Context<instructions::user_account::CreateUserAccount>,
    ) -> Result<()> {
        instructions::user_account::create_handler(ctx)
    }

    pub fn open_position(
        ctx: Context<instructions::positions::OpenPosition>,
        asset_type: u8,
        size: i64,
        leverage: u8,
    ) -> Result<()> {
        // Apply any pending funding before opening a new position
        // instructions::positions::apply_funding_handler(Context::new(
        //     &ctx.program_id,
        //     &ctx.accounts,
        //     &ctx.remaining_accounts,
        //     ctx.bumps.clone(),
        // ))?;

        instructions::positions::open_handler(ctx, asset_type, size, leverage)
    }

    pub fn close_position(
        ctx: Context<instructions::positions::ClosePosition>,
        asset_type: u8,
    ) -> Result<()> {
        // Apply any pending funding before closing the position
        // instructions::positions::apply_funding_handler(Context::new(
        //     &ctx.program_id,
        //     &ctx.accounts,
        //     &ctx.remaining_accounts,
        //     ctx.bumps.clone(),
        // ))?;

        instructions::positions::close_handler(ctx, asset_type)
    }

    pub fn calculate_funding(ctx: Context<instructions::funding::CalculateFunding>) -> Result<()> {
        instructions::funding::handler(ctx)
    }

    pub fn apply_funding(ctx: Context<instructions::positions::ApplyFunding>) -> Result<()> {
        instructions::positions::apply_funding_handler(ctx)
    }
}
