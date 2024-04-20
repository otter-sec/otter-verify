use borsh::BorshSerialize;
use otter_verify::state::BuildParams;

pub fn create_instruction_data(
    ix_tag: u8,
    pda_bump: u8,
    params: &BuildParams,
) -> [u8; BuildParams::MAX_SIZE + 2] {
    let serialized = params.try_to_vec().unwrap();

    // Create a Vec with MAX_SIZE of BuildParams and 2 extra bytes for the tag and bump
    let mut buffer = [0; BuildParams::MAX_SIZE + 2];
    buffer[0] = ix_tag; // Tag
    buffer[1] = pda_bump; // Bump
    let data_length = serialized.len();
    buffer[2..10].copy_from_slice(&data_length.to_le_bytes());
    // Copy the data into the buffer
    buffer[10..=data_length + 9].copy_from_slice(&serialized);
    buffer
}
