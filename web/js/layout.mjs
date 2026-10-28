/** Keychron K3 Pro ANSI 84: id === HID usage name from boot_report.rs */
export const K3_LAYOUT = [
  [
    { id: 'esc', label: 'Esc' },
    { id: 'f1', label: 'F1' },
    { id: 'f2', label: 'F2' },
    { id: 'f3', label: 'F3' },
    { id: 'f4', label: 'F4' },
    { id: 'f5', label: 'F5' },
    { id: 'f6', label: 'F6' },
    { id: 'f7', label: 'F7' },
    { id: 'f8', label: 'F8' },
    { id: 'f9', label: 'F9' },
    { id: 'f10', label: 'F10' },
    { id: 'f11', label: 'F11' },
    { id: 'f12', label: 'F12' },
    { id: 'del', label: 'Del' },
  ],
  [
    { id: 'grave', label: '`' },
    { id: '1', label: '1' },
    { id: '2', label: '2' },
    { id: '3', label: '3' },
    { id: '4', label: '4' },
    { id: '5', label: '5' },
    { id: '6', label: '6' },
    { id: '7', label: '7' },
    { id: '8', label: '8' },
    { id: '9', label: '9' },
    { id: '0', label: '0' },
    { id: 'minus', label: '-' },
    { id: 'equal', label: '=' },
    { id: 'backspace', label: '⌫', wide: 2 },
  ],
  [
    { id: 'tab', label: 'Tab', wide: 1.5 },
    { id: 'q', label: 'Q' },
    { id: 'w', label: 'W' },
    { id: 'e', label: 'E' },
    { id: 'r', label: 'R' },
    { id: 't', label: 'T' },
    { id: 'y', label: 'Y' },
    { id: 'u', label: 'U' },
    { id: 'i', label: 'I' },
    { id: 'o', label: 'O' },
    { id: 'p', label: 'P' },
    { id: 'lbracket', label: '[' },
    { id: 'rbracket', label: ']' },
    { id: 'backslash', label: '\\', wide: 1.5 },
  ],
  [
    { id: 'caps', label: 'Caps', wide: 1.75 },
    { id: 'a', label: 'A' },
    { id: 's', label: 'S' },
    { id: 'd', label: 'D' },
    { id: 'f', label: 'F' },
    { id: 'g', label: 'G' },
    { id: 'h', label: 'H' },
    { id: 'j', label: 'J' },
    { id: 'k', label: 'K' },
    { id: 'l', label: 'L' },
    { id: 'semicolon', label: ';' },
    { id: 'quote', label: "'" },
    { id: 'enter', label: 'Enter', wide: 2.25 },
  ],
  [
    { id: 'lshift', label: 'Shift', wide: 2.25 },
    { id: 'z', label: 'Z' },
    { id: 'x', label: 'X' },
    { id: 'c', label: 'C' },
    { id: 'v', label: 'V' },
    { id: 'b', label: 'B' },
    { id: 'n', label: 'N' },
    { id: 'm', label: 'M' },
    { id: 'comma', label: ',' },
    { id: 'dot', label: '.' },
    { id: 'slash', label: '/' },
    { id: 'rshift', label: 'Shift', wide: 2.75 },
  ],
  [
    { id: 'lctrl', label: 'Ctrl', wide: 1.25 },
    { id: 'lgui', label: 'Win', wide: 1.25 },
    { id: 'lalt', label: 'Alt', wide: 1.25 },
    { id: 'space', label: 'Space', wide: 6.25 },
    { id: 'ralt', label: 'Alt', wide: 1.25 },
    { id: 'rctrl', label: 'Ctrl', wide: 1.25 },
    { id: 'left', label: '←' },
    { id: 'down', label: '↓' },
    { id: 'right', label: '→' },
  ],
]

export function k3KeyIds() {
  return K3_LAYOUT.flat().map((key) => key.id)
}

export function litKeyIds(report) {
  const ids = []
  for (const key of report.keys ?? []) ids.push(key)
  for (const modifier of report.modifiers ?? []) ids.push(modifier)
  return ids
}
