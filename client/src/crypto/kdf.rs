use std::error::Error;
use hkdf::Hkdf;
use sha2::Sha256;
use solana_sdk::pubkey::Pubkey;
use x25519_dalek::SharedSecret;

const KDF_VERSION: &[u8] = b"solana-ecdh-aes256gcm-v1";

async fn hkdf_expand(shared: &[u8], salt: &[u8], ctx: &[u8], label: &[u8], out: &mut [u8])
                     -> Result<(), Box<dyn Error + Sync + Send>> {
  let hk = Hkdf::<Sha256>::new(Some(salt), shared);

  let mut info = Vec::with_capacity(KDF_VERSION.len() + ctx.len() + label.len());
  info.extend_from_slice(KDF_VERSION);
  info.extend_from_slice(ctx);
  info.extend_from_slice(label);

  hk.expand(&info, out)
      .or_else(|e| Err(format!("{}", e)))?;

  Ok(())
}

pub async fn get_nonce_and_kek(eph_pk: &[u8; 32], recv_pk: &Pubkey, shared_secret: &SharedSecret)
                               -> Result<([u8; 12], [u8; 32]), Box<dyn Error + Sync + Send>> {
  let mut ctx = Vec::with_capacity(64);
  ctx.extend_from_slice(eph_pk);
  ctx.extend_from_slice(recv_pk.as_array());

  let mut nonce = [0u8; 12];
  let mut kek = [0u8; 32];
  hkdf_expand(shared_secret.as_bytes(), eph_pk, ctx.as_slice(), b"|wrap-nonce", &mut nonce).await?;
  hkdf_expand(shared_secret.as_bytes(), eph_pk, ctx.as_slice(), b"|wrap-key", &mut kek).await?;

  Ok((nonce, kek))
}
