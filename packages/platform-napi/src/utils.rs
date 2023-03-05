use core::ops::BitAnd;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub enum RawEvent {
  KeyDown,
  KeyUp,
  MouseDown,
  MouseUp,
  MouseMove,
  MouseWheel,
  UnKnown,
}

impl Default for RawEvent {
  fn default() -> Self {
    RawEvent::UnKnown
  }
}

#[napi(object)]
#[derive(Default)]
pub struct InputEvent {
  pub kind: RawEvent,
  pub x: i32,
  pub y: i32,
  pub scancode: u32,
  pub virtual_keycode: u32,
  pub delta: i32,
  pub modifiers: u32,
}

bitflags! {
  /// Represents the current state of the keyboard modifiers
  ///
  /// Each flag represents a modifier and is set if this modifier is active.
  #[derive(Default)]
  pub struct ModifiersState: u32 {
      // left and right modifiers are currently commented out, but we should be able to support
      // them in a future release
      /// The "shift" key.
      const SHIFT = 0b100;
      // const LSHIFT = 0b010;
      // const RSHIFT = 0b001;
      /// The "control" key.
      const CTRL = 0b100 << 3;
      // const LCTRL = 0b010 << 3;
      // const RCTRL = 0b001 << 3;
      /// The "alt" key.
      const ALT = 0b100 << 6;
      // const LALT = 0b010 << 6;
      // const RALT = 0b001 << 6;
      /// This is the "windows" key on PC and "command" key on Mac.
      const LOGO = 0b100 << 9;
      // const LLOGO = 0b010 << 9;
      // const RLOGO = 0b001 << 9;
  }
}

pub fn has_flag<T>(bitset: T, flag: T) -> bool
where
  T: Copy + PartialEq + BitAnd<T, Output = T>,
{
  bitset & flag == flag
}
