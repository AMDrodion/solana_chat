use std::error::Error;
use borsh::BorshDeserialize;
use futures::{stream, StreamExt};
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, RpcFilterType};
use solana_sdk::account::ReadableAccount;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

use crate::config::{DEVNET_URL, PROGRAM_PUBKEY};
use crate::program::MessageAccount;
use super::message_entity::{msg_acc_to_msg_entity, MessageEntity};

/// Получает все сообщения, адресованные указанному публичному ключу
pub async fn get_messages(your_sk: &Keypair)
                          -> Result<Vec<MessageEntity>, Box<dyn Error + Send + Sync + 'static>> {
  let client = RpcClient::new(DEVNET_URL.to_string());

  let filters = vec![
    RpcFilterType::Memcmp(
      Memcmp::new_base58_encoded(40, your_sk.pubkey().as_array())
    )
  ];

  let filters = RpcProgramAccountsConfig {
    filters: Some(filters),
    account_config: RpcAccountInfoConfig {
      encoding: Some(UiAccountEncoding::Base64),
      ..Default::default()
    },
    ..Default::default()
  };

  let messages_vec = client.get_program_accounts_with_config(
    &Pubkey::from_str_const(PROGRAM_PUBKEY),
    filters
  ).await?;

  let messages_vec: Vec<(Pubkey, MessageAccount)> = messages_vec.into_iter()
      .map(|acc| {
        if let Ok(msg_acc) = MessageAccount::try_from_slice(acc.1.data()) {
          return Some((acc.0, msg_acc))
        }
        None
      })
      .flatten()
      .collect();

  let messages_vec: Vec<MessageEntity> = stream::iter(messages_vec.into_iter())
      .then(|msg| async move {
        msg_acc_to_msg_entity(&msg.1, &msg.0, your_sk).await
      })
      .filter_map(|result| async move {
        result.ok()
      })
      .collect()
      .await;

  Ok(messages_vec)
}
