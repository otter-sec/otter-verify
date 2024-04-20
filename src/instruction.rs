use solana_program::program_error::ProgramError;

use crate::state::BuildParams;

#[derive(Debug)]
pub enum OtterVerifyInstruction {
    CreateAccount(u8, BuildParams),

    UpdateAccount(u8, BuildParams),
}

impl OtterVerifyInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => {
                let (&bump, data) = rest
                    .split_first()
                    .ok_or(ProgramError::InvalidInstructionData)?;
                let params = BuildParams::unpack(data)?;

                Ok(Self::CreateAccount(bump, params))
            }
            1 => {
                let (&bump, data) = rest
                    .split_first()
                    .ok_or(ProgramError::InvalidInstructionData)?;
                let params = BuildParams::unpack(data)?;
                Ok(Self::UpdateAccount(bump, params))
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
