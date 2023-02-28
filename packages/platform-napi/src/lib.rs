#[cfg(windows)]
mod win32;

#[cfg(windows)]
#[allow(unused)]
mod windows {
  use super::win32::power;
  use super::win32::window;
  use napi::{bindgen_prelude::*, JsBigInt};
  use napi_derive::napi;

  #[napi]
  pub fn setup_inactive_window(bigint: JsBigInt, callback: Option<JsFunction>) {
    unsafe {
      // step 1: set parent window
      window::setup_interactive_parent_window(bigint);
    }
  }

  #[napi]
  pub fn restore_inactive_window() {
    unsafe {
      window::restore_interactive_parent_window();
    }
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

#[cfg(linux)]
#[allow(unused)]
mod linux {}

#[cfg(macos)]
#[allow(unused)]
mod macos {}
