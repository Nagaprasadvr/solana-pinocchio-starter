use pinocchio::program_error::ProgramError;

pub mod initialize_mystate;
pub mod update_mystate;

pub use initialize_mystate::*;
pub use update_mystate::*;

#[repr(u8)]
pub enum MyProgramInstruction {
    InitializeState,
    UpdateState,
}

impl TryFrom<&u8> for MyProgramInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MyProgramInstruction::InitializeState),
            1 => Ok(MyProgramInstruction::UpdateState),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

mod idl_gen {
    use super::InitializeMyStateV1IxData;

    #[derive(shank::ShankInstruction)]
    enum _MyProgramInstruction {
        #[account(0, writable, signer, name = "payer_acc", desc = "Fee payer account")]
        #[account(1, writable, name = "state_acc", desc = "New State account")]
        #[account(2, name = "sysvar_rent_acc", desc = "Sysvar rent account")]
        #[account(3, name = "system_program_acc", desc = "System program account")]
        InitializeState(InitializeMyStateV1IxData),
        #[account(0, writable, signer, name = "payer_acc", desc = "Fee payer account")]
        #[account(1, writable, name = "state_acc", desc = "State account")]
        UpdateState,
    }
}
