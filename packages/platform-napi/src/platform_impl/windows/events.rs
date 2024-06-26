use crate::{
  platform_impl::platform::util::encode_wide,
  util::{InputEvent, ModifiersState, RawEvent},
};
use napi::{bindgen_prelude::*, threadsafe_function::ThreadsafeFunction, JsBigInt, JsFunction};
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM},
    UI::{
      Input::{
        GetRawInputData,
        KeyboardAndMouse::{
          GetKeyState, MapVirtualKeyA, MAPVK_VSC_TO_VK_EX, VIRTUAL_KEY, VK_CONTROL, VK_LCONTROL,
          VK_LMENU, VK_LWIN, VK_MENU, VK_PAUSE, VK_RCONTROL, VK_RMENU, VK_RWIN, VK_SCROLL,
          VK_SHIFT,
        },
        RegisterRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RAWKEYBOARD,
        RAWMOUSE, RIDEV_EXINPUTSINK, RID_INPUT, RIM_TYPEKEYBOARD, RIM_TYPEMOUSE,
      },
      WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetCursorPos,
        GetMessageW, IsWindow, PostMessageW, RegisterClassExW, TranslateMessage, UnregisterClassW,
        CS_HREDRAW, CS_NOCLOSE, CS_OWNDC, CS_VREDRAW, MSG, RI_KEY_E0, RI_KEY_E1,
        RI_MOUSE_LEFT_BUTTON_DOWN, RI_MOUSE_LEFT_BUTTON_UP, RI_MOUSE_RIGHT_BUTTON_DOWN,
        RI_MOUSE_RIGHT_BUTTON_UP, RI_MOUSE_WHEEL, WHEEL_DELTA, WM_INPUT, WM_KEYDOWN, WM_KEYUP,
        WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_SYSKEYDOWN, WM_SYSKEYUP, WNDCLASSEXW,
        WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
        WS_POPUP, WS_VISIBLE,
      },
    },
  },
};

use super::window::is_desktop;

lazy_static! {
  static ref CLASS_NAME: Vec<u16> = encode_wide("NATMRI_INTERACTIVE_MESSAGE");
}

static mut CALLBACK: Option<ThreadsafeFunction<InputEvent>> = None;
static mut WINDOW: Option<HWND> = None;

pub fn setup_interactive_window(window: JsBigInt, callback: Option<JsFunction>) -> Result<()> {
  unsafe {
    if let Some(callback) = callback {
      let callback: ThreadsafeFunction<InputEvent> =
        callback.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;

      CALLBACK = Some(callback);
    }

    if let Ok((window, _)) = window.get_u64() {
      let window = HWND(window as isize);
      if IsWindow(window).as_bool() {
        WINDOW = Some(window);
      }
    }

    std::thread::spawn(move || {
      let mut wcx = WNDCLASSEXW::default();
      wcx.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
      wcx.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC | CS_NOCLOSE;
      wcx.lpszClassName = PCWSTR(CLASS_NAME.as_ptr());
      wcx.lpfnWndProc = Some(window_proc);

      RegisterClassExW(&wcx);

      let h_wnd = CreateWindowExW(
        WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE,
        PCWSTR(CLASS_NAME.as_ptr()),
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

      let mut mouse_raw_input_device = RAWINPUTDEVICE::default();
      let mut keyboard_raw_input_device = RAWINPUTDEVICE::default();

      mouse_raw_input_device.usUsagePage = 0x01;
      mouse_raw_input_device.usUsage = 0x06;
      mouse_raw_input_device.dwFlags = RIDEV_EXINPUTSINK;
      mouse_raw_input_device.hwndTarget = h_wnd;

      keyboard_raw_input_device.usUsagePage = 0x01;
      keyboard_raw_input_device.usUsage = 0x02;
      keyboard_raw_input_device.dwFlags = RIDEV_EXINPUTSINK;
      keyboard_raw_input_device.hwndTarget = h_wnd;

      RegisterRawInputDevices(
        &[mouse_raw_input_device, keyboard_raw_input_device],
        std::mem::size_of::<RAWINPUTDEVICE>() as u32,
      );

      let mut msg = MSG::default();

      while GetMessageW(&mut msg, h_wnd, 0, 0).into() {
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);
      }

      // clear
      DestroyWindow(h_wnd);
      UnregisterClassW(wcx.lpszClassName, wcx.hInstance);
    });
  }

  Ok(())
}

