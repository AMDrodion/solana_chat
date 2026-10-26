use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProgramInstruction {
  SendMessage {
    eph_pk: [u8; 32],
    encoded_key: Vec<u8>,
    /// Текст сообщения для отправки
    encoded_msg: Vec<u8>
  }
}
