import { platform } from 'node:process'

function acceleratorKeyFromKeyCodeWin(code: number): string {
  // Alt, 164,
  // AltGr, 165,
  // Backspace, 0x08,
  // CapsLock, 20,
  // ControlLeft, 162,
  // ControlRight, 163,
  // Delete, 46,
  // DownArrow, 40,
  // End, 35,
  // Escape, 27,
  // F1, 112,
  // F10, 121,
  // F11, 122,
  // F12, 123,
  // F2, 113,
  // F3, 114,
  // F4, 115,
  // F5, 116,
  // F6, 117,
  // F7, 118,
  // F8, 119,
  // F9, 120,
  // Home, 36,
  // LeftArrow, 37,
  // MetaLeft, 91,
  // PageDown, 34,
  // PageUp, 33,
  // Return, 0x0D,
  // RightArrow, 39,
  // ShiftLeft, 160,
  // ShiftRight, 161,
  // Space, 32,
  // Tab, 0x09,
  // UpArrow, 38,
  // PrintScreen, 44,
  // ScrollLock, 145,
  // Pause, 19,
  // NumLock, 144,
  // BackQuote, 192,
  // Num1, 49,
  // Num2, 50,
  // Num3, 51,
  // Num4, 52,
  // Num5, 53,
  // Num6, 54,
  // Num7, 55,
  // Num8, 56,
  // Num9, 57,
  // Num0, 48,
  // Minus, 189,
  // Equal, 187,
  // KeyQ, 81,
  // KeyW, 87,
  // KeyE, 69,
  // KeyR, 82,
  // KeyT, 84,
  // KeyY, 89,
  // KeyU, 85,
  // KeyI, 73,
  // KeyO, 79,
  // KeyP, 80,
  // LeftBracket, 219,
  // RightBracket, 221,
  // KeyA, 65,
  // KeyS, 83,
  // KeyD, 68,
  // KeyF, 70,
  // KeyG, 71,
  // KeyH, 72,
  // KeyJ, 74,
  // KeyK, 75,
  // KeyL, 76,
  // SemiColon, 186,
  // Quote, 222,
  // BackSlash, 220,
  // IntlBackslash, 226,
  // KeyZ, 90,
  // KeyX, 88,
  // KeyC, 67,
  // KeyV, 86,
  // KeyB, 66,
  // KeyN, 78,
  // KeyM, 77,
  // Comma, 188,
  // Dot, 190,
  // Slash, 191,
  // Insert, 45,
  // //KP_RETURN, 13,
  // KpMinus, 109,
  // KpPlus, 107,
  // KpMultiply, 106,
  // KpDivide, 111,
  // Kp0, 96,
  // Kp1, 97,
  // Kp2, 98,
  // Kp3, 99,
  // Kp4, 100,
  // Kp5, 101,
  // Kp6, 102,
  // Kp7, 103,
  // Kp8, 104,
  // Kp9, 105,
  // KpDelete, 110

  switch (code) {
    case 0x08:
      return 'Backspace'
    case 0x0D:
      return 'Enter'
    case 0x09:
      return 'Tab'
    case 0x1B:
      return 'Escape'
    case 0x20:
      return 'Space'
    default:
      return '[skip]'
  }
}

function acceleratorKeyFromKeyCodeLinux(_code: number): string {
  return '[skip]'
}

function acceleratorKeyFromKeyCodeDarwin(code: number): string {
  switch (code) {
    case 58:
      return 'Alt'
    case 61:
      return 'AltGr'
    case 51:
      return 'Backspace'
    case 57:
      return 'CapsLock'
    case 59:
    case 62:
      return 'Control'
    case 125:
      return 'Down'
    case 53:
      return 'Escape'
    case 122:
      return 'F1'
    case 109:
      return 'F10'
    case 103:
      return 'F11'
    case 111:
      return 'F12'
    case 120:
      return 'F2'
    case 99:
      return 'F3'
    case 118:
      return 'F4'
    case 96:
      return 'F5'
    case 97:
      return 'F6'
    case 98:
      return 'F7'
    case 100:
      return 'F8'
    case 101:
      return 'F9'
    case 63:
      return 'Fn'
    case 123:
      return 'Left'
    case 55:
      return 'Meta'
    case 54:
      return 'Meta'
    case 36:
      return 'Enter'
    case 124:
      return 'Right'
    case 56:
    case 60:
      return 'Shift'
    case 49:
      return 'Space'
    case 48:
      return 'Tab'
    case 126:
      return 'Up'
    case 50:
      return 'Backspace'
    case 18:
      return 'num1'
    case 19:
      return 'num2'
    case 20:
      return 'num3'
    case 21:
      return 'num4'
    case 23:
      return 'num5'
    case 22:
      return 'num6'
    case 26:
      return 'num7'
    case 28:
      return 'num8'
    case 25:
      return 'num9'
    case 29:
      return 'num0'
    case 27:
      return 'numsub'
    case 24:
      return '='
    case 33:
      return '['
    case 30:
      return ']'
    case 12:
      return 'Q'
    case 13:
      return 'W'
    case 14:
      return 'E'
    case 15:
      return 'R'
    case 17:
      return 'T'
    case 16:
      return 'Y'
    case 32:
      return 'U'
    case 34:
      return 'I'
    case 35:
      return 'P'
    case 0:
      return 'A'
    case 1:
      return 'S'
    case 2:
      return 'D'
    case 3:
      return 'F'
    case 5:
      return 'G'
    case 4:
      return 'H'
    case 6:
      return 'Z'
    case 7:
      return 'X'
    case 8:
      return 'C'
    case 9:
      return 'V'
    case 11:
      return 'B'
  }
}

export function acceleratorKeyFromKeyCode(code: number) {
  switch (platform) {
    case 'win32':
      return acceleratorKeyFromKeyCodeWin(code)
    case 'linux':
      return acceleratorKeyFromKeyCodeLinux(code)
    case 'darwin':
      return acceleratorKeyFromKeyCodeDarwin(code)
    default:
      return acceleratorKeyFromKeyCodeWin(code)
  }
}
