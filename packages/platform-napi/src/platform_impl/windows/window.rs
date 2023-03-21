use core::ops::{BitOr, BitXor};

use napi::JsBigInt;
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::{BOOL, HWND, LPARAM, WPARAM},
    UI::{
      Shell::{SHGetSetSettings, SHELLSTATEA, SSF_HIDEICONS},
      WindowsAndMessaging::{
        EnumWindows, FindWindowExW, FindWindowW, GetDesktopWindow, GetForegroundWindow,
        SendMessageTimeoutW, SendMessageW, SetParent, ShowWindow, SystemParametersInfoW,
        SMTO_NORMAL, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER, SW_HIDE, SW_SHOW, WNDENUMPROC,
      },
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
static mut WORKER_WINDOW: HWND = HWND(0);
static mut WORKER_ORIG_WINDOW: HWND = HWND(0);
static mut INITIALIZED: bool = false;

pub fn setup_interactive_parent_window(bigint: JsBigInt) {
  unsafe {
    if !INITIALIZED {
      initialize();
    }

    if WORKER_WINDOW.eq(&HWND::default()) {
      find_desktop_handles();
    }

    if let Ok((h_wnd, _)) = bigint.get_u64() {
      set_parent_workerw(HWND(h_wnd as isize));

      ShowWindow(WORKER_WINDOW, SW_SHOW);
    }
  }
}

pub fn restore_interactive_parent_window() {
  unsafe {
    if !INITIALIZED {
      return;
    }

    SystemParametersInfoW(SPI_SETDESKWALLPAPER, 0, None, SPIF_UPDATEINIFILE);
    ShowWindow(WORKER_WINDOW, SW_SHOW);
  }
}

pub fn get_desktop_icon_visibility() -> bool {
  let mut state = SHELLSTATEA::default();

  unsafe {
    SHGetSetSettings(Some(&mut state), SSF_HIDEICONS, BOOL(0));
  }

  state._bitfield1 & 0x00001000_i32 == 0x00001000_i32
}

pub fn set_desktop_icon_visibility(isVisible: bool) {
  unsafe {
    if get_desktop_icon_visibility().bitxor(isVisible) {
      SendMessageW(
        find_desktop_shell_dll_def_view(),
        0x0111,
        WPARAM(0x7402),
        LPARAM::default(),
      );
    }
  }
}

unsafe fn find_desktop_shell_dll_def_view() -> HWND {
  let mut hShellViewWin = HWND::default();
  let mut hWorkerW = HWND::default();

  let hProgman = find_progman_window();
  let hDesktopWnd = GetDesktopWindow();

  if hProgman.ne(&HWND::default()) {
    hShellViewWin = FindWindowExW(
      hProgman,
      HWND::default(),
      PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
      PCWSTR(EMPTY.as_ptr()),
    );

    if hShellViewWin.eq(&HWND::default()) {
      loop {
        hWorkerW = FindWindowExW(
          hDesktopWnd,
          hWorkerW,
          PCWSTR(WORKER_W.as_ptr()),
          PCWSTR(EMPTY.as_ptr()),
        );
        hShellViewWin = FindWindowExW(
          hWorkerW,
          HWND::default(),
          PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
          PCWSTR(EMPTY.as_ptr()),
        );

        if !(hShellViewWin.eq(&HWND::default()) && hWorkerW.ne(&HWND::default())) {
          break;
        }
      }
    }
  }

  hShellViewWin
}

unsafe fn initialize() {
  PROGMAN_WINDOW = find_progman_window();

  SendMessageTimeoutW(
    PROGMAN_WINDOW,
    0x052C,
    WPARAM(0xD),
    LPARAM(0x1),
    SMTO_NORMAL,
    1000,
    None,
  );

  EnumWindows(Some(window_proc), LPARAM::default());

  INITIALIZED = true;
}

unsafe fn set_parent_workerw(window_handle: HWND) {
  let ver = os_info::get().version().to_string();
  let mut iter = ver.trim().split_terminator('.').fuse();
  let major: u64 = iter.next().and_then(|s| s.parse().ok()).unwrap();
  let minor: u64 = iter.next().unwrap_or("0").parse().ok().unwrap();
  let patch: u64 = iter.next().unwrap_or("0").parse().ok().unwrap();

  // Legacy, Windows 7
  if major == 6 && minor == 1 {
    if WORKER_WINDOW.ne(&PROGMAN_WINDOW) {
      ShowWindow(WORKER_WINDOW, SW_HIDE);
    }

    let ret = SetParent(window_handle, PROGMAN_WINDOW);
    if ret.eq(&HWND::default()) {
      // ... error handling
      eprintln!("Failed to set window parent")
    }
    WORKER_WINDOW = PROGMAN_WINDOW;
  } else {
    let ret = SetParent(window_handle, WORKER_WINDOW);
    if ret.eq(&HWND::default()) {
      // ... error handling
      eprintln!("Failed to set window parent");
    }
  }
}

fn find_progman_window() -> HWND {
  unsafe { FindWindowW(PCWSTR(PROGMAN.as_ptr()), PCWSTR(PROGMAN_MANAGER.as_ptr())) }
}

unsafe fn find_desktop_handles() -> HWND {
  if WORKER_ORIG_WINDOW.ne(&HWND::default()) {
    return WORKER_ORIG_WINDOW;
  }

  WORKER_ORIG_WINDOW = HWND::default();
  PROGMAN_WINDOW = HWND::default();

  PROGMAN_WINDOW = find_progman_window();
  let mut folder_view = FindWindowExW(
    PROGMAN_WINDOW,
    HWND::default(),
    PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
    PCWSTR(EMPTY.as_ptr()),
  );

  SendMessageTimeoutW(
    PROGMAN_WINDOW,
    0x052C,
    WPARAM(0xD),
    LPARAM(0x1),
    SMTO_NORMAL,
    1000,
    None,
  );

  if folder_view.eq(&HWND::default()) {
    loop {
      WORKER_ORIG_WINDOW = FindWindowExW(
        GetDesktopWindow(),
        WORKER_ORIG_WINDOW,
        PCWSTR(WORKER_W.as_ptr()),
        PCWSTR(EMPTY.as_ptr()),
      );
      folder_view = FindWindowExW(
        WORKER_ORIG_WINDOW,
        HWND::default(),
        PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
        PCWSTR(EMPTY.as_ptr()),
      );

      if !(folder_view.eq(&HWND::default()) && WORKER_ORIG_WINDOW.ne(&HWND::default())) {
        break;
      }
    }
  }

  WORKER_ORIG_WINDOW
}

unsafe extern "system" fn window_proc(tophandle: HWND, topparamhanle: LPARAM) -> BOOL {
  let p = FindWindowExW(
    tophandle,
    HWND::default(),
    PCWSTR(SHELL_DLL_DEF_VIEW.as_ptr()),
    PCWSTR(EMPTY.as_ptr()),
  );

  if p.ne(&HWND::default()) {
    // Gets the WorkerW window after the current one.
    WORKER_WINDOW = FindWindowExW(
      HWND::default(),
      tophandle,
      PCWSTR(WORKER_W.as_ptr()),
      PCWSTR(EMPTY.as_ptr()),
    );
  }

  true.into()
}

pub unsafe fn is_desktop() -> bool {
  let window = GetForegroundWindow();
  window.eq(&find_progman_window()) || window.eq(&find_desktop_handles())
}

#[cfg(test)]
mod test {
  use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageW, ShowWindow, SW_HIDE},
  };

  use crate::platform_impl::window::{
    find_desktop_handles, find_desktop_shell_dll_def_view, get_desktop_icon_visibility, initialize,
    WORKER_WINDOW,
  };

  #[test]
  fn test_find_worker_window() {
    let ver = os_info::get().version().to_string();
    let mut iter = ver.trim().split_terminator('.').fuse();

    let major: u64 = iter.next().and_then(|s| s.parse().ok()).unwrap();
    let minor: u64 = iter.next().unwrap_or("0").parse().ok().unwrap();
    let patch: u64 = iter.next().unwrap_or("0").parse().ok().unwrap();

    println!("major {major} minor {minor} patch {patch}");
  }

  #[test]
  fn test_worker_window() {
    unsafe {
      println!("{:?}", find_desktop_shell_dll_def_view());
      println!("{:?}", get_desktop_icon_visibility());

      // SendMessageW(
      //   find_desktop_shell_dll_def_view(),
      //   0x0111,
      //   WPARAM(0x7402),
      //   LPARAM::default(),
      // );

      // let win = find_desktop_handles();
      // println!("{:#x}", win.0);
      // initialize();
      // println!("{:#x}", WORKER_WINDOW.0);

      // assert!(win.ne(&HWND::default()));
    }
  }
}
