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
