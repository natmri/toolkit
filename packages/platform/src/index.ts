import type { BrowserWindow } from 'electron'
import type { InputEvent } from '@natmri/platform-napi'
import { setupInteractiveWindow } from '@natmri/platform-napi'
export { setMainWindowHandle, releaseShutdownBlock, acquireShutdownBlock, insertWndProcHook, removeWndProcHook } from '@natmri/platform-napi'
export { restoreInteractiveWindow as destroyInteractiveWindow } from '@natmri/platform-napi'
export type { InputEvent, RawEvent } from '@natmri/platform-napi'

export function createInteractiveWindow(window: BrowserWindow, callback?: (err: Error | null, event: InputEvent) => void): void {
  const windowHandle = window.getNativeWindowHandle().readBigUint64LE()

  setupInteractiveWindow(windowHandle, callback)
}
