use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
  JsBoolean, Result,
};
use rdev::{grab, listen, Event};
use std::{
  sync::mpsc::{self, Sender},
  thread::spawn,
};

#[napi(ts_args_type = "callback: (event: string) => void")]
pub fn on_input_event(callback: JsFunction) -> Result<()> {
  let jsfn: ThreadsafeFunction<String, ErrorStrategy::Fatal> =
    callback.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;

  spawn(|| {
    if let Err(error) = listen(move |event| {
      jsfn.call(
        serde_json::to_string(&event).unwrap(),
        ThreadsafeFunctionCallMode::NonBlocking,
      );
    }) {
      println!("Error: {:?}", error)
    }
  });
  Ok(())
}

#[napi(ts_args_type = "callback: (event: string) => boolean")]
pub fn grab_input_event(callback: JsFunction) -> Result<()> {
  let jsfn: ThreadsafeFunction<String, ErrorStrategy::Fatal> =
    callback.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;

  let gcallback = move |event: Event| -> Option<Event> {
    let (s, r): (Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel::<bool>();
    jsfn.call_with_return_value(
      serde_json::to_string(&event).unwrap(),
      ThreadsafeFunctionCallMode::NonBlocking,
      move |e: JsBoolean| {
        if let Ok(goon) = e.get_value() {
          if !goon {
            // 需要拦截事件
            s.send(false).unwrap();
          }
        }
        s.send(true).unwrap();
        Ok(())
      },
    );
    for i in r {
      if !i {
        return None;
      }
    }
    Some(event)
  };

  spawn(|| {
    if let Err(error) = grab(gcallback) {
      println!("GrabError: {:?}", error)
    }
  });
  Ok(())
}
