use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::{
    error::MyProgramError,
    state::{
        try_from_account_info_mut,
        utils::{load_ix_data, DataLen},
        Initialized, MyStateV1, MyStateV2,
    },
};

// V1 instruction data (custom serialization)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateMyStateV1IxData {
    pub data: [u8; 32],
}

// V2 instruction data (bytemuck serialization)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct UpdateMyStateV2IxData {
    pub data: [u8; 32],
}

impl DataLen for UpdateMyStateV1IxData {
    const LEN: usize = core::mem::size_of::<UpdateMyStateV1IxData>();
}

impl DataLen for UpdateMyStateV2IxData {
    const LEN: usize = core::mem::size_of::<UpdateMyStateV2IxData>();
}

// V1 instruction update function (custom serialization)
pub fn process_update_state_v1(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer_acc, state_acc] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let my_state = unsafe { try_from_account_info_mut::<MyStateV1>(state_acc)? };

    // CHECK if my_state is initialized
    if !my_state.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Validate PDA
    MyStateV1::validate_pda(my_state.bump, state_acc.key(), payer_acc.key())?;

    if my_state.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    let ix_data = unsafe { load_ix_data::<UpdateMyStateV1IxData>(data)? };

    my_state.update(&ix_data)?;

    Ok(())
}

// V2 update function (bytemuck serialization)
pub fn process_update_state_v2(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer_acc, state_acc] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Using bytemuck for instruction data deserialization
    let ix_data =
        bytemuck::from_bytes::<UpdateMyStateV2IxData>(&data[..UpdateMyStateV2IxData::LEN]);

    // Using bytemuck for account state deserialization
    let account_data = &mut state_acc.try_borrow_mut_data()?;
    let my_state = bytemuck::from_bytes_mut::<MyStateV2>(&mut account_data[..MyStateV2::LEN]);

    // CHECK if my_state is initialized
    if !my_state.is_initialized() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Validate PDA
    MyStateV2::validate_pda(my_state.bump, state_acc.key(), payer_acc.key())?;

    if my_state.owner.ne(payer_acc.key()) {
        return Err(MyProgramError::InvalidOwner.into());
    }

    my_state.update(&ix_data)?;

    Ok(())
}
