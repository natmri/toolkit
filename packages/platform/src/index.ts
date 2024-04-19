import process from 'node:process'
import type { BrowserWindow } from 'electron'
import type { InputEvent } from '@natmri/platform-napi'
import { RawEvent, createShutdownBlocker, destroyShutdownBlocker, setupInteractiveWindow } from '@natmri/platform-napi'
import { powerMonitor as _powerMonitor } from 'electron'

export { restoreInteractiveWindow as destroyInteractiveWindow } from '@natmri/platform-napi'
export { getDesktopIconVisibility, setDesktopIconVisibility, isDesktop } from '@natmri/platform-napi'
export type * from '@natmri/platform-napi'

export type Nil = undefined | void | null
export type EventKey = string
export type Listener<O extends Record<EventKey, any>, K extends keyof O, V = O[K]> =
  V extends Array<any>
    ? V extends [infer Arg] ? (arg1: Arg) => void | Promise<void>
      : V extends [infer Arg1, infer Arg2] ? (arg1: Arg1, arg2: Arg2) => void | Promise<void>
        : V extends [infer Arg1, infer Arg2, infer Arg3] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3) => void | Promise<void>
          : V extends [infer Arg1, infer Arg2, infer Arg3, infer Arg4] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4) => void | Promise<void>
            : V extends [infer Arg1, infer Arg2, infer Arg3, infer Arg4, infer Arg5] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5) => void | Promise<void>
              : V extends [infer Arg1, infer Arg2, infer Arg3, infer Arg4, infer Arg5, infer Arg6] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6) => void | Promise<void>
                : V extends [infer Arg1, infer Arg2, infer Arg3, infer Arg4, infer Arg5, infer Arg6, infer Arg7] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7) => void | Promise<void>
                  : V extends [infer Arg1, infer Arg2, infer Arg3, infer Arg4, infer Arg5, infer Arg6, infer Arg7, infer Arg8] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8) => void | Promise<void>
                    : V extends [infer Arg1, infer Arg2, infer Arg3, infer Arg4, infer Arg5, infer Arg6, infer Arg7, infer Arg8, infer Arg9] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9) => void | Promise<void>
                      : V extends [infer Arg1, infer Arg2, infer Arg3, infer Arg4, infer Arg5, infer Arg6, infer Arg7, infer Arg8, infer Arg9, infer Arg10] ? (arg1: Arg1, arg2: Arg2, arg3: Arg3, arg4: Arg4, arg5: Arg5, arg6: Arg6, arg7: Arg7, arg8: Arg8, arg9: Arg9, arg10: Arg10) => void | Promise<void>
                        : (...args: any[]) => void | Promise<void>
    : V extends Nil
      ? () => void | Promise<void>
      : (arg: V) => void | Promise<void>

type PowerMonitorEvents = 'suspend' | 'resume' | 'on-ac' | 'on-battery' | 'shutdown' | 'lock-screen' | 'unlock-screen' | 'user-did-groupe-active' | 'user-did-resign-active'

type PowerMonitorEvent = {
  [e in PowerMonitorEvents]: Function
}

/**
 * Native electron not support 'shutdown' event for windows
 *
 * The PowerMonitor patch native electron support 'shutdown' event for windows
 */
export class PowerMonitor {
  static SHUT_DOWN_REASON = 'Please wait for some data to be saved'

  private init = false
  private readonly eventListeners: Map<keyof PowerMonitorEvent, Listener<PowerMonitorEvent, any>[]> = new Map()

  get eventNames() {
    return this.eventListeners.keys()
  }

  private onShutdown(listener: Listener<PowerMonitorEvent, any>, prepend = false) {
    const result = this.eventListeners.get('shutdown') ?? []
    if (prepend)
      result.push(listener)
    else
      result.unshift(listener)

    this.eventListeners.set('shutdown', result)
    if (!this.init) {
      createShutdownBlocker(PowerMonitor.SHUT_DOWN_REASON, () => {
        // @ts-expect-error void
        (this.eventListeners.get('shutdown') ?? [])!.forEach(l => l())
      })

      this.init = true
    }
  }

