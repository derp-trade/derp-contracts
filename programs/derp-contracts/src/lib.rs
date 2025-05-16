pub mod constants;
pub mod errors;
pub mod instructions;
pub mod math;
pub mod pyth;
pub mod state;

pub use crate::instructions::*;

use anchor_lang::prelude::*;

declare_id!("GZeLk7wqD1MUk2ELdMw4KskogNQsckoVqCTCbepxT1h3");

#[program]
pub mod derp_contracts {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        gold_pyth_account: Pubkey,
        sol_pyth_account: Pubkey,
        fartcoin_pyth_account: Pubkey,
    ) -> Result<()> {
        initialize_handler(
            ctx,
            gold_pyth_account,
            sol_pyth_account,
            fartcoin_pyth_account,
        )
    }

    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        create_handler(ctx)
    }

    pub fn open_position<'info>(
        mut ctx: Context<'_, '_, '_, 'info, OpenPosition<'info>>,
        asset_type: u8,
        size: i64,
        leverage: u8,
    ) -> Result<()> {
        // Apply any pending funding before opening a new position
        apply_funding_handler(&mut ctx)?;
        open_handler(ctx, asset_type, size, leverage)
    }

    pub fn close_position(mut ctx: Context<OpenPosition>, asset_type: u8) -> Result<()> {
        // Apply any pending funding before closing the position
        apply_funding_handler(&mut ctx)?;
        close_handler(ctx, asset_type)
    }

    pub fn calculate_funding(ctx: Context<CalculateFunding>) -> Result<()> {
        calculate_funding_handler(ctx)
    }

    pub fn apply_funding(mut ctx: Context<OpenPosition>) -> Result<()> {
        apply_funding_handler(&mut ctx)
    }
}
