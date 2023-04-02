#![allow(unused)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

mod platform_impl;
mod util;

/// TODO list:
/// [ ] multiple display support
/// [ ] interruptable interactivity
/// [ ] keycode mapping to string
/// [x] improved computer power patch API

#[cfg(windows_platform)]
mod windows {
  use napi::{JsBigInt, JsFunction};
  use napi_derive::napi;

  use crate::platform_impl::events;
  use crate::platform_impl::power;
  use crate::platform_impl::window;

  #[napi(
    ts_args_type = "window: BigInt, callback?: (err: null | Error, event: InputEvent) => void"
  )]
  pub fn setup_interactive_window(window: JsBigInt, callback: Option<JsFunction>) {
    // step 1: set parent window
    window::setup_interactive_parent_window(window);
    // step 2: set keyboard and mouse events
    events::setup_interactive_window(window, callback);
  }

  #[napi]
  pub fn get_desktop_icon_visibility() -> bool {
    window::get_desktop_icon_visibility()
  }

  #[napi]
  pub fn set_desktop_icon_visibility(isVisible: bool) {
    window::set_desktop_icon_visibility(isVisible)
  }

  #[napi]
  pub fn is_desktop() -> bool {
    window::is_desktop()
  }

  #[napi]
  pub fn restore_interactive_window() {
    window::restore_interactive_parent_window();
    events::restore_interactive_window();
  }

  #[napi]
  pub fn create_shutdown_blocker(reason: String, callback: JsFunction) {
    unsafe {
      power::create_shutdown_blocker(reason.as_str(), callback);
    }
  }

  #[napi]
  pub fn destroy_shutdown_blocker() {
    unsafe {
      power::destroy_shutdown_blocker();
    }
  }
}

#[cfg(linux_platform)]
mod linux {
  use napi::{JsBigInt, JsFunction};
  use napi_derive::napi;

  #[napi(
    ts_args_type = "window: BigInt, callback?: (err: null | Error, event: InputEvent) => void"
  )]
  pub fn setup_interactive_window(window: JsBigInt, callback: Option<JsFunction>) {
    unsafe {
      // step: set keyboard and mouse events
    }
  }

  #[napi]
  pub fn restore_interactive_window() {}

  #[napi]
  pub fn get_desktop_icon_visibility() -> bool {
    true
  }

  #[napi]
  pub fn set_desktop_icon_visibility(isVisible: bool) {}

  #[napi]
  pub fn set_main_window_handle(bigint: JsBigInt) {}

  #[napi]
  pub fn is_desktop() -> bool {
    false
  }

  #[napi]
  pub fn create_shutdown_blocker(reason: String, callback: JsFunction) {}

  #[napi]
  pub fn destroy_shutdown_blocker() {}
}

#[cfg(macos_platform)]
mod macos {
  use napi::{JsBigInt, JsFunction};
  use napi_derive::napi;

  #[napi(
    ts_args_type = "window: BigInt, callback?: (err: null | Error, event: InputEvent) => void"
  )]
  pub fn setup_interactive_window(window: JsBigInt, callback: Option<JsFunction>) {
    unsafe {
      // step: set keyboard and mouse events
    }
  }

  #[napi]
  pub fn restore_interactive_window() {}

  #[napi]
  pub fn get_desktop_icon_visibility() -> bool {
    true
  }

  #[napi]
  pub fn set_desktop_icon_visibility(isVisible: bool) {}

  #[napi]
  pub fn set_main_window_handle(bigint: JsBigInt) {}

  #[napi]
  pub fn is_desktop() -> bool {
    false
  }

  #[napi]
  pub fn create_shutdown_blocker(reason: String, callback: JsFunction) {}

  #[napi]
  pub fn destroy_shutdown_blocker() {}
}
