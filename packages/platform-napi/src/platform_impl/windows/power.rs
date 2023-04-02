use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::JsFunction;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Shutdown::{ShutdownBlockReasonCreate, ShutdownBlockReasonDestroy};
use windows::Win32::System::Threading::SetProcessShutdownParameters;
use windows::Win32::UI::WindowsAndMessaging::{
  CallWindowProcW, CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassW,
  SetWindowLongPtrW, WNDCLASSW, WNDPROC, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
  WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE,
};
use windows::Win32::UI::WindowsAndMessaging::{GWLP_WNDPROC, WM_ENDSESSION, WM_QUERYENDSESSION};

use super::util::encode_wide;

pub static mut MAIN_WINDOW: HWND = HWND(0);
pub static mut PREV_WND_PROC: WNDPROC = None;
pub static mut SHOULD_BLOCK_SHUTDOWN: bool = false;
pub static mut FN: Option<ThreadsafeFunction<i32>> = None;

/// Low api implement windows 'shutdown' event for electron
/// ref: https://github.com/paymoapp/electron-shutdown-handler/blob/master/module/WinShutdownHandler.cpp

pub unsafe fn create_shutdown_blocker(reason: &str, callback: JsFunction) {
  if MAIN_WINDOW.eq(&HWND::default()) {
    let name = encode_wide("POWERSHUTDOWN_WINDOW").as_ptr();
    let mut wcx = WNDCLASSW::default();
    wcx.lpfnWndProc = Some(window_proc);

    wcx.lpszClassName = PCWSTR(name);
    RegisterClassW(&wcx);
    MAIN_WINDOW = CreateWindowExW(
      WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE,
      PCWSTR(name),
      None,
      WS_VISIBLE | WS_POPUP,
      0,
      0,
      1,
      1,
      None,
      None,
      None,
      None,
    );
  }

  if let Ok(func) = callback.create_threadsafe_function(0, |ctx| Ok(vec![0])) {
    FN = Some(func)
  }

  let mut reason: Vec<u16> = reason.encode_utf16().collect();
  reason.push(0);
  ShutdownBlockReasonCreate(MAIN_WINDOW, PCWSTR(reason.as_ptr()));
  SetProcessShutdownParameters(0x3FF, 0);
  SHOULD_BLOCK_SHUTDOWN = true;
}

pub unsafe fn destroy_shutdown_blocker() {
  SHOULD_BLOCK_SHUTDOWN = false;
  ShutdownBlockReasonDestroy(MAIN_WINDOW);
  DestroyWindow(MAIN_WINDOW);
  FN = None;
}

unsafe extern "system" fn window_proc(
  h_wnd: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  if msg == WM_QUERYENDSESSION {
    if let Some(func) = &FN {
      func.call(Ok(0), ThreadsafeFunctionCallMode::Blocking);
    }

    if SHOULD_BLOCK_SHUTDOWN {
      return LRESULT(0);
    }

    return LRESULT(1);
  } else if msg == WM_ENDSESSION {
    if w_param.0 == 0 {
      return LRESULT(0);
    }
  }

  DefWindowProcW(h_wnd, msg, w_param, l_param)
}
