use mollusk_svm::{program, Mollusk};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_pinocchio_starter::{
    instruction::{InitializeMyStateIxData, UpdateMyStateIxData},
    state::{to_bytes, DataLen, MyState, State},
    ID,
};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    sysvar::Sysvar,
};
use solana_sdk::{pubkey, rent::Rent};

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

pub fn get_rent_data() -> Vec<u8> {
    let rent = Rent::default();
    unsafe {
        core::slice::from_raw_parts(&rent as *const Rent as *const u8, Rent::size_of()).to_vec()
    }
}

fn main() {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/solana_pinocchio_starter");

    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    let (mystate_pda, bump) =
        Pubkey::find_program_address(&[MyState::SEED.as_bytes(), &PAYER.to_bytes()], &PROGRAM);

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
    let ix_data = InitializeMyStateIxData {
        owner: *PAYER.as_array(),
        data: [1; 32],
    };

    // Ix discriminator = 0
    let mut ser_ix_data = vec![0];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    // Create instruction
    let instruction0 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

    // Create tx_accounts vec
    let tx_accounts0 = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda, mystate_account.clone()),
        (RENT, rent_account.clone()),
        (system_program, system_account.clone()),
    ];

    let rent = mollusk.sysvars.rent.minimum_balance(MyState::LEN);
    let mut mystate_account = Account::new(rent, MyState::LEN, &ID.into());

    let my_state = MyState {
        is_initialized: 1,
        owner: *PAYER.as_array(),
        state: State::Initialized,
        data: [1; 32],
        update_count: 0,
        bump,
    };

    mystate_account.data = unsafe { to_bytes(&my_state).to_vec() };

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(mystate_pda, false),
    ];

    // Create the instruction data
    let ix_data = UpdateMyStateIxData { data: [1; 32] };

    // Ix discriminator = 1
    let mut ser_ix_data = vec![1];

    // Serialize the instruction data
    ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

    // Create instruction
    let instruction1 = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);
    // Create tx_accounts vec
    let tx_accounts1 = &vec![
        (PAYER, payer_account.clone()),
        (mystate_pda, mystate_account.clone()),
    ];

    MolluskComputeUnitBencher::new(mollusk)
        .bench(("InitializeMyState", &instruction0, tx_accounts0))
        .bench(("UpdateMyState", &instruction1, tx_accounts1))
        .must_pass(true)
        .out_dir("benches/")
        .execute();
}
