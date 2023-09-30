use enigo::{Enigo, KeyboardControllable, MouseButton, MouseControllable};
use napi::bindgen_prelude::*;

#[napi]
pub fn send_keyboard_simulation(cmd: String) {
  let mut enigo = Enigo::new();
  enigo.key_sequence_parse(&cmd);
}

#[napi]
pub enum MouseBtn {
  Left,
  Middle,
  Right,
  #[cfg(any(target_os = "windows", target_os = "linux"))]
  Back,
  #[cfg(any(target_os = "windows", target_os = "linux"))]
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

fn convert_btn(btn: Option<MouseBtn>) -> MouseButton {
  match btn {
    Some(MouseBtn::Left) => MouseButton::Left,
    Some(MouseBtn::Middle) => MouseButton::Middle,
    Some(MouseBtn::Right) => MouseButton::Right,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Some(MouseBtn::Back) => MouseButton::Back,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Some(MouseBtn::Forward) => MouseButton::Forward,
    None => panic!("未输入按钮"),
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
    MouseAction::Up => {
      enigo.mouse_up(convert_btn(input.button));
      None
    }
    MouseAction::Down => {
      enigo.mouse_down(convert_btn(input.button));
      None
    }
    MouseAction::Click => {
      enigo.mouse_click(convert_btn(input.button));
      None
    }
  }
}

// #[napi(ts_args_type = "callback: (content: any) => void")]
// pub fn on_input_event(callback: JsFunction) {}
