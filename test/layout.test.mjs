import assert from 'node:assert/strict'
import { describe, it } from 'node:test'
import { litKeyIds, k3KeyIds } from '../web/js/layout.mjs'

describe('litKeyIds', () => {
  it('lights a and left shift from a boot report', () => {
    const ids = litKeyIds({
      keys: ['a'],
      modifiers: ['lshift'],
    })
    assert.deepEqual(ids.sort(), ['a', 'lshift'])
  })

  it('lights nothing on empty report', () => {
    assert.deepEqual(litKeyIds({ keys: [], modifiers: [] }), [])
  })
})

describe('k3KeyIds', () => {
  it('includes esc y h enter 1 f1 for the probe set', () => {
    const ids = k3KeyIds()
    for (const id of ['esc', 'y', 'h', 'enter', '1', 'f1']) {
      assert.ok(ids.includes(id), `falta ${id}`)
    }
  })
})
