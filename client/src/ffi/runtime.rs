use std::any::Any;
use std::ffi::CString;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use futures::FutureExt;
use lazy_static::lazy_static;
use tokio::runtime::Builder;

use super::convert::Callback;

lazy_static! {
  static ref RT: tokio::runtime::Runtime = Builder::new_multi_thread()
    .enable_all()
    .thread_name("ffi tokio")
    .build()
    .expect("Failed to create Tokio runtime");
}

/// Запускает задачу в рантайме. Если задача паникует, вызывает callback
pub fn spawn_with_callback<F>(callback: Callback, task: F)
where
  F: Future<Output = ()> + Send + 'static
{
  RT.spawn(async move {
    if let Err(payload) = AssertUnwindSafe(task).catch_unwind().await {
      let msg = format!("internal error: {}", panic_message(&payload));
      let p = CString::new(msg)
          .unwrap_or_else(|_| CString::new("internal error").unwrap())
          .into_raw();
      callback(false, p);
    }
  });
}

fn panic_message(payload: &Box<dyn Any + Send>) -> &str {
  if let Some(s) = payload.downcast_ref::<&str>() {
    s
  } else if let Some(s) = payload.downcast_ref::<String>() {
    s
  } else {
    "panic"
  }
}
