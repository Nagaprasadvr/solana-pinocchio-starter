use super::utils::{DataLen, Initialized};
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use bytemuck::{Pod,Zeroable};

use crate::{
    error::MyProgramError,
    instruction::{InitializeMyStateV1IxData, UpdateMyStateV1IxData},
    instruction::{InitializeMyStateV2IxData,UpdateMyStateV2IxData},
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
pub struct MyStateV1 {
    pub is_initialized: u8,
    pub owner: Pubkey,
    pub state: State,
    pub data: [u8; 32],
    pub update_count: u32,
    pub bump: u8,
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct MyStateV2 {
    pub owner: Pubkey,
    pub data: [u8; 32],
    pub update_count: u32,
    pub state: u8,
    pub is_initialized: u8,
    pub bump: u8,
    pub _padding: u8,
}

impl DataLen for MyStateV2 {
    const LEN: usize = core::mem::size_of::<MyStateV2>();
}

impl DataLen for MyStateV1 {
    const LEN: usize = core::mem::size_of::<MyStateV1>();
}

impl Initialized for MyStateV1 {
    fn is_initialized(&self) -> bool {
        self.is_initialized > 0
    }
}

impl Initialized for MyStateV2 {
    fn is_initialized(&self) -> bool {
        self.is_initialized > 0
    }
}

impl MyStateV1 {
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
        ix_data: &InitializeMyStateV1IxData,
        bump: u8,
    ) -> ProgramResult {
        let my_state = unsafe { try_from_account_info_mut::<MyStateV1>(my_stata_acc) }?;

        my_state.owner = ix_data.owner;
        my_state.state = State::Initialized;
        my_state.data = ix_data.data;
        my_state.update_count = 0;
        my_state.bump = bump;
        my_state.is_initialized = 1;

        Ok(())
    }

    pub fn update(&mut self, ix_data: &UpdateMyStateV1IxData) -> ProgramResult {
        self.data = ix_data.data;
        if self.state != State::Updated {
            self.state = State::Updated;
        }
        self.update_count += 1;

        Ok(())
    }
}



impl MyStateV2 {
    pub const SEED: &'static str = "mystatev2";

    //How to work without involving Enum (v1)
    pub fn get_state(&self) -> State {
        match self.state {
            0 => State::Uninitialized,
            1 => State::Initialized,
            2 => State::Updated,
            _ => State::Uninitialized // for fallback
        }
    }

    pub fn set_state(&mut self,state: State) {
        self.state = state as u8;
    }

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
        ix_data: &InitializeMyStateV2IxData,
        bump: u8,
    ) -> ProgramResult {
       //Using the Bytemuck for zero copy deserilization instead of unsafe block
       let account_data = &mut my_stata_acc.try_borrow_mut_data()?;
       let my_state = bytemuck::from_bytes_mut::<MyStateV2>(&mut account_data[..MyStateV2::LEN]);

        my_state.owner = ix_data.owner;
        my_state.set_state(State::Initialized);
        my_state.data = ix_data.data;
        my_state.update_count = 0;
        my_state.bump = bump;
        my_state.is_initialized = 1;

        Ok(())
    }

    pub fn update(&mut self, ix_data: &UpdateMyStateV2IxData) -> ProgramResult {
        self.data = ix_data.data;
        if self.get_state() != State::Updated {
            self.set_state(State::Updated);
        }
        self.update_count += 1;

        Ok(())
    }


}