pub fn restore_interactive_window() {
  unsafe {
    if CALLBACK.is_some() {
      CALLBACK = None;
    }
  }
}

unsafe fn setup_keybroad_events(raw_keybroad: RAWKEYBOARD) {
  let pressed = raw_keybroad.Message == WM_KEYDOWN || raw_keybroad.Message == WM_SYSKEYDOWN;
  let released = raw_keybroad.Message == WM_KEYUP || raw_keybroad.Message == WM_SYSKEYUP;

  if pressed || released {
    let scancode = raw_keybroad.MakeCode;
    let extended = crate::util::has_flag(raw_keybroad.Flags, RI_KEY_E0 as u16)
      | crate::util::has_flag(raw_keybroad.Flags, RI_KEY_E1 as u16);
    let kind = if pressed {
      RawEvent::KeyDown
    } else {
      RawEvent::KeyUp
    };

    if let Some((vkey, scancode)) =
      handle_extended_keys(VIRTUAL_KEY(raw_keybroad.VKey), scancode as u32, extended)
    {
      let virtual_keycode = vkey;

      if let Some(callback) = CALLBACK.clone() {
        callback.call(
          Ok(InputEvent {
            kind,
            scancode,
            virtual_keycode: virtual_keycode.0 as u32,
            modifiers: get_key_mods().bits(),
            ..Default::default()
          }),
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      }

      if is_desktop() {
        if let Some(window) = WINDOW {
          let mut lparam = raw_keybroad.MakeCode as u32;
          let mut lparam = lparam.reverse_bits();
          lparam |= 1_u32 | lparam;
          lparam |= 1_u32 << 24;
          lparam |= 0_u32 << 29;
          if !pressed {
            lparam |= 3_u32 << 30;
          }

          PostMessageW(
            window,
            raw_keybroad.Message,
            WPARAM(raw_keybroad.VKey as usize),
            LPARAM(lparam as isize),
          );
        }
      }
    }
  }
}

unsafe fn setup_mouse_events(raw_mouse: RAWMOUSE) {
  let mut p = POINT::default();

  if !GetCursorPos(&mut p).as_bool() {
    return;
  }

  if let Some(callback) = CALLBACK.clone() {
    match raw_mouse.Anonymous.Anonymous.usButtonFlags as u32 {
      // handle mouse left button
      RI_MOUSE_LEFT_BUTTON_DOWN => {
        callback.call(
          Ok(InputEvent {
            kind: RawEvent::MouseDown,
            x: p.x,
            y: p.y,
            ..Default::default()
          }),
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
      RI_MOUSE_LEFT_BUTTON_UP => {
        callback.call(
          Ok(InputEvent {
            kind: RawEvent::MouseUp,
            x: p.x,
            y: p.y,
            ..Default::default()
          }),
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
      // handle mouse middle button
      RI_MOUSE_WHEEL => {
        let delta = raw_mouse.Anonymous.Anonymous.usButtonData as i16 as f32 / WHEEL_DELTA as f32;

        callback.call(
          Ok(InputEvent {
            kind: RawEvent::MouseWheel,
            x: p.x,
            y: p.y,
            delta: delta as i32,
            ..Default::default()
          }),
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
      _ => {
        callback.call(
          Ok(InputEvent {
            kind: RawEvent::MouseMove,
            x: p.x,
            y: p.y,
            ..Default::default()
          }),
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      }
    }
  }

  if is_desktop() {
    if let Some(window) = WINDOW {
      match raw_mouse.Anonymous.Anonymous.usButtonFlags as u32 {
        RI_MOUSE_LEFT_BUTTON_UP => {
          PostMessageW(
            window,
            WM_LBUTTONUP,
            WPARAM(0x0001),
            LPARAM((p.y << 16 | p.x) as isize),
          );
        }
        RI_MOUSE_LEFT_BUTTON_DOWN => {
          PostMessageW(
            window,
            WM_LBUTTONDOWN,
            WPARAM(0x0001),
            LPARAM((p.y << 16 | p.x) as isize),
          );
        }
        RI_MOUSE_WHEEL => {}
        RI_MOUSE_RIGHT_BUTTON_DOWN => {}
        RI_MOUSE_RIGHT_BUTTON_UP => {}
        _ => {
          PostMessageW(
            window,
            WM_MOUSEMOVE,
            WPARAM(0x0020),
            LPARAM((p.y << 16 | p.x) as isize),
          );
        }
      }
    }
  }
}

fn key_pressed(vkey: VIRTUAL_KEY) -> bool {
  unsafe { crate::util::has_flag(GetKeyState(vkey.0 as i32), 1 << 15) }
}

fn get_key_mods() -> ModifiersState {
  let filter_out_altgr = key_pressed(VK_RMENU);

  let mut mods = ModifiersState::empty();
  mods.set(ModifiersState::SHIFT, key_pressed(VK_SHIFT));
  mods.set(
    ModifiersState::CTRL,
    key_pressed(VK_CONTROL) && !filter_out_altgr,
  );
  mods.set(
    ModifiersState::ALT,
    key_pressed(VK_MENU) && !filter_out_altgr,
  );
  mods.set(
    ModifiersState::LOGO,
    key_pressed(VK_LWIN) || key_pressed(VK_RWIN),
  );
  mods
}

fn get_raw_input_data(handle: HRAWINPUT) -> Option<RAWINPUT> {
  let mut data: RAWINPUT = unsafe { std::mem::zeroed() };
  let mut data_size = std::mem::size_of::<RAWINPUT>() as u32;
  let header_size = std::mem::size_of::<RAWINPUTHEADER>() as u32;

  let status = unsafe {
    GetRawInputData(
      handle,
      RID_INPUT,
      Some(&mut data as *mut _ as _),
      &mut data_size,
      header_size,
    )
  };

  if status == u32::MAX || status == 0 {
    return None;
  }

  Some(data)
}

fn handle_extended_keys(
  vkey: VIRTUAL_KEY,
  mut scancode: u32,
  extended: bool,
) -> Option<(VIRTUAL_KEY, u32)> {
  // Welcome to hell https://blog.molecular-matters.com/2011/09/05/properly-handling-keyboard-input/
  scancode |= if extended { 0xE000 } else { 0x0000 };
  let vkey: VIRTUAL_KEY = match vkey {
    VK_SHIFT => unsafe { VIRTUAL_KEY(MapVirtualKeyA(scancode, MAPVK_VSC_TO_VK_EX) as u16) },
    VK_CONTROL => {
      if extended {
        VK_RCONTROL
      } else {
        VK_LCONTROL
      }
    }
    VK_MENU => {
      if extended {
        VK_RMENU
      } else {
        VK_LMENU
      }
    }
    _ => {
      match scancode {
        // When VK_PAUSE is pressed it emits a LeftControl + NumLock scancode event sequence, but reports VK_PAUSE
        // as the virtual key on both events, or VK_PAUSE on the first event or 0xFF when using raw input.
        // Don't emit anything for the LeftControl event in the pair...
        0xE01D if vkey == VK_PAUSE => return None,
        // ...and emit the Pause event for the second event in the pair.
        0x45 if vkey == VK_PAUSE || vkey.eq(&VIRTUAL_KEY(0xFF)) => {
          scancode = 0xE059;
          VK_PAUSE
        }
        // VK_PAUSE has an incorrect vkey value when used with modifiers. VK_PAUSE also reports a different
        // scancode when used with modifiers than when used without
        0xE046 => {
          scancode = 0xE059;
          VK_PAUSE
        }
        // VK_SCROLL has an incorrect vkey value when used with modifiers.
        0x46 => VK_SCROLL,
        _ => vkey,
      }
    }
  };
  Some((vkey, scancode))
}

unsafe extern "system" fn window_proc(
  h_wnd: HWND,
  msg: u32,
  w_param: WPARAM,
  l_param: LPARAM,
) -> LRESULT {
  match msg {
    WM_INPUT => {
      if let Some(data) = get_raw_input_data(HRAWINPUT(l_param.0)) {
        if data.header.dwType.eq(&RIM_TYPEMOUSE.0) {
          setup_mouse_events(data.data.mouse);
        } else if data.header.dwType.eq(&RIM_TYPEKEYBOARD.0) {
          setup_keybroad_events(data.data.keyboard);
        }
      }

      LRESULT(1)
    }
    _ => DefWindowProcW(h_wnd, msg, w_param, l_param),
  }
}
