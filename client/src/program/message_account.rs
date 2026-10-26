use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

/// Структура аккаунта для хранения сообщения
#[derive(BorshDeserialize, Debug)]
pub struct MessageAccount {
  /// Дискриминатор для идентификации типа аккаунта
  pub discriminator: [u8; 8],
  /// Публичный ключ отправителя
  pub sender: Pubkey,
  /// Публичный ключ получателя
  pub recipient: Pubkey,
  /// Время создания аккаунта
  pub unix_timestamp: i64,
  /// Bump-значение для PDA-аккаунта
  pub bump: u8,
  pub eph_pk: [u8; 32],
  pub encoded_key: Vec<u8>,
  /// Текст сообщения
  pub encoded_msg: Vec<u8>
}
