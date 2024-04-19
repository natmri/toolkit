use napi::{threadsafe_function::ThreadsafeFunction, JsBigInt, JsFunction};
use rdev::{listen, EventType};

use crate::util::InputEvent;

use super::util::code_from_key;

static mut CALLBACK: Option<ThreadsafeFunction<InputEvent>> = None;

pub fn setup_interactive_window(
  _window: JsBigInt,
  callback: Option<JsFunction>,
) -> Result<(), napi::Error> {
  unsafe {
    if let Some(callback) = callback {
      let callback: ThreadsafeFunction<InputEvent> =
        callback.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;

      CALLBACK = Some(callback);
    }
  }

  let _ = std::thread::spawn(|| loop {
    listen(|e| {
      let input_event = match e.event_type {
        EventType::MouseMove { x, y } => InputEvent {
          kind: crate::util::RawEvent::MouseMove,
          x: x as i32,
          y: y as i32,
          ..Default::default()
        },
        EventType::Wheel { delta_x, delta_y } => InputEvent {
          kind: crate::util::RawEvent::MouseWheel,
          delta: delta_y as i32,
          ..Default::default()
        },
        EventType::ButtonPress(button) => InputEvent {
          kind: crate::util::RawEvent::MouseDown,
          ..Default::default()
        },
        EventType::ButtonRelease(button) => InputEvent {
          kind: crate::util::RawEvent::MouseUp,
          ..Default::default()
        },
        EventType::KeyPress(key) => InputEvent {
          kind: crate::util::RawEvent::KeyDown,
          virtual_keycode: code_from_key(key).unwrap_or(0_u32),
          ..Default::default()
        },
        EventType::KeyRelease(key) => InputEvent {
          kind: crate::util::RawEvent::KeyUp,
          virtual_keycode: code_from_key(key).unwrap_or(0_u32),
          ..Default::default()
        },
      };

      unsafe {
        if let Some(callback) = CALLBACK.clone() {
          callback.call(
            Ok(input_event),
            napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
          );
        }
      }
    });
  });

  Ok(())
}

pub fn restore_interactive_window() {
  unsafe {
    if CALLBACK.is_some() {
      CALLBACK = None;
    }
  }
}
