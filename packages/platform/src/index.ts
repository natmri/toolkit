import { RawEvent, setupInteractiveWindow } from '@natmri/platform-napi'
import type { BrowserWindow } from 'electron'

export { setMainWindowHandle, releaseShutdownBlock, acquireShutdownBlock, insertWndProcHook } from '@natmri/platform-napi'
export { restoreInteractiveWindow as destroyInteractiveWindow } from '@natmri/platform-napi'

export function createInteractiveWindow(window: BrowserWindow) {
  setupInteractiveWindow(window.getNativeWindowHandle().readBigUInt64LE(), (err, event) => {
    // ignore errors
    if (!err)
      return

    switch (event.kind) {
      case RawEvent.KeyDown:
        window.webContents.sendInputEvent({
          type: 'keyDown',
          // TODO: correct convert key code
          keyCode: String.fromCharCode(event.virtualKeycode),
        })
        break
      case RawEvent.KeyUp:
        window.webContents.sendInputEvent({
          type: 'keyUp',
          // TODO: correct convert key code
          keyCode: String.fromCharCode(event.virtualKeycode),
        })
        break
      case RawEvent.MouseMove:
        window.webContents.sendInputEvent({
          type: 'mouseMove',
          x: event.x,
          y: event.y,
        })
        break
      case RawEvent.MouseDown:
        window.webContents.sendInputEvent({
          type: 'mouseDown',
          x: event.x,
          y: event.y,
        })
        break
      case RawEvent.MouseUp:
        window.webContents.sendInputEvent({
          type: 'mouseUp',
          x: event.x,
          y: event.y,
        })
        break
      case RawEvent.MouseWheel:
        window.webContents.sendInputEvent({
          type: 'mouseWheel',
          deltaX: event.delta,
          x: event.x,
          y: event.y,
        })
        break
      default:
        break
    }
  })
}
