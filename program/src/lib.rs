#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod instruction;
pub mod state;

pinocchio_pubkey::declare_id!("BDLpZV7rAqUrE4KnXfctjvmUwbcMtnEV2d9VAtmDqc2U");
