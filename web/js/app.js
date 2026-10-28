import { K3_LAYOUT, litKeyIds } from './layout.mjs'

const PROBE = new Set(['y', 'h', 'enter', '1', 'f1'])
const board = document.getElementById('board')
const hexEl = document.getElementById('hex')
const slotsEl = document.getElementById('slots')
const keysEl = document.getElementById('keys')
const statusEl = document.getElementById('status')
const pathEl = document.getElementById('log-path')
const tailEl = document.getElementById('tail')
const lines = []

function paintBoard() {
  board.replaceChildren()
  for (const row of K3_LAYOUT) {
    const rowEl = document.createElement('div')
    rowEl.className = 'row'
    for (const key of row) {
      const btn = document.createElement('button')
      btn.type = 'button'
      btn.className = 'key'
      btn.dataset.id = key.id
      btn.textContent = key.label
      btn.style.setProperty('--w', String(key.wide ?? 1))
      if (PROBE.has(key.id)) btn.classList.add('probe')
      rowEl.append(btn)
    }
    board.append(rowEl)
  }
}

function setLit(ids) {
  const on = new Set(ids)
  for (const btn of board.querySelectorAll('.key')) {
    btn.classList.toggle('on', on.has(btn.dataset.id))
  }
}

function pushTail(text) {
  lines.push(text)
  if (lines.length > 24) lines.shift()
  tailEl.textContent = lines.join('\n')
  tailEl.scrollTop = tailEl.scrollHeight
}

function onEvent(payload) {
  if (payload.kind === 'error') {
    statusEl.textContent = payload.message
    pushTail(`ERR ${payload.message}`)
    return
  }
  if (payload.kind === 'inventory') {
    statusEl.textContent = `${payload.devices?.length ?? 0} HID`
    pushTail(`INV ${JSON.stringify(payload.devices)}`)
    return
  }
  if (payload.kind === 'open') {
    statusEl.textContent = `open ${payload.path}`
    pushTail(`OPEN ${payload.path}`)
    return
  }
  if (payload.kind === 'report' || payload.kind === 'raw') {
    hexEl.textContent = payload.hex || '—'
    slotsEl.textContent = `${payload.slots_used ?? 0}/6`
    const names = [...(payload.modifiers ?? []), ...(payload.keys ?? [])]
    keysEl.textContent = names.length ? names.join(' ') : 'solto'
    setLit(litKeyIds(payload))
    pushTail(`${payload.kind} ${payload.hex} · ${names.join(',') || 'solto'}`)
  }
}

paintBoard()

const tauri = window.__TAURI__
if (!tauri) {
  statusEl.textContent = 'abra com npm run dev'
} else {
  const { invoke } = tauri.core
  const { listen } = tauri.event
  invoke('log_path').then((path) => {
    pathEl.textContent = path
  })
  listen('hid-event', (event) => onEvent(event.payload))
  statusEl.textContent = 'sonda no ar'
}
