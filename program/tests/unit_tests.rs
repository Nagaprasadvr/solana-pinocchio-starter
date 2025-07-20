use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use solana_pinocchio_starter::instruction::{InitializeMyStateV1IxData, UpdateMyStateV1IxData};
use solana_pinocchio_starter::instruction::{InitializeMyStateV2IxData,UpdateMyStateV2IxData};
use solana_pinocchio_starter::state::{to_bytes, DataLen, MyStateV1,MyStateV2};
use solana_pinocchio_starter::ID;
use solana_sdk::rent::Rent;
use solana_sdk::sysvar::Sysvar;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/solana_pinocchio_starter");
    mollusk
}

pub fn get_rent_data() -> Vec<u8> {
    let rent = Rent::default();
    unsafe {
        core::slice::from_raw_parts(&rent as *const Rent as *const u8, Rent::size_of()).to_vec()
    }
}

#[test]
fn test_initialize_mystate() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    let (mystate_pda, _bump) =
        Pubkey::find_program_address(&[MyStateV2::SEED.as_bytes(), &PAYER.to_bytes()], &PROGRAM);

    //Initialize the accounts
    let payer_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let mystate_account = Account::new(0, 0, &system_program);
    let min_balance = mollusk.sysvars.rent.minimum_balance(Rent::size_of());
    let mut rent_account = Account::new(min_balance, Rent::size_of(), &RENT);
    rent_account.data = get_rent_data();

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(mystate_pda, false),
        AccountMeta::new_readonly(RENT, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // Create the instruction data
    let ix_data = InitializeMyStateV2IxData {
        owner: *PAYER.as_array(),
        data: [1; 32],
    };

    // Ix discriminator = 0
    let mut ser_ix_data = vec![0];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    // Create instruction
    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

    // Create tx_accounts vec
    let tx_accounts = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda, mystate_account.clone()),
        (RENT, rent_account.clone()),
        (system_program, system_account.clone()),
    ];

    let init_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(init_res.program_result == ProgramResult::Success);
}

#[test]
fn test_update_mystate() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, _system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    let (mystate_pda, bump) =
        Pubkey::find_program_address(&[MyStateV2::SEED.as_bytes(), &PAYER.to_bytes()], &PROGRAM);

    //Initialize the accounts
    let payer_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let rent = mollusk.sysvars.rent.minimum_balance(MyStateV2::LEN);

    let mut mystate_account = Account::new(rent, MyStateV2::LEN, &ID.into());

    let my_state = MyStateV2 {
        is_initialized: 1,
        owner: *PAYER.as_array(),
        state: 1,
        data: [1; 32],
        update_count: 0,
        bump,
        _padding: 1
    };

    mystate_account.data = unsafe { to_bytes(&my_state).to_vec() };

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(mystate_pda, false),
    ];

    // Create the instruction data
    let ix_data = UpdateMyStateV2IxData { data: [1; 32] };

    // Ix discriminator = 1
    let mut ser_ix_data = vec![1];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    // Create instruction
    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);
    // Create tx_accounts vec
    let tx_accounts = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda, mystate_account.clone()),
    ];

    let update_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(update_res.program_result == ProgramResult::Success);
}
