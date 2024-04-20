import process from 'node:process'
import type { BrowserWindow } from 'electron'
import { RawEvent, setupInteractiveWindow } from '@natmri/platform-napi'
import { acceleratorKeyFromKeyCode } from './keycodes'

export { powerMonitor } from './power'
export { restoreInteractiveWindow as destroyInteractiveWindow } from '@natmri/platform-napi'
export { getDesktopIconVisibility, setDesktopIconVisibility, isDesktop } from '@natmri/platform-napi'
export type * from '@natmri/platform-napi'

export function createInteractiveWindow(window: BrowserWindow): void {
  const windowHandle = window.getNativeWindowHandle().readBigUint64LE()
  let lastX: number
  let lastY: number
  setupInteractiveWindow(windowHandle, (err, event) => {
    if (err || process.platform === 'win32')
      return

    switch (event.kind) {
      case RawEvent.KeyDown:
        window.webContents.sendInputEvent({
          type: 'keyDown',
          keyCode: acceleratorKeyFromKeyCode(event.virtualKeycode),
        })
        break
      case RawEvent.KeyUp:
        window.webContents.sendInputEvent({
          type: 'keyUp',
          keyCode: acceleratorKeyFromKeyCode(event.virtualKeycode),
        })
        break
      case RawEvent.MouseMove:
        lastX = event.x
        lastY = event.y
        window.webContents.sendInputEvent({
          type: 'mouseMove',
          x: event.x,
          y: event.y,
        })
        break
      case RawEvent.MouseDown:
        window.webContents.sendInputEvent({
          type: 'mouseDown',
          x: lastX,
          y: lastY,
        })
        break
      case RawEvent.MouseUp:
        window.webContents.sendInputEvent({
          type: 'mouseUp',
          x: lastX,
          y: lastY,
        })
        break
      case RawEvent.MouseWheel:
        window.webContents.sendInputEvent({
          type: 'mouseWheel',
          deltaX: event.delta,
          deltaY: event.delta,
          x: lastX,
          y: lastY,
        })
        break
    }
  })
}
