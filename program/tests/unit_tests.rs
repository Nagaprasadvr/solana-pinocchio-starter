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
use solana_pinocchio_starter::instruction::{InitializeMyStateV2IxData, UpdateMyStateV2IxData};
use solana_pinocchio_starter::state::{to_bytes, DataLen, MyStateV1, MyStateV2, State};
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
    let (mystate_pda_v2, _bump) =
        Pubkey::find_program_address(&[MyStateV2::SEED.as_bytes(), &PAYER.to_bytes()], &PROGRAM);

    let (mystate_pda_v1, _bump) =
        Pubkey::find_program_address(&[MyStateV1::SEED.as_bytes(), &PAYER.to_bytes()], &PROGRAM);

    //Initialize the accounts
    let payer_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let mystate_account_v1 = Account::new(0, 0, &system_program);
    let mystate_account_v2 = Account::new(0, 0, &system_program);
    let min_balance = mollusk.sysvars.rent.minimum_balance(Rent::size_of());
    let mut rent_account = Account::new(min_balance, Rent::size_of(), &RENT);
    rent_account.data = get_rent_data();

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts_v2 = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(mystate_pda_v2, false),
        AccountMeta::new_readonly(RENT, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let ix_accounts_v1 = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(mystate_pda_v1, false),
        AccountMeta::new_readonly(RENT, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // Create the instruction data

    let ix_data_v1 = InitializeMyStateV1IxData {
        owner: *PAYER.as_array(),
        data: [1; 32],
    };
    let ix_data_v2 = InitializeMyStateV2IxData {
        owner: *PAYER.as_array(),
        data: [1; 32],
    };

    // Ix discriminator = 0 and 1 for both init v1 and v2
    let mut ser_ix_data_v1 = vec![0];
    let mut ser_ix_data_v2 = vec![1];

    // Serialize the instruction data
    ser_ix_data_v1.extend_from_slice(unsafe { to_bytes(&ix_data_v1) });
    ser_ix_data_v2.extend_from_slice(unsafe { to_bytes(&ix_data_v2) });

    // Create instruction
    let instruction_v1 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data_v1, ix_accounts_v1);
    let instruction_v2 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data_v2, ix_accounts_v2);

    // Create tx_accounts vec

    let tx_accounts_v1 = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda_v1, mystate_account_v1.clone()),
        (RENT, rent_account.clone()),
        (system_program, system_account.clone()),
    ];
    let tx_accounts_v2 = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda_v2, mystate_account_v2.clone()),
        (RENT, rent_account.clone()),
        (system_program, system_account.clone()),
    ];

    let init_res_v1 = mollusk.process_and_validate_instruction(
        &instruction_v1,
        tx_accounts_v1,
        &[Check::success()],
    );
    let init_res_v2 = mollusk.process_and_validate_instruction(
        &instruction_v2,
        tx_accounts_v2,
        &[Check::success()],
    );

    assert!(init_res_v1.program_result == ProgramResult::Success);
    assert!(init_res_v2.program_result == ProgramResult::Success);
}

#[test]
fn test_update_mystate() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, _system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    let (mystate_pda_v1, bump_v1) =
        Pubkey::find_program_address(&[MyStateV1::SEED.as_bytes(), &PAYER.to_bytes()], &PROGRAM);
    let (mystate_pda_v2, bump_v2) =
        Pubkey::find_program_address(&[MyStateV2::SEED.as_bytes(), &PAYER.to_bytes()], &PROGRAM);

    //Initialize the accounts
    let payer_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let rent = mollusk.sysvars.rent.minimum_balance(MyStateV2::LEN);

    let mut mystate_account_v1 = Account::new(rent, MyStateV1::LEN, &ID.into());
    let mut mystate_account_v2 = Account::new(rent, MyStateV2::LEN, &ID.into());

    let my_state_v1 = MyStateV1 {
        is_initialized: 1,
        owner: *PAYER.as_array(),
        state: State::Initialized,
        data: [1; 32],
        update_count: 0,
        bump: bump_v1,
    };
    let my_state_v2 = MyStateV2 {
        is_initialized: 1,
        owner: *PAYER.as_array(),
        state: 1,
        data: [1; 32],
        update_count: 0,
        bump: bump_v2,
        _padding: 1,
    };

    mystate_account_v1.data = unsafe { to_bytes(&my_state_v1).to_vec() };
    mystate_account_v2.data = unsafe { to_bytes(&my_state_v2).to_vec() };

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts_v1 = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(mystate_pda_v1, false),
    ];
    let ix_accounts_v2 = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(mystate_pda_v2, false),
    ];

    // Create the instruction data
    let ix_data_v1 = UpdateMyStateV1IxData { data: [1; 32] };
    let ix_data_v2 = UpdateMyStateV2IxData { data: [1; 32] };

    // Ix discriminator = 2, 3 for update v2 and v3 respectively
    let mut ser_ix_data_v1 = vec![2];
    let mut ser_ix_data_v2 = vec![3];

    // Serialize the instruction data
    ser_ix_data_v1.extend_from_slice(unsafe { to_bytes(&ix_data_v1) });
    ser_ix_data_v2.extend_from_slice(unsafe { to_bytes(&ix_data_v2) });

    // Create instruction
    let instruction_v1 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data_v1, ix_accounts_v1);
    let instruction_v2 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data_v2, ix_accounts_v2);

    // Create tx_accounts vec
    let tx_accounts_v1 = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda_v1, mystate_account_v1.clone()),
    ];
    let tx_accounts_v2 = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda_v2, mystate_account_v2.clone()),
    ];

    let update_res_v1 = mollusk.process_and_validate_instruction(
        &instruction_v1,
        tx_accounts_v1,
        &[Check::success()],
    );
    let update_res_v2 = mollusk.process_and_validate_instruction(
        &instruction_v2,
        tx_accounts_v2,
        &[Check::success()],
    );

    assert!(update_res_v1.program_result == ProgramResult::Success);
    assert!(update_res_v2.program_result == ProgramResult::Success);
}
