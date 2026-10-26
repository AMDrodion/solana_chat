use std::error::Error;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use curve25519_dalek::edwards::CompressedEdwardsY;
use rand_core::{OsRng, RngCore};
use solana_sdk::pubkey::Pubkey;
use x25519_dalek::EphemeralSecret;
use x25519_dalek::PublicKey as x25519PubKey;

use super::kdf::get_nonce_and_kek;

pub async fn encode_msg(to: &Pubkey, msg: &String)
                        -> Result<(Vec<u8>, Vec<u8>, [u8; 32]), Box<dyn Error>> {
  let x25519_pubkey = CompressedEdwardsY(to.to_bytes())
      .decompress()
      .ok_or("invalid recipient pubkey: not an ed25519 point")?
      .to_montgomery()
      .to_bytes();

  let eph_sk = EphemeralSecret::random_from_rng(OsRng);
  let eph_pk = x25519PubKey::from(&eph_sk).to_bytes();

  let shared_secret = eph_sk.diffie_hellman(&x25519_pubkey.into());

  if shared_secret.as_bytes().iter().all(|&b| b == 0) {
    return Err("invalid shared secret (all zero)".into());
  }

  let (nonce, kek) = get_nonce_and_kek(&eph_pk, &to, &shared_secret)
      .await.or_else(|e| Err(format!("{}", e)))?;
  let cipher = Aes256Gcm::new_from_slice(&kek)?;

  let mut aes_key = [0u8; 32];
  OsRng.fill_bytes(&mut aes_key);

  let encrypted_aes_key = cipher.encrypt(
    Nonce::from_slice(nonce.as_slice()),
    aes_key.as_ref()
  ).or_else(|e| Err(format!("{}", e)))?;

  let msg_cipher = Aes256Gcm::new_from_slice(aes_key.as_slice())?;
  let encrypted_msg = msg_cipher.encrypt(
    Nonce::from_slice(nonce.as_slice()),
    msg.as_bytes()
  ).or_else(|e| Err(format!("{}", e)))?;

  Ok((
    encrypted_aes_key,
    encrypted_msg,
    eph_pk
  ))
}
