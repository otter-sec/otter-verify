use instruction::OtterVerifyInstruction;
use processor::Processor;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub mod instruction;
pub mod processor;
pub mod state;
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Getting PDA Bump from instruction data
    let instruction = OtterVerifyInstruction::unpack(instruction_data)?;

    match instruction {
        OtterVerifyInstruction::CreateAccount(bump, params) => {
            Processor::process_create_account(program_id, accounts, bump, params)?
        }
        OtterVerifyInstruction::UpdateAccount(bump, params) => {
            Processor::process_update_account(program_id, accounts, bump, params)?
        }
    }

    Ok(())
}
