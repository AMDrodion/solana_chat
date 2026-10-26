use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use crate::instructions::ProgramInstruction;
use crate::instructions::ProgramInstruction::SendMessage;
use crate::processor::process_send_message::process_send_message;

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  let instruction = ProgramInstruction::try_from_slice(instruction_data)?;
  match instruction {
    SendMessage { eph_pk, encoded_key, encoded_msg } => {
      process_send_message(program_id, accounts, eph_pk, encoded_key, encoded_msg)
    }
  }
}