  private offShutdown(listener?: Function) {
    if (!this.init)
      return

    destroyShutdownBlocker()

    if (!listener)
      return this.eventListeners.delete('shutdown')

    if (this.eventListeners.has('shutdown'))
      this.eventListeners.set('shutdown', (this.eventListeners.get('shutdown') ?? []).filter(l => l !== listener))
  }

  on<N extends keyof PowerMonitorEvent>(eventName: N, listener: Listener<PowerMonitorEvent, N>) {
    if (eventName === 'shutdown' && process.platform === 'win32') {
      this.onShutdown(listener)
      return this
    }

    // @ts-expect-error event name
    _powerMonitor.on(eventName, listener)

    return this
  }

  once<N extends keyof PowerMonitorEvent>(eventName: N, listener: Listener<PowerMonitorEvent, N>) {
    const _listener = async (...args: any[]) => {
      // @ts-expect-error rest parameter allow
      await listener(...args)
      // @ts-expect-error rest parameter allow
      this.off(eventName, _listener)
    }
    // @ts-expect-error rest parameter allow
    this.on(eventName, _listener)
    return this
  }

  addListener<N extends keyof PowerMonitorEvent>(eventName: N, listener: Listener<PowerMonitorEvent, N>) {
    this.on(eventName, listener)
    return this
  }

  off<N extends keyof PowerMonitorEvent>(eventName: N, listener: Listener<PowerMonitorEvent, N>): this {
    if (eventName === 'shutdown' && process.platform === 'win32') {
      this.offShutdown(listener)
      return this
    }
    _powerMonitor.off(eventName, listener)
    return this
  }

  emit<N extends keyof PowerMonitorEvent>(eventName: N, ...args: Parameters<Listener<PowerMonitorEvent, N>>) {
    for (const callback of (this.eventListeners.get(eventName) ?? [])) {
      // @ts-expect-error rest parameter allow
      callback(...args)
      // @ts-expect-error promise
        ?.then(() => { /* ignore void */ })
        ?.catch(() => { /* ignore error */ })
    }
    return this
  }

  removeListener<N extends keyof PowerMonitorEvent>(eventName: N, listener: Listener<PowerMonitorEvent, N>): this {
    return this.off(eventName, listener)
  }

  prependListener<N extends keyof PowerMonitorEvent>(eventName: N, listener: Listener<PowerMonitorEvent, N>): this {
    if (eventName === 'shutdown' && process.platform === 'win32') {
      this.onShutdown(listener, true)
      return this
    }

    _powerMonitor.prependListener(eventName, listener)

    return this
  }

  removeAllListeners<N extends keyof PowerMonitorEvent>(eventName?: N) {
    if (eventName === 'shutdown' && process.platform === 'win32') {
      this.offShutdown()
    }
    else if (!eventName) {
      this.eventListeners.clear()
      _powerMonitor.removeAllListeners()
    }
    else {
      _powerMonitor.removeAllListeners(eventName)
    }

    return this
  }

  listenerCount<N extends keyof PowerMonitorEvent>(eventName?: N): number {
    if (eventName === 'shutdown' && process.platform === 'win32')
      return this.eventListeners.get('shutdown')?.length ?? 0
    else if (!eventName)
      return (this.eventListeners.get('shutdown')?.length ?? 0) + _powerMonitor.listenerCount(eventName)
    else
      return _powerMonitor.listenerCount(eventName)
  }
}

export const powerMonitor = new PowerMonitor()

export function createInteractiveWindow(window: BrowserWindow): void {
  const windowHandle = window.getNativeWindowHandle().readBigUint64LE()
  let lastX: number
  let lastY: number
  setupInteractiveWindow(windowHandle, (err, event) => {
    if (err)
      return

    switch (event.kind) {
      case RawEvent.KeyDown:
        window.webContents.sendInputEvent({
          type: 'keyDown',
          keyCode: 'A',
        })
        break
      case RawEvent.KeyUp:
        window.webContents.sendInputEvent({
          type: 'keyUp',
          keyCode: 'A',
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
          x: event.x,
          y: event.y,
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
