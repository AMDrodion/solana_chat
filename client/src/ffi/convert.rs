use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

pub type Callback = extern "C" fn(success: bool, error_msg: *const c_char);

pub fn c_char_to_pubkey(pubkey_str: *const c_char, callback: Callback) -> Option<Pubkey> {
  if pubkey_str.is_null() {
    callback(false, CString::new("pubkey_str is null").unwrap().into_raw());
    return None;
  }
  let s = match unsafe { CStr::from_ptr(pubkey_str) }.to_str() {
    Ok(s) => s,
    Err(e) => {
      let p = CString::new(format!("Invalid UTF-8: {e}")).unwrap().into_raw();
      callback(false, p);
      return None;
    }
  };

  let pk = match Pubkey::from_str(s) {
    Ok(p) => p,
    Err(e) => {
      let p = CString::new(format!("Invalid pubkey: {e}")).unwrap().into_raw();
      callback(false, p);
      return None;
    }
  };
  Some(pk)
}

pub fn keypair_from_ptr(keypair_ptr: *const [u8; 64], callback: Callback) -> Option<Keypair> {
  if keypair_ptr.is_null() {
    callback(false, CString::new("keypair_ptr is null").unwrap().into_raw());
    return None;
  }
  match unsafe { Keypair::try_from((*keypair_ptr).as_slice()) } {
    Ok(kp) => Some(kp),
    Err(e) => {
      let p = CString::new(format!("Error: {e}")).unwrap().into_raw();
      callback(false, p);
      None
    }
  }
}
