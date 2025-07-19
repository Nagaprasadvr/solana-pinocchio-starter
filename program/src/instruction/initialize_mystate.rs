use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::rent::Rent,
    ProgramResult,
};

use bytemuck::{Pod, Zeroable};

use pinocchio_system::instructions::CreateAccount;

use crate::{
    error::MyProgramError,
    state::{
        utils::{load_ix_data, DataLen},
        MyState, MyStateV2,
    },
};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct InitializeMyStateV2IxData {
    pub owner: Pubkey,
    pub data: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct InitializeMyStateIxData {
    pub owner: Pubkey,
    pub data: [u8; 32],
}

impl DataLen for InitializeMyStateIxData {
    const LEN: usize = core::mem::size_of::<InitializeMyStateIxData>();
}

impl DataLen for InitializeMyStateV2IxData {
    const LEN: usize = core::mem::size_of::<InitializeMyStateV2IxData>();
}

// Fixed typo: process_initilaize_state -> process_initialize_state
pub fn process_initialize_state(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer_acc, state_acc, sysvar_rent_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !state_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data = unsafe { load_ix_data::<InitializeMyStateIxData>(data)? };

    if ix_data.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let seeds = &[MyState::SEED.as_bytes(), &ix_data.owner];

    let (derived_my_state_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_my_state_pda.ne(state_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];

    let signer_seeds = [
        Seed::from(MyState::SEED.as_bytes()),
        Seed::from(&ix_data.owner),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer_acc,
        to: state_acc,
        space: MyState::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(MyState::LEN),
    }
    .invoke_signed(&signers)?;

    MyState::initialize(state_acc, &ix_data, bump)?;

    Ok(())
}

// Fixed typo: process_initilaize_stateV2 -> process_initialize_state_v2
pub fn process_initialize_state_v2(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer_acc, state_acc, sysvar_rent_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !state_acc.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let rent = Rent::from_account_info(sysvar_rent_acc)?;

    let ix_data =
        bytemuck::from_bytes::<InitializeMyStateV2IxData>(&data[..InitializeMyStateV2IxData::LEN]);

    if ix_data.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let seeds = &[MyStateV2::SEED.as_bytes(), &ix_data.owner];

    let (derived_my_state_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_my_state_pda.ne(state_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];

    let signer_seeds = [
        Seed::from(MyStateV2::SEED.as_bytes()),
        Seed::from(&ix_data.owner),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer_acc,
        to: state_acc,
        space: MyStateV2::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(MyStateV2::LEN),
    }
    .invoke_signed(&signers)?;

    // Using bytemuck initialization
    MyStateV2::initialize(state_acc, &ix_data, bump)?;

    Ok(())
}
