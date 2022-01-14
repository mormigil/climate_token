use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(seed: u8)]
pub struct SubmitClimateData<'info> {
    #[account(signer)]
    pub climate_contributor: AccountInfo<'info>,

    #[account(
        init,
        seeds = [&[seed]],
        bump = seed,
        payer = climate_contributor,
        space = 8 + 32 + 32 + 8 + 1
    )]
    pub climate_pda: Account<'info, ClimateDataContract>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<SubmitClimateData>, seed: u8,
    data: u128, collateral: u8) -> ProgramResult {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let program_id = Pubkey::new_unique();

        let (vkey, vbump) = Pubkey::find_program_address(&[b"escrow"], &program_id);

        let (vkey2, vbump2) = Pubkey::find_program_address(&[b"escrow"], &program_id);

        println!("vkey is: {} and bump value is {}", &vkey, &vbump);

        println!("vkey is: {} and bump value is {}", &vkey2, &vbump2);
    }
}