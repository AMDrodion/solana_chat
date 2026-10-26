use std::error::Error;
use std::str::from_utf8;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use curve25519_dalek::scalar::clamp_integer;
use sha2::{Digest, Sha512};
use solana_sdk::signature::{Keypair, Signer};
use x25519_dalek::StaticSecret;
use x25519_dalek::PublicKey as x25519PubKey;

use super::kdf::get_nonce_and_kek;

pub async fn decode_msg(
  sk_recv: &Keypair,
  eph_pk: &x25519PubKey,
  encrypted_key: &Vec<u8>,
  msg: &Vec<u8>
) -> Result<String, Box<dyn Error + Send + Sync>> {
  let hash: [u8; 32] = Sha512::digest(sk_recv.secret_bytes())[..32].try_into()?;
  let x25519_secret = clamp_integer(hash);
  let x25519_secret = StaticSecret::from(x25519_secret);

  let shared_secret = x25519_secret.diffie_hellman(&eph_pk);

  if shared_secret.as_bytes().iter().all(|&b| b == 0) {
    return Err("invalid shared secret on receiver (all zero)".into());
  }

  let (nonce, kek) = get_nonce_and_kek(
    &eph_pk.as_bytes(),
    &sk_recv.pubkey(),
    &shared_secret
  ).await?;

  let cipher = Aes256Gcm::new_from_slice(&kek)?;
  let decrypted_aes_key = cipher.decrypt(
    Nonce::from_slice(nonce.as_slice()),
    encrypted_key.as_ref()
  ).or_else(|e| Err(format!("{}", e)))?;

  let msg_cipher = Aes256Gcm::new_from_slice(decrypted_aes_key.as_slice())?;
  let msg = msg_cipher.decrypt(
    Nonce::from_slice(nonce.as_slice()),
    msg.as_slice()
  ).or_else(|e| Err(format!("{}", e)))?;

  Ok(from_utf8(msg.as_slice())?.to_string())
}
