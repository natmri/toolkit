import { EventEmitter } from 'events'
import { setupInteractiveWindow } from '../index'

const events = new EventEmitter()

events.on('input', (event) => {
  console.log('input-event', event)
})

setupInteractiveWindow(BigInt(0), (err, event) => {
  if (!err) {
    events.emit('input', {
      ...event,
      delta: event.delta * 120,
    })
  }
})
