use anchor_lang::prelude::*;

declare_id!("72hPR1CB4gmUjUyBFBBdsvCcETAEn965kmZUm9sakrxN");

const PDA_SEED: &[u8] = b"otter_verify";
const COMMAND_SIZE: usize = 500;

#[program]
pub mod otter_verify {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InputParams) -> Result<()> {
        let otter_verify = &mut ctx.accounts.build_params;
        otter_verify.command = params.command;
        otter_verify.bump = ctx.bumps.build_params;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Space : 8 discriminator + 2 * 500 (command) + 1 bump
    #[account(
        init, 
        seeds = [PDA_SEED, authority.key().as_ref(), program_address.key().as_ref()],
        bump,
        payer = authority, 
        space = 8 + 2 * COMMAND_SIZE + 1, 
    )]
    pub build_params: Account<'info, BuildParams>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub program_address: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BuildParams {
    pub command: Vec<String>,
    bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct InputParams {
    pub command: Vec<String>,
}
