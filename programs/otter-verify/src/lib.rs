use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("72hPR1CB4gmUjUyBFBBdsvCcETAEn965kmZUm9sakrxN");

const PDA_SEED: &[u8] = b"otter_verify";


#[program]
pub mod otter_verify {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InputParams) -> Result<()> {
        // Print space of the account
        msg!("Initialize otter-verify program");
        msg!("Space: {}", ctx.accounts.build_params.to_account_info().data_len());
        let otter_verify = &mut ctx.accounts.build_params;
        otter_verify.command = params.command;
        otter_verify.bump = ctx.bumps.build_params;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, params: InputParams) -> Result<()> {
        msg!("Update otter-verify program");
        msg!("Space: {}", ctx.accounts.build_params.to_account_info().data_len());
        let otter_verify = &mut ctx.accounts.build_params;
        otter_verify.command = params.command;
        Ok(())
    }
}


fn calculate_space(input: &[String]) -> usize {
    // 8 bytes for discriminator 
    // 4 bytes for length of the vector
    // 4 + len bytes for each string in the vector
    // 1 byte for bump
    8 + 4 + input.iter().map(|x| 4+x.len()).sum::<usize>() + 1
}

#[derive(Accounts)]
#[instruction(instruction_data: InputParams)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds = [PDA_SEED, authority.key().as_ref(), program_address.key().as_ref()],
        bump,
        payer = authority, 
        space =  calculate_space(&instruction_data.command) 
    )]
    pub build_params: Account<'info, BuildParams>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub program_address: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}




#[derive(Accounts)]
#[instruction(instruction_data: InputParams)]
pub struct Update<'info> {
    #[account(
        mut,
        realloc = calculate_space(&instruction_data.command),
        realloc::zero = false, 
        realloc::payer=authority
    )]
    pub build_params: Account<'info, BuildParams>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK:
    pub program_address: AccountInfo<'info>,
    #[account(address = system_program::ID)]
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
