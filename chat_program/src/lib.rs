mod processor;
mod instructions;
mod accounts;

use solana_program::{declare_id, entrypoint};

use crate::processor::process_instruction;

declare_id!("EFzL4paTDicz7v1rnXzHrWcpLAtrKJcZr2Buw8iGwLW7");

entrypoint!(process_instruction);
