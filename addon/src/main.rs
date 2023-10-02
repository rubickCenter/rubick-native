// #[cfg(feature = "unstable_grab")]
use rdev::{grab, Event};

fn main() {
  // #[cfg(feature = "unstable_grab")]
  let callback = |event: Event| -> Option<Event> {
    println!("{}", serde_json::to_string_pretty(&event).unwrap());
    None // CapsLock is now effectively disabled
         // if let EventType::KeyPress(Key::CapsLock) = event.event_type {
         // } else {
         //   Some(event)
         // }
  };
  // This will block.
  // #[cfg(feature = "unstable_grab")]
  println!("{}", 1);
  if let Err(error) = grab(callback) {
    println!("Error: {:?}", error)
  }
}
