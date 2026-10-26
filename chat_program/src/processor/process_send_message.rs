use borsh::BorshSerialize;

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, clock,
                     program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
                     account_info::next_account_infos, program::invoke_signed};
use solana_sdk_ids::system_program;
use solana_system_interface::instruction;

use crate::accounts::MessageAccount;
use crate::processor::get_discriminator::get_discriminator;

struct SendMessageAccounts<'a, 'info> {
  sender: &'a AccountInfo<'info>,
  recipient: &'a AccountInfo<'info>,
  msg_pda: &'a AccountInfo<'info>,
  system_program: &'a AccountInfo<'info>,
}

pub fn process_send_message(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  eph_pk: [u8; 32],
  encoded_key: Vec<u8>,
  encoded_msg: Vec<u8>
) -> ProgramResult {
  let accounts = parse_accounts(accounts)?;
  validate_accounts(&accounts)?;

  let bump = verify_msg_pda(program_id, &accounts, &eph_pk)?;
  let account_space = message_account_space(&encoded_key, &encoded_msg);
  create_msg_pda(program_id, &accounts, &eph_pk, bump, account_space)?;

  let msg_account = MessageAccount {
    discriminator: get_discriminator(b"MessageAccount"),
    sender: *accounts.sender.key,
    recipient: *accounts.recipient.key,
    unix_timestamp: clock::Clock::get()?.unix_timestamp,
    bump,
    eph_pk,
    encoded_key,
    encoded_msg
  };
  write_msg_account(accounts.msg_pda, &msg_account)?;

  msg!("Account {} was created.", accounts.msg_pda.key);
  Ok(())
}

fn parse_accounts<'a, 'info>(
  accounts: &'a [AccountInfo<'info>]
) -> Result<SendMessageAccounts<'a, 'info>, ProgramError> {
  let account_iter = &mut accounts.iter();
  let [sender, recipient, msg_pda, system_program]
      = next_account_infos(account_iter, 4)?
  else {
   return Err(ProgramError::NotEnoughAccountKeys)
  };

  Ok(SendMessageAccounts { sender, recipient, msg_pda, system_program })
}

fn validate_accounts(accounts: &SendMessageAccounts) -> ProgramResult {
  if *accounts.system_program.key != system_program::ID {
    return Err(ProgramError::IncorrectProgramId)
  }

  if !accounts.sender.is_signer {
    return Err(ProgramError::MissingRequiredSignature)
  }

  Ok(())
}

fn msg_pda_seeds<'a>(accounts: &'a SendMessageAccounts, eph_pk: &'a [u8; 32]) -> [&'a [u8]; 3] {
  [
    accounts.sender.key.as_ref(),
    accounts.recipient.key.as_ref(),
    eph_pk.as_ref(),
  ]
}

fn verify_msg_pda(
  program_id: &Pubkey,
  accounts: &SendMessageAccounts,
  eph_pk: &[u8; 32]
) -> Result<u8, ProgramError> {
  let (expected_pda, bump) = Pubkey::find_program_address(
    &msg_pda_seeds(accounts, eph_pk),
    program_id
  );

  if accounts.msg_pda.lamports() > 0 {
    return Err(ProgramError::AccountAlreadyInitialized)
  }

  if expected_pda != *accounts.msg_pda.key {
    return Err(ProgramError::InvalidAccountData)
  }

  Ok(bump)
}

fn message_account_space(encoded_key: &[u8], encoded_msg: &[u8]) -> usize {
  8 + // discriminator
  32 + // sender
  32 + // recipient
  8 + // unix_timestamp
  1 + // bump
  32 + //eph_pk
  4 + encoded_key.len() + // prefix + key
  4 + encoded_msg.len() // prefix + message
}

fn create_msg_pda(
  program_id: &Pubkey,
  accounts: &SendMessageAccounts,
  eph_pk: &[u8; 32],
  bump: u8,
  account_space: usize
) -> ProgramResult {
  let SendMessageAccounts { sender, msg_pda, system_program, .. } = accounts;
  let [sender_seed, recipient_seed, eph_pk_seed] = msg_pda_seeds(accounts, eph_pk);

  let required_lamports = Rent::get()?.minimum_balance(account_space);
  invoke_signed(
    &instruction::create_account(
      sender.key,
      msg_pda.key,
      required_lamports,
      account_space as u64,
      program_id,
    ),
    &[(*sender).clone(), (*msg_pda).clone(), (*system_program).clone()],
    &[&[sender_seed, recipient_seed, eph_pk_seed, &[bump]]]
  )
}

fn write_msg_account(msg_pda: &AccountInfo, msg_account: &MessageAccount) -> ProgramResult {
  let mut msg_account_data = msg_pda.data.borrow_mut();
  msg_account.serialize(&mut &mut msg_account_data[..])?;
  Ok(())
}
