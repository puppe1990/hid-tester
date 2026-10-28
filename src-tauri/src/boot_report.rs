#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootKeyboardReport {
    pub modifiers: Vec<&'static str>,
    pub usages: Vec<u8>,
    pub keys: Vec<&'static str>,
    pub slots_used: usize,
    pub rollover: bool,
    pub hex: String,
}

const MOD_NAMES: [&str; 8] = [
    "lctrl", "lshift", "lalt", "lgui", "rctrl", "rshift", "ralt", "rgui",
];

pub fn parse_boot_report(bytes: &[u8]) -> Result<BootKeyboardReport, String> {
    let hex = to_hex(bytes);
    let body = strip_report_id(bytes);
    if body.len() < 8 {
        return Err(format!(
            "relatório HID curto: recebido {} byte(s), esperado 8 (boot keyboard)",
            bytes.len()
        ));
    }
    let modifiers = modifiers_from(body[0]);
    let slot_bytes = &body[2..8];
    let rollover = slot_bytes.iter().any(|&b| b == 0x01);
    let usages: Vec<u8> = slot_bytes
        .iter()
        .copied()
        .filter(|&b| b >= 0x04)
        .collect();
    let keys: Vec<&'static str> = usages.iter().copied().map(usage_name).collect();
    Ok(BootKeyboardReport {
        modifiers,
        usages,
        keys,
        slots_used: slot_bytes.iter().filter(|&&b| b != 0).count(),
        rollover,
        hex,
    })
}

fn strip_report_id(bytes: &[u8]) -> &[u8] {
    if bytes.len() >= 9 && bytes[0] == 0x01 {
        return &bytes[1..];
    }
    bytes
}

fn modifiers_from(bits: u8) -> Vec<&'static str> {
    MOD_NAMES
        .iter()
        .enumerate()
        .filter(|(i, _)| bits & (1 << i) != 0)
        .map(|(_, name)| *name)
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn usage_name(usage: u8) -> &'static str {
    match usage {
        0x04 => "a",
        0x05 => "b",
        0x06 => "c",
        0x07 => "d",
        0x08 => "e",
        0x09 => "f",
        0x0A => "g",
        0x0B => "h",
        0x0C => "i",
        0x0D => "j",
        0x0E => "k",
        0x0F => "l",
        0x10 => "m",
        0x11 => "n",
        0x12 => "o",
        0x13 => "p",
        0x14 => "q",
        0x15 => "r",
        0x16 => "s",
        0x17 => "t",
        0x18 => "u",
        0x19 => "v",
        0x1A => "w",
        0x1B => "x",
        0x1C => "y",
        0x1D => "z",
        0x1E => "1",
        0x1F => "2",
        0x20 => "3",
        0x21 => "4",
        0x22 => "5",
        0x23 => "6",
        0x24 => "7",
        0x25 => "8",
        0x26 => "9",
        0x27 => "0",
        0x28 => "enter",
        0x29 => "esc",
        0x2A => "backspace",
        0x2B => "tab",
        0x2C => "space",
        0x2D => "minus",
        0x2E => "equal",
        0x2F => "lbracket",
        0x30 => "rbracket",
        0x31 => "backslash",
        0x33 => "semicolon",
        0x34 => "quote",
        0x35 => "grave",
        0x36 => "comma",
        0x37 => "dot",
        0x38 => "slash",
        0x39 => "caps",
        0x3A => "f1",
        0x3B => "f2",
        0x3C => "f3",
        0x3D => "f4",
        0x3E => "f5",
        0x3F => "f6",
        0x40 => "f7",
        0x41 => "f8",
        0x42 => "f9",
        0x43 => "f10",
        0x44 => "f11",
        0x45 => "f12",
        0x4C => "del",
        0x4F => "right",
        0x50 => "left",
        0x51 => "down",
        0x52 => "up",
        0xE0 => "lctrl",
        0xE1 => "lshift",
        0xE2 => "lalt",
        0xE3 => "lgui",
        0xE4 => "rctrl",
        0xE5 => "rshift",
        0xE6 => "ralt",
        0xE7 => "rgui",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::parse_boot_report;

    #[test]
    fn shift_and_a_fills_one_slot() {
        let report = parse_boot_report(&[0x02, 0x00, 0x04, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(report.modifiers, vec!["lshift"]);
        assert_eq!(report.usages, vec![0x04]);
        assert_eq!(report.keys, vec!["a"]);
        assert_eq!(report.slots_used, 1);
        assert!(!report.rollover);
        assert_eq!(report.hex, "02 00 04 00 00 00 00 00");
    }

    #[test]
    fn six_filled_slots_is_rollover_risk() {
        let report = parse_boot_report(&[0, 0, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]).unwrap();
        assert_eq!(report.slots_used, 6);
        assert_eq!(report.keys, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn hid_error_rollover_flag() {
        let report = parse_boot_report(&[0, 0, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]).unwrap();
        assert!(report.rollover);
        assert!(report.keys.is_empty());
    }

    #[test]
    fn short_frame_names_got_and_expected() {
        let err = parse_boot_report(&[0x02, 0x00]).unwrap_err();
        assert!(err.contains("2"), "{err}");
        assert!(err.contains("8"), "{err}");
    }

    #[test]
    fn report_id_prefix_is_stripped() {
        let report = parse_boot_report(&[0x01, 0x00, 0x00, 0x29, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(report.keys, vec!["esc"]);
        assert_eq!(report.hex, "01 00 00 29 00 00 00 00 00");
    }
}
