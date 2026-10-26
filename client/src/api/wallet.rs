use std::error::Error;
use std::time::Duration;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};

use crate::config::DEVNET_URL;

/// Генерирует ключи
pub fn get_keypair_as_bytes() -> [u8; 64] {
  Keypair::new().to_bytes()
}

/// Запрашивает эирдроп на указанный кошелёк в Devnet
pub async fn airdrop(pubkey: &Pubkey) -> Result<(), Box<dyn Error>> {
  let rpc_client = RpcClient::new(DEVNET_URL.to_string());
  let lamports = LAMPORTS_PER_SOL * 2;

  let signature = rpc_client.request_airdrop(pubkey, lamports).await?;

  wait_for_confirmation(&rpc_client, &signature, Duration::from_secs(30)).await
}

/// Ожидает подтверждения транзакции
async fn wait_for_confirmation(client: &RpcClient, signature: &Signature, timeout: Duration)
  -> Result<(), Box<dyn Error>> {
  let start = std::time::Instant::now();

  while start.elapsed() < timeout {
    let status = client.get_signature_status(signature).await?;

    if let Some(transaction_status) = status {
      return if transaction_status.is_ok() {
        Ok(())
      } else {
        Err("Transaction failed".into())
      }
    };
    tokio::time::sleep(Duration::from_millis(500)).await;
  }

  Err("Confirmation timeout".into())
}
