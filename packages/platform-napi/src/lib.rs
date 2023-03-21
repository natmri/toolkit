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
/// [ ] improved computer power patch API

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
  pub fn restore_interactive_window() {
    window::restore_interactive_parent_window();
    events::restore_interactive_window();
  }

  #[napi]
  pub fn set_main_window_handle(bigint: JsBigInt) {
    unsafe {
      if let Ok((h_wnd, _)) = bigint.get_u64() {
        power::set_main_window_handle(windows::Win32::Foundation::HWND(h_wnd as isize));
      }
    }
  }

  #[napi]
  pub fn insert_wnd_proc_hook(callback: JsFunction) {
    unsafe {
      power::insert_wnd_proc_hook(callback);
    }
  }

  #[napi]
  pub fn remove_wnd_proc_hook() -> bool {
    unsafe { power::remove_wnd_proc_hook() }
  }

  #[napi]
  pub fn acquire_shutdown_block(reason: String) -> bool {
    unsafe { power::acquire_shutdown_block(reason.as_str()) }
  }

  #[napi]
  pub fn release_shutdown_block() -> bool {
    unsafe { power::release_shutdown_block() }
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
  pub fn insert_wnd_proc_hook(callback: JsFunction) {}

  #[napi]
  pub fn remove_wnd_proc_hook() -> bool {
    true
  }

  #[napi]
  pub fn acquire_shutdown_block(reason: String) -> bool {
    true
  }

  #[napi]
  pub fn release_shutdown_block() -> bool {
    true
  }
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
  pub fn insert_wnd_proc_hook(callback: JsFunction) {}

  #[napi]
  pub fn remove_wnd_proc_hook() -> bool {
    true
  }

  #[napi]
  pub fn acquire_shutdown_block(reason: String) -> bool {
    true
  }

  #[napi]
  pub fn release_shutdown_block() -> bool {
    true
  }
}
