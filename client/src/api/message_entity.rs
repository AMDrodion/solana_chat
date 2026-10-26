use std::error::Error;
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use x25519_dalek::PublicKey as x25519PubKey;

use crate::crypto::decode_msg;
use crate::program::MessageAccount;

#[derive(Debug, Serialize)]
pub struct MessageEntity {
  pub message_address: String,
  pub sender: String,
  pub unix_timestamp: i64,
  pub message: String
}

pub async fn msg_acc_to_msg_entity(acc: &MessageAccount, msg_addr: &Pubkey, sk: &Keypair)
                                   -> Result<MessageEntity, Box<dyn Error + Send + Sync>> {
  let message = decode_msg(
    sk,
    &x25519PubKey::from(acc.eph_pk),
    &acc.encoded_key,
    &acc.encoded_msg
  ).await?;

  Ok(MessageEntity {
    message_address: msg_addr.to_string(),
    sender: acc.sender.to_string(),
    unix_timestamp: acc.unix_timestamp,
    message
  })
}
