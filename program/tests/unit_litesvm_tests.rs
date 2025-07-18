use litesvm::types::{FailedTransactionMetadata, TransactionMetadata};
use litesvm::LiteSVM;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::v0::Message;
use solana_sdk::message::VersionedMessage;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;
extern crate alloc;
use alloc::vec;

use solana_pinocchio_starter::instruction::{InitializeMyStateIxData, UpdateMyStateIxData};
use solana_pinocchio_starter::state::{to_bytes, MyState};
use solana_pinocchio_starter::ID;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

#[test]
fn test_initialize_mystate() {
    let (mut svm, fee_payer) = setup_test();

    // Create the PDA
    let (mystate_pda, _bump) = Pubkey::find_program_address(
        &[MyState::SEED.as_bytes(), &fee_payer.pubkey().to_bytes()],
        &PROGRAM,
    );
    let ix_accounts = vec![
        AccountMeta::new(fee_payer.pubkey(), true),
        AccountMeta::new(mystate_pda, false),
        AccountMeta::new_readonly(RENT, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let ix_data = InitializeMyStateIxData {
        owner: fee_payer.pubkey().to_bytes(),
        data: [1; 32],
    };
    let mut ser_ix_data = vec![0];
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    let res = send_transaction(&mut svm, &fee_payer, ix_accounts, ser_ix_data);
    assert!(res.is_ok());

    let transaction_metadata = res.unwrap();
    println!(
        "test::init cu: {}",
        transaction_metadata.compute_units_consumed
    );
}

#[test]
fn test_update_mystate() {
    let (mut svm, fee_payer) = setup_test();

    // Create the PDA
    let (mystate_pda, _bump) = Pubkey::find_program_address(
        &[MyState::SEED.as_bytes(), &fee_payer.pubkey().to_bytes()],
        &PROGRAM,
    );
    let ix_accounts = vec![
        AccountMeta::new(fee_payer.pubkey(), true),
        AccountMeta::new(mystate_pda, false),
        AccountMeta::new_readonly(RENT, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    let ix_data = InitializeMyStateIxData {
        owner: fee_payer.pubkey().to_bytes(),
        data: [1; 32],
    };
    let mut ser_ix_data = vec![0];
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    let res = send_transaction(&mut svm, &fee_payer, ix_accounts, ser_ix_data);
    assert!(res.is_ok());
    let transaction_metadata = res.unwrap();
    println!(
        "test::update  init cu: {}",
        transaction_metadata.compute_units_consumed
    );

    // Update the mystate

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(fee_payer.pubkey(), true),
        AccountMeta::new(mystate_pda, false),
    ];

    // Create the instruction data
    let ix_data = UpdateMyStateIxData { data: [1; 32] };

    // Ix discriminator = 1
    let mut ser_ix_data = vec![1];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    let instruction = Instruction {
        program_id: PROGRAM,
        accounts: ix_accounts,
        data: ser_ix_data,
    };

    // Create tx_accounts vec
    let msg = Message::try_compile(
        &fee_payer.pubkey(),
        &[instruction],
        &[],
        svm.latest_blockhash(),
    )
    .unwrap();

    let tx = VersionedTransaction::try_new(VersionedMessage::V0(msg), &[fee_payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());

    let transaction_metadata = res.unwrap();
    println!(
        "test::update  update cu: {}",
        transaction_metadata.compute_units_consumed
    );
}

fn setup_test() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();

    // Add the program to the LiteSVM
    svm.add_program_from_file(PROGRAM, "target/deploy/solana_pinocchio_starter.so")
        .unwrap();

    // Defining constant fee_payer for testing to avoid bump mismatch between test cases
    let fee_payer = Keypair::from_bytes(&[
        130, 101, 234, 182, 1, 73, 82, 145, 145, 214, 14, 229, 210, 123, 80, 217, 218, 234, 148,
        128, 239, 202, 246, 85, 203, 247, 7, 222, 58, 216, 205, 29, 225, 151, 189, 79, 127, 69, 8,
        90, 127, 127, 167, 85, 46, 142, 92, 26, 85, 202, 112, 113, 112, 21, 225, 159, 88, 106, 13,
        26, 179, 125, 169, 194,
    ])
    .unwrap();

    svm.airdrop(&fee_payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .unwrap();
    (svm, fee_payer)
}

fn send_transaction(
    svm: &mut LiteSVM,
    fee_payer: &Keypair,
    ix_accounts: Vec<AccountMeta>,
    ser_ix_data: Vec<u8>,
) -> Result<TransactionMetadata, FailedTransactionMetadata> {
    // Create instruction
    let instruction = Instruction {
        program_id: PROGRAM,
        accounts: ix_accounts,
        data: ser_ix_data,
    };

    // Create tx_accounts vec
    let msg = Message::try_compile(
        &fee_payer.pubkey(),
        &[instruction],
        &[],
        svm.latest_blockhash(),
    )
    .unwrap();

    let tx = VersionedTransaction::try_new(VersionedMessage::V0(msg), &[fee_payer]).unwrap();

    svm.send_transaction(tx)
}
