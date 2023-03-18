use napi::JsBigInt;
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::{BOOL, HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{
      EnumWindows, FindWindowExW, FindWindowW, GetDesktopWindow, GetForegroundWindow,
      SendMessageTimeoutW, SetParent, ShowWindow, SMTO_NORMAL, SW_HIDE, SW_SHOW,
    },
  },
};

use super::util::encode_wide;

lazy_static! {
  static ref PROGMAN: Vec<u16> = encode_wide("Progman");
  static ref PROGMAN_MANAGER: Vec<u16> = encode_wide("Program Manager");
  static ref SHELL_DLL_DEF_VIEW: Vec<u16> = encode_wide("SHELLDLL_DefView");
  static ref SYS_LIST_VIEW: Vec<u16> = encode_wide("SysListView32");
  static ref FOLDER_VIEW: Vec<u16> = encode_wide("FolderView");
  static ref WORKER_W: Vec<u16> = encode_wide("WorkerW");
  static ref EMPTY: Vec<u16> = encode_wide("");
}

static mut PROGMAN_WINDOW: HWND = HWND(0);
static mut WORKER_WINDOW_HANDLER: HWND = HWND(0);
static mut WORKER_WINDOW_ORIG: HWND = HWND(0);
static mut DEF_VIEW_WINDOW_HANDLER: HWND = HWND(0);
static mut __WORKER_WINDOW_HANDLER: HWND = HWND(0);
static mut FOLDER_VIEW_WINDOW_HANDLER: HWND = HWND(0);

pub unsafe fn setup_interactive_parent_window(bigint: JsBigInt) {
  let progman = find_progman_window();
  SendMessageTimeoutW(
    progman,
    0x052C,
    WPARAM(0xD),
    LPARAM(0x1),
    SMTO_NORMAL,
    1000,
    None,
  );

  if WORKER_WINDOW_ORIG.eq(&HWND::default()) {
    find_desktop_handles();
  }

  if let Ok((h_wnd, _)) = bigint.get_u64() {
    SetParent(HWND(h_wnd as isize), WORKER_WINDOW_ORIG);

    ShowWindow(WORKER_WINDOW_ORIG, SW_SHOW);
  }
}

pub unsafe fn restore_interactive_parent_window() {
  ShowWindow(WORKER_WINDOW_ORIG, SW_HIDE);
  // ShowWindow(WORKER_WINDOW_HANDLER, SW_HIDE);
}

fn find_worker_window() -> HWND {
  unsafe {
    let progman = find_progman_window();
    SendMessageTimeoutW(
      progman,
      0x052C,
      WPARAM(0xD),
      LPARAM(0x1),
      SMTO_NORMAL,
      1000,
      None,
    );

    if WORKER_WINDOW_HANDLER.eq(&HWND::default()) {
      EnumWindows(Some(enum_windows_proc), LPARAM(0));
    }

    WORKER_WINDOW_HANDLER
  }
}

fn find_progman_window() -> HWND {
  unsafe { FindWindowW(PCWSTR(PROGMAN.as_ptr()), PCWSTR(PROGMAN_MANAGER.as_ptr())) }
}

unsafe fn find_desktop_handles() -> HWND {
  WORKER_WINDOW_ORIG = HWND::default();
  PROGMAN_WINDOW = HWND::default();

  PROGMAN_WINDOW = find_progman_window();
  FOLDER_VIEW_WINDOW_HANDLER = FindWindowExW(
    PROGMAN_WINDOW,
    HWND::default(),
    PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
    PCWSTR(EMPTY.as_ptr()),
  );

  if FOLDER_VIEW_WINDOW_HANDLER.eq(&HWND::default()) {
    while FOLDER_VIEW_WINDOW_HANDLER.eq(&HWND::default()) && WORKER_WINDOW_ORIG.ne(&HWND::default())
    {
      WORKER_WINDOW_ORIG = FindWindowExW(
        GetDesktopWindow(),
        WORKER_WINDOW_ORIG,
        PCWSTR(WORKER_W.as_ptr()),
        PCWSTR(EMPTY.as_ptr()),
      );
      FOLDER_VIEW_WINDOW_HANDLER = FindWindowExW(
        WORKER_WINDOW_ORIG,
        HWND::default(),
        PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
        PCWSTR(EMPTY.as_ptr()),
      );
    }
  }

  WORKER_WINDOW_ORIG
}

unsafe extern "system" fn enum_windows_proc(h_wnd: HWND, _: LPARAM) -> BOOL {
  let def_view = FindWindowExW(
    h_wnd,
    HWND::default(),
    PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
    PCWSTR(EMPTY.as_ptr()),
  );

  if def_view.ne(&HWND::default()) {
    DEF_VIEW_WINDOW_HANDLER = def_view;
    __WORKER_WINDOW_HANDLER = h_wnd;
    FOLDER_VIEW_WINDOW_HANDLER = FindWindowExW(
      DEF_VIEW_WINDOW_HANDLER,
      HWND::default(),
      PCWSTR(SYS_LIST_VIEW.as_ptr()),
      PCWSTR(FOLDER_VIEW.as_ptr()),
    );
    WORKER_WINDOW_HANDLER = FindWindowExW(
      HWND::default(),
      h_wnd,
      PCWSTR(WORKER_W.as_ptr()),
      PCWSTR(EMPTY.as_ptr()),
    );
    return false.into();
  }

  true.into()
}

pub unsafe fn is_desktop() -> bool {
  let window = GetForegroundWindow();
  window.eq(&find_progman_window()) || window.eq(&WORKER_WINDOW_ORIG)
}

#[cfg(test)]
mod test {
  use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{ShowWindow, SW_HIDE},
  };

  use crate::platform_impl::window::find_desktop_handles;
  use crate::platform_impl::window::find_worker_window;

  #[test]
  fn test_find_worker_window() {
    println!("{:#x}", find_worker_window().0);

    assert!(find_worker_window().ne(&HWND::default()));
  }

  #[test]
  fn test_worker_window() {
    unsafe {
      let win = find_desktop_handles();
      ShowWindow(win, SW_HIDE);
    }
  }
}
