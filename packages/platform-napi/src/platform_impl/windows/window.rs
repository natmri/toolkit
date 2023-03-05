use napi::JsBigInt;
use wchar::{wchar_t, wchz};
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::{BOOL, HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{
      EnumWindows, FindWindowExW, FindWindowW, SendMessageTimeoutW, SetParent, ShowWindow,
      SMTO_NORMAL, SW_HIDE, SW_SHOW,
    },
  },
};

const PROGMAN: &[wchar_t] = wchz!("Progman");
const PROGMAN_MANAGER: &[wchar_t] = wchz!("Program Manager");
const SHELL_DLL_DEF_VIEW: &[wchar_t] = wchz!("SHELLDLL_DefView");
const SYS_LIST_VIEW: &[wchar_t] = wchz!("SysListView32");
const FOLDER_VIEW: &[wchar_t] = wchz!("FolderView");
const WORKER_W: &[wchar_t] = wchz!("WorkerW");
const EMPTY: &[wchar_t] = wchz!("");

static mut WORKER_WINDOW_HANDLER: HWND = HWND(0);
static mut DEF_VIEW_WINDOW_HANDLER: HWND = HWND(0);
static mut __WORKER_WINDOW_HANDLER: HWND = HWND(0);
static mut FOLDER_VIEW_WINDOW_HANDLER: HWND = HWND(0);

pub unsafe fn setup_interactive_parent_window(bigint: JsBigInt) {
  if WORKER_WINDOW_HANDLER.eq(&HWND::default()) {
    WORKER_WINDOW_HANDLER = find_worker_window();

    // window 7 support
    if WORKER_WINDOW_HANDLER.eq(&HWND::default()) {
      WORKER_WINDOW_HANDLER = find_progman_window();
    }
  }

  if let Ok((h_wnd, _)) = bigint.get_u64() {
    SetParent(HWND(h_wnd as isize), WORKER_WINDOW_HANDLER);

    ShowWindow(WORKER_WINDOW_HANDLER, SW_SHOW);
  }
}

pub unsafe fn restore_interactive_parent_window() {
  if WORKER_WINDOW_HANDLER.0 == 0 {
    find_worker_window();
  }

  ShowWindow(WORKER_WINDOW_HANDLER, SW_HIDE);
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
