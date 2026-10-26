use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod convert;
mod runtime;

use convert::{c_char_to_pubkey, keypair_from_ptr, Callback};
use runtime::spawn_with_callback;
use crate::api::airdrop as request_airdrop;
use crate::api::{get_keypair_as_bytes, get_messages, send_message};

#[unsafe(no_mangle)]
pub extern "C" fn rust_string_free(p: *const c_char) {
  if !p.is_null() {
    unsafe { let _ = CString::from_raw(p as *mut c_char); }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_messages_async_from_sk_bytes(
  sk_ptr: *const [u8; 64],
  callback: Callback
) {
  let keypair = match keypair_from_ptr(sk_ptr, callback) {
    Some(kp) => kp,
    None => return 
  };

  spawn_with_callback(callback, async move {
    match get_messages(&keypair).await {
      Ok(msgs) => {
        match serde_json::to_string(&msgs) {
          Ok(json) => {
            let p = CString::new(json).unwrap().into_raw();
            callback(true, p);
          }
          Err(e) => {
            let p = CString::new(format!("serialize failed: {e}"))
                .unwrap()
                .into_raw();
            callback(false, p);
          }
        }
      }
      Err(e) => {
        let p = CString::new(e.to_string())
            .unwrap_or_else(|_| CString::new("Unknown error").unwrap())
            .into_raw();
        callback(false, p);
      }
    }
  });
}

#[unsafe(no_mangle)]
pub extern "C" fn send_message_ffi(
  from_bytes: *const[u8; 64],
  to_pubkey: *const c_char,
  msg_str: *const c_char,
  callback: Callback
) {
  if from_bytes.is_null() || to_pubkey.is_null() || msg_str.is_null() {
    callback(false, CString::new("nullptr provided").unwrap().into_raw());
    return;
  }

  let keypair = match keypair_from_ptr(from_bytes, callback) {
    Some(kp) => kp,
    None => return
  };

  let pubkey = match c_char_to_pubkey(to_pubkey, callback) {
    Some(p) => p,
    None => return
  };

  let msg_cstr = unsafe { CStr::from_ptr(msg_str) };
  let msg = match msg_cstr.to_str() {
    Ok(s) => s.to_string(),
    Err(_) => {
      let error = CString::new("Invalid UTF-8 in message").unwrap().into_raw();
      callback(false, error);
      return;
    }
  };

  spawn_with_callback(callback, async move {
    match send_message(&keypair, &pubkey, &msg).await {
      Ok(()) => callback(true, std::ptr::null()),
      Err(e) => {
        let error = CString::new(e.to_string())
            .unwrap_or_else(|_| CString::new("Unknown error").unwrap())
            .into_raw();
        callback(false, error);
      }
    }
  });
}

#[unsafe(no_mangle)]
pub extern "C" fn get_keypair(buffer: *mut u8) {
  if buffer.is_null() {
    return;
  }
  let bytes = get_keypair_as_bytes();
  unsafe {
    std::ptr::copy_nonoverlapping(
      bytes.as_ptr(),
      buffer,
      64
    )
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn airdrop(
  pubkey_str: *const c_char,
  callback: Callback
) {
  let pk = match c_char_to_pubkey(pubkey_str, callback) {
    Some(p) => p,
    None => return,
  };

  spawn_with_callback(callback, async move {
    let res = request_airdrop(&pk).await;
    match res {
      Ok(()) => callback(true, std::ptr::null::<c_char>()),
      Err(e) => {
        let msg = CString::new(e.to_string()).unwrap_or(CString::new("Unknown error").unwrap());
        callback(false, msg.into_raw());
      }
    }
  });
}
