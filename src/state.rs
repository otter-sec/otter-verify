use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Eq, Debug, Clone)]
pub struct BuildParams {
    /// URL to the git repository
    pub repository_url: String,
    /// commit hash
    pub commit: String,
    /// Path to mount to the docker image
    pub mount_directory: Option<String>,
    /// Which binary file to build (applies to repositories with multiple programs)
    pub library_name: Option<String>,
    /// Optionally specify a custom base docker image to use for building the program repository
    pub base_image: Option<String>,
    /// If the program requires cargo build-bpf (instead of cargo build-sbf), as for anchor program, set this flag
    pub bpf: bool,
    /// Arguments to pass to the underlying `cargo build-bpf` command
    pub cargo_args: Option<Vec<String>>,
}

impl BuildParams {
    pub const MAX_SIZE: usize = 8 + std::mem::size_of::<BuildParams>();

    // Note: 8 Bytes are reserved for the size of the data length
    pub fn pack(&self, buffer: &mut [u8]) -> ProgramResult {
        let buffer_length = buffer.len();
        // Check if the buffer is large enough to hold the data which is to be packed
        let data = self.try_to_vec().unwrap();
        let data_length = data.len();
        if buffer_length < data_length + 8 {
            return Err(ProgramError::AccountDataTooSmall);
        }

        // Copy the data length into the buffer
        buffer[..8].copy_from_slice(&data_length.to_le_bytes());
        // Copy the data into the buffer
        buffer[8..=data_length + 8].copy_from_slice(&data);
        Ok(())
    }

    pub fn unpack(buffer: &[u8]) -> Result<Self, ProgramError> {
        // First 8 bytes are the length of the data
        let data_length = usize::from_le_bytes(buffer[..8].try_into().unwrap());
        let data = buffer
            .iter()
            .skip(8)
            .take(data_length)
            .copied()
            .collect::<Vec<u8>>();
        let build_params: BuildParams = BuildParams::try_from_slice(&data).expect("Invalid data");
        Ok(build_params)
    }

    pub fn save(&self, account: &AccountInfo) -> ProgramResult {
        let data = self.try_to_vec().map_err(|error| {
            msg!("Error: {}", error);
            ProgramError::InvalidAccountData
        })?;
        let data_length = data.len();
        let buffer = &mut account.data.borrow_mut();
        if buffer.len() < 8 + data_length {
            return Err(ProgramError::AccountDataTooSmall);
        }
        buffer[..8].copy_from_slice(&data_length.to_le_bytes());
        buffer[8..data_length + 8].copy_from_slice(&data);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Test the serialization and deserialization of the BuildParams struct
    #[test]
    fn test_build_params_serialization() {
        let build_params = BuildParams {
            repository_url: "https://github.com/Ellipsis-Labs/phoenix-v1".to_string(),
            commit: "dac82fe".to_string(),
            mount_directory: None,
            library_name: None,
            base_image: None,
            bpf: false,
            cargo_args: None,
        };
        let serialized = build_params.try_to_vec().unwrap();
        let deserialized = BuildParams::try_from_slice(&serialized).unwrap();
        assert_eq!(build_params, deserialized);
    }
}
