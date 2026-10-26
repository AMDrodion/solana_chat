use sha2::{Digest, Sha256};

pub fn get_discriminator(input: &[u8]) -> [u8; 8] {
  let mut hasher = Sha256::new();
  Digest::update(&mut hasher, input);
  let hash = hasher.finalize();
  hash[..8].try_into().unwrap()
}
