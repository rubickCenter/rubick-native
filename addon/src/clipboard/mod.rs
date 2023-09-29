use clipboard_master::{CallbackResult, ClipboardHandler, Master};

use std::{
  sync::mpsc::{self, Sender},
  thread::spawn,
};

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

struct Handler {
  pub tx: Sender<()>,
}

impl ClipboardHandler for Handler {
  fn on_clipboard_change(&mut self) -> CallbackResult {
    self.tx.send(()).unwrap();
    CallbackResult::Next
  }
}

#[napi]
pub fn on_clipboard_change(callback: JsFunction) {
  let jsfn: ThreadsafeFunction<(), ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |_| Ok(vec![0]))
    .unwrap();
  let (tx, rx) = mpsc::channel();
  spawn(|| {
    let _ = Master::new(Handler { tx }).run();
  });
  spawn(move || {
    for _ in rx {
      jsfn.call(Ok(()), ThreadsafeFunctionCallMode::Blocking);
    }
  });
}
