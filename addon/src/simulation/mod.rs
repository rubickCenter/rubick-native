use enigo::{Enigo, KeyboardControllable, MouseButton, MouseControllable};
use napi::bindgen_prelude::*;
use napi::Result;

#[napi]
pub fn send_keyboard_simulation(cmd: String) -> Result<()> {
  let mut enigo = Enigo::new();
  if let Err(e) = enigo.key_sequence_parse_try(&cmd) {
    Err(napi::Error::from_reason(e.to_string()))
  } else {
    Ok(())
  }
}

#[napi]
#[derive(Debug)]
pub enum MouseBtn {
  Left,
  Middle,
  Right,
  Back,
  Forward,
}

#[napi]
pub enum MouseAction {
  Locaion,
  MoveRelative,
  MoveTo,
  ScrollX,
  ScrollY,
  Up,
  Down,
  Click,
}

#[napi(object)]
pub struct MouseActionInput {
  pub action: MouseAction,
  pub data: Option<Position>,
  pub button: Option<MouseBtn>,
}

#[napi(object)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

fn convert_btn(btn: Option<MouseBtn>) -> Option<MouseButton> {
  match btn {
    Some(MouseBtn::Left) => Some(MouseButton::Left),
    Some(MouseBtn::Middle) => Some(MouseButton::Middle),
    Some(MouseBtn::Right) => Some(MouseButton::Right),
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Some(MouseBtn::Back) => Some(MouseButton::Back),
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Some(MouseBtn::Forward) => Some(MouseButton::Forward),
    #[allow(unreachable_patterns)]
    Some(b) => {
      println!("未识别按钮: {:#?}", b);
      None
    }
    None => {
      println!("未输入按钮");
      None
    }
  }
}

#[napi]
pub fn send_mouse_simulation(input: MouseActionInput) -> Option<Position> {
  let mut enigo = Enigo::new();

  match input.action {
    MouseAction::MoveRelative => {
      if let Some(p) = input.data {
        enigo.mouse_move_relative(p.x, p.y);
      }
      None
    }
    MouseAction::MoveTo => {
      if let Some(p) = input.data {
        enigo.mouse_move_to(p.x, p.y);
      }
      None
    }
    MouseAction::ScrollX => {
      if let Some(p) = input.data {
        enigo.mouse_scroll_x(p.x);
      }
      None
    }
    MouseAction::ScrollY => {
      if let Some(p) = input.data {
        enigo.mouse_scroll_y(p.y);
      }
      None
    }
    MouseAction::Locaion => {
      let (x, y) = enigo.mouse_location();
      Some(Position { x, y })
    }
    MouseAction::Down => {
      if let Some(b) = convert_btn(input.button) {
        enigo.mouse_down(b);
      }
      None
    }
    MouseAction::Up => {
      if let Some(b) = convert_btn(input.button) {
        enigo.mouse_up(b);
      }
      None
    }
    MouseAction::Click => {
      if let Some(b) = convert_btn(input.button) {
        enigo.mouse_click(b);
      }
      None
    }
  }
}
