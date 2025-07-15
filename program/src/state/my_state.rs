use super::utils::{DataLen, Initialized};
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::{
    error::MyProgramError,
    instruction::{InitializeMyStateIxData, UpdateMyStateIxData},
    state::try_from_account_info_mut,
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub enum State {
    Uninitialized,
    Initialized,
    Updated,
}

#[repr(C)] //keeps the struct layout the same across different architectures
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankAccount)]
pub struct MyState {
    pub is_initialized: u8,
    pub owner: Pubkey,
    pub state: State,
    pub data: [u8; 32],
    pub update_count: u32,
    pub bump: u8,
}

impl DataLen for MyState {
    const LEN: usize = core::mem::size_of::<MyState>();
}

impl Initialized for MyState {
    fn is_initialized(&self) -> bool {
        self.is_initialized > 0
    }
}

impl MyState {
    pub const SEED: &'static str = "mystate";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seeds = &[Self::SEED.as_bytes(), owner];
        let derived = pinocchio_pubkey::derive_address(seeds, bump, &crate::ID);
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }

    pub fn initialize(
        my_stata_acc: &AccountInfo,
        ix_data: &InitializeMyStateIxData,
        bump: u8,
    ) -> ProgramResult {
        let my_state = unsafe { try_from_account_info_mut::<MyState>(my_stata_acc) }?;

        my_state.owner = ix_data.owner;
        my_state.state = State::Initialized;
        my_state.data = ix_data.data;
        my_state.update_count = 0;
        my_state.bump = bump;
        my_state.is_initialized = 1;

        Ok(())
    }

    pub fn update(&mut self, ix_data: &UpdateMyStateIxData) -> ProgramResult {
        self.data = ix_data.data;
        if self.state != State::Updated {
            self.state = State::Updated;
        }
        self.update_count += 1;

        Ok(())
    }
}
