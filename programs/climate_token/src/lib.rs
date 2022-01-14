use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod climate_token {

    use super::*;

    pub fn submit_climate_data(ctx: Context<SubmitClimateData>,
        seed: u8, data: u128, collateral: u8) -> ProgramResult {
        instructions::submit_climate_data::handler(ctx, seed, data, collateral)
    }

    pub fn review_climate_data(ctx: Context<Review>) -> ProgramResult {
        Ok(())
    }

    pub fn report_climate_data(ctx: Context<Report>) -> ProgramResult {
        Ok(())
    }

    pub fn distribute_collateral(ctx: Context<DistributeCollateral>) -> ProgramResult {
        Ok(())
    }

    pub fn initialize_make_whole_account(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
pub struct Review {}

#[derive(Accounts)]
pub struct Report {}

#[derive(Accounts)]
pub struct DistributeCollateral {}