use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use solana_security_txt::security_txt;

declare_id!("72hPR1CB4gmUjUyBFBBdsvCcETAEn965kmZUm9sakrxN");

const PDA_SEED: &[u8] = b"otter_verify";

security_txt! {
  name: "Otter Verify",
  project_url: "https://osec.io/",
  contacts: "email:contact@osec.io",
  preferred_languages: "en",
  source_code: "https://github.com/otter-sec/otter-verify",
  policy: "https://github.com/otter-sec/otter-verify/SECURITY.md"
}

#[program]
pub mod otter_verify {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InputParams) -> Result<()> {
        let otter_verify = &mut ctx.accounts.build_params;
        otter_verify.git_url = params.git_url;
        otter_verify.commit = params.commit;
        otter_verify.command = params.command;
        otter_verify.bump = ctx.bumps.build_params;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, params: InputParams) -> Result<()> {
        let otter_verify = &mut ctx.accounts.build_params;
        otter_verify.git_url = params.git_url;
        otter_verify.commit = params.commit;
        otter_verify.command = params.command;
        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }
}

fn calculate_space(input: &InputParams) -> usize {
    // 8 bytes for discriminator
    // 4 + len(git_url) for git_url
    // 4 + len(commit) for commit
    // 4 bytes for length of the vector
    // 4 + len bytes for each string in the vector
    // 1 byte for bump
    8 + 4
        + input.git_url.len()
        + 4
        + input.commit.len()
        + 4
        + input.command.iter().map(|x| 4 + x.len()).sum::<usize>()
        + 1
}

#[derive(Accounts)]
#[instruction(instruction_data: InputParams)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [PDA_SEED, authority.key().as_ref(), program_address.key().as_ref()],
        bump,
        payer = authority,
        space =  calculate_space(&instruction_data)
    )]
    pub build_params: Account<'info, BuildParams>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(executable)]
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
        seeds = [PDA_SEED, authority.key().as_ref(), program_address.key().as_ref()],
        bump = build_params.bump,
        realloc = calculate_space(&instruction_data),
        realloc::zero = false,
        realloc::payer=authority
    )]
    pub build_params: Account<'info, BuildParams>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(executable)]
    /// CHECK:
    pub program_address: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BuildParams {
    pub git_url: String,
    pub commit: String,
    pub command: Vec<String>,
    bump: u8,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(
        mut,
        seeds = [PDA_SEED, authority.key().as_ref(), program_address.key().as_ref()],
        bump,
        close = authority
    )]
    pub data_account: Account<'info, BuildParams>,
    #[account(executable)]
    /// CHECK:
    pub program_address: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct InputParams {
    pub git_url: String,
    pub commit: String,
    pub command: Vec<String>,
}
