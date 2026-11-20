//! The RISCV NC data mover: the same mover as RISCV B's (`src/mover.rs`), on
//! NC's mailbox, list ring and scratch (`tensix_isa::dm::nc`). Loaded at the top
//! of L1 and reached through the stub at NC's reset PC
//! (`Device::load_and_start_nc`).

#![no_std]
#![no_main]

/// NC's mailbox, list ring and scratch.
const M: tensix_isa::dm::Mover = tensix_isa::dm::Mover::NC;

include!("../mover.rs");
