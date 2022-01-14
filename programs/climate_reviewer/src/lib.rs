use anchor_lang::prelude::*;

declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[program]
pub mod climate_reviewer {
    use super::*;

    pub fn apply(ctx: Context<Application>) -> ProgramResult {
        Ok(())
    }

    pub fn review(ctx: Context<Review>) -> ProgramResult {
        Ok(())
    }

    pub fn endorse(ctx: Context<Endorse>) -> ProgramResult {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Application {}

#[derive(Accounts)]
pub struct Review {}


#[derive(Accounts)]
pub struct Endorse {}
