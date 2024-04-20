use crate::state::BuildParams;
use solana_program::program::invoke_signed;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
};

pub struct Processor;
impl Processor {
    pub fn process_create_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        pda_bump: u8,
        params: BuildParams,
    ) -> ProgramResult {
        msg!("Processing CreateAccount");
        // get accounts using iterator
        let accounts_iter = &mut accounts.iter();

        let signer = next_account_info(accounts_iter)?;
        let system_program_info = next_account_info(accounts_iter)?;
        let program_address = next_account_info(accounts_iter)?;
        let pda_account = next_account_info(accounts_iter)?;

        if !signer.is_signer {
            msg!("Missig a required signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let seeds: &[&[u8]; 4] = &[
            b"otter_verify",
            &program_address.key.to_bytes(),
            &signer.key.to_bytes(),
            &[pda_bump],
        ];

        let pda_generated = Pubkey::create_program_address(seeds, program_id)?;

        if pda_generated.ne(pda_account.key) {
            msg!(
                "Invalid PDA account provided {} != generated {}",
                pda_account.key,
                pda_generated
            );
            return Err(ProgramError::InvalidArgument);
        }

        // Calculate the required lamports for the account
        let space = BuildParams::MAX_SIZE;
        let required_lamports = Rent::get()?.minimum_balance(space).max(1);

        // Create PDA account
        invoke_signed(
            &system_instruction::create_account(
                signer.key,
                pda_account.key,
                required_lamports,
                space as u64,
                program_id,
            ),
            &[
                signer.clone(),
                pda_account.clone(),
                system_program_info.clone(),
            ],
            &[seeds],
        )?;
        msg!("Account created successfully: {:?}", pda_account.key);

        // Serialize the BuildParams struct and save it to the account
        params.save(pda_account)?;

        msg!("BuildParams saved successfully");
        Ok(())
    }

    pub fn process_update_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bump: u8,
        params: BuildParams,
    ) -> ProgramResult {
        msg!("Processing UpdateAccount");
        let accounts_iter = &mut accounts.iter();

        let signer = next_account_info(accounts_iter)?;
        let program_address = next_account_info(accounts_iter)?;
        let pda_account = next_account_info(accounts_iter)?;

        // Check if the account is signed
        if !signer.is_signer {
            msg!("Missig a required signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check if the account is owned by the program
        if pda_account.owner != program_id {
            msg!("Account is not owned by the program");
            return Err(ProgramError::IncorrectProgramId);
        }

        let seeds: &[&[u8]; 4] = &[
            b"otter_verify",
            &program_address.key.to_bytes(),
            &signer.key.to_bytes(),
            &[bump],
        ];

        let pda_generated = Pubkey::create_program_address(seeds, program_id)?;

        if pda_generated.ne(pda_account.key) {
            msg!(
                "Invalid PDA account provided {} != generated {}",
                pda_account.key,
                pda_generated
            );
            return Err(ProgramError::InvalidArgument);
        }

        // Serialize the BuildParams struct and save it to the account
        params.save(pda_account)?;

        msg!("BuildParams updated successfully");
        Ok(())
    }
}
