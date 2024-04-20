use std::str::FromStr;

use otter_verify::state::BuildParams;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    system_program,
    transaction::Transaction,
};
mod utils;

#[tokio::test]
async fn test_upload_params() {
    // Let it be the program_id of the program that will be deployed
    let program_id = Pubkey::from_str("HvXzmbnVADibsqVCPFdESEyxMyEo9BFBFtoRuXZndgJ2").unwrap();

    // Program Addess to create PDA for
    let other_program_address =
        Pubkey::from_str("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY").unwrap();

    let program_test = ProgramTest::new("otter_verify", program_id, None);

    let (mut banks_client, payer, _) = program_test.start().await;

    // PDA Account
    let seeds: &[&[u8]; 3] = &[
        b"otter_verify",
        &other_program_address.to_bytes(),
        &payer.pubkey().to_bytes(),
    ];

    let (pda_account, pda_bump) = Pubkey::find_program_address(seeds, &program_id);

    let build_params = BuildParams {
        repository_url: "https://github.com/Ellipsis-Labs/phoenix-v1".to_string(),
        commit: "dac82fe".to_string(),
        mount_directory: None,
        library_name: None,
        base_image: None,
        bpf: false,
        cargo_args: None,
    };

    let ix_data = utils::create_instruction_data(0, pda_bump, &build_params);

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &ix_data,
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(other_program_address, false),
                AccountMeta::new(pda_account, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    let last_blockhash = banks_client.get_latest_blockhash().await.unwrap();

    transaction.sign(&[&payer], last_blockhash);

    banks_client
        .process_transaction(transaction)
        .await
        .expect("Transaction failed");

    // get the pda account and check if the data is as expected
    let account = banks_client
        .get_account(pda_account)
        .await
        .expect("get_account failed")
        .expect("pda not found");

    let pda_build_params = BuildParams::unpack(&account.data).unwrap();
    println!("{:?}", pda_build_params);
    assert!(pda_build_params.repository_url == build_params.repository_url);

    // // Update the account
    let new_build_params = BuildParams {
        repository_url: "Ellipsis-Labs/phoenix-v1".to_string(),
        commit: "dac82fe".to_string(),
        mount_directory: None,
        library_name: None,
        base_image: None,
        bpf: true,
        cargo_args: None,
    };
    let new_ix_data = utils::create_instruction_data(1, pda_bump, &new_build_params);

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &new_ix_data,
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(other_program_address, false),
                AccountMeta::new(pda_account, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    let last_blockhash = banks_client.get_latest_blockhash().await.unwrap();

    transaction.sign(&[&payer], last_blockhash);

    banks_client
        .process_transaction(transaction)
        .await
        .expect("Transaction failed");

    // get the pda account and check if the data is as expected
    let account = banks_client
        .get_account(pda_account)
        .await
        .expect("get_account failed")
        .expect("pda not found");
    let pda_build_params = BuildParams::unpack(&account.data).unwrap();
    println!("{:?}", pda_build_params);
    assert!(pda_build_params.bpf);
}
