use borsh::BorshSerialize;

/// Инструкции программы для обработки транзакций
#[derive(BorshSerialize, Debug)]
pub enum ProgramInstruction {
  /// Отправить сообщение с указанным содержимым
  SendMessage {
    eph_pk: [u8; 32],
    encoded_key: Vec<u8>,
    /// Текст сообщения для отправки
    encoded_msg: Vec<u8>
  }
}
