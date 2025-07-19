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
        MyStateV1, MyStateV2,
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
pub struct InitializeMyStateV1IxData {
    pub owner: Pubkey,
    pub data: [u8; 32],
}

impl DataLen for InitializeMyStateV1IxData {
    const LEN: usize = core::mem::size_of::<InitializeMyStateV1IxData>();
}

impl DataLen for InitializeMyStateV2IxData {
    const LEN: usize = core::mem::size_of::<InitializeMyStateV2IxData>();
}


pub fn process_initialize_state_v1(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
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

    let ix_data = unsafe { load_ix_data::<InitializeMyStateV1IxData>(data)? };

    if ix_data.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let seeds = &[MyStateV1::SEED.as_bytes(), &ix_data.owner];
    // derive the canonical bump during account init
    let (derived_my_state_pda, bump) = pubkey::find_program_address(seeds, &crate::ID);
    if derived_my_state_pda.ne(state_acc.key()) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    let bump_binding = [bump];
    //Signer Seeds
    let signer_seeds = [
        Seed::from(MyStateV1::SEED.as_bytes()),
        Seed::from(&ix_data.owner),
        Seed::from(&bump_binding),
    ];
    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer_acc,
        to: state_acc,
        space: MyStateV1::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(MyStateV1::LEN),
    }
    .invoke_signed(&signers)?;

    MyStateV1::initialize(state_acc, &ix_data, bump)?;

    Ok(())
}


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
    // Using bytemuck initialization
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

    
    MyStateV2::initialize(state_acc, &ix_data, bump)?;

    Ok(())
}
