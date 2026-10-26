use std::error::Error;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_system_interface::program::ID as SystemProgramID;

use crate::config::{DEVNET_URL, PROGRAM_PUBKEY};
use crate::crypto::encode_msg;
use crate::program::ProgramInstruction;

/// Отправляет сообщение указанному получателю
pub async fn send_message(from: &Keypair, to: &Pubkey, msg: &String)
                          -> Result<(), Box<dyn Error>> {
  let client = RpcClient::new(DEVNET_URL.to_string());

  let (encoded_key, encoded_msg, eph_pk) = encode_msg(to, msg).await?;

  let program_id = Pubkey::from_str_const(PROGRAM_PUBKEY);

  let (account_pda, _) = Pubkey::find_program_address(
    &[
      from.pubkey().as_ref(), to.as_ref(), eph_pk.as_ref()
    ],
    &program_id
  );

  if client.get_account(&account_pda).await.is_ok() {
    return Err(format!("message account {account_pda} already exists").into());
  }

  let accounts = vec![
    AccountMeta::new(from.pubkey(), true),
    AccountMeta::new_readonly(to.clone(), false),
    AccountMeta::new(account_pda.clone(), false),
    AccountMeta::new_readonly(Pubkey::from(SystemProgramID.to_bytes()), false)
  ];

  let instruction_data = ProgramInstruction::SendMessage { eph_pk, encoded_key, encoded_msg };

  let send_ix = Instruction::new_with_borsh(
    program_id.clone(),
    &instruction_data,
    accounts
  );

  let hash = client.get_latest_blockhash().await?;
  let send_tx = Transaction::new_signed_with_payer(
    &[send_ix],
    Some(&from.pubkey()),
    &[from],
    hash
  );

  client.send_and_confirm_transaction(&send_tx).await?;

  Ok(())
}
