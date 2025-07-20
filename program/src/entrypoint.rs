#![allow(unexpected_cfgs)]

use crate::instruction::{self, MyProgramInstruction};
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, msg, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match MyProgramInstruction::try_from(ix_disc)? {
        MyProgramInstruction::InitializeState => {
            msg!("Ix:0");
            instruction::process_initialize_state_v1(accounts, instruction_data)?;
            instruction::process_initialize_state_v2(accounts,instruction_data)?;
            Ok(())

        }
        MyProgramInstruction::UpdateState => {
            msg!("Ix:1");
            instruction::process_update_state_v1(accounts, instruction_data)?;
            instruction::process_update_state_v2(accounts, instruction_data)?;
            Ok(())
        }
    }
}
