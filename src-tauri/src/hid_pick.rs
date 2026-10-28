#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidIface {
    pub vid: u16,
    pub pid: u16,
    pub usage_page: u16,
    pub usage: u16,
    pub path: String,
    pub product: String,
}

pub const KEYCHRON_VID: u16 = 0x3434;
pub const K3_PRO_PID: u16 = 0x0230;

pub fn pick_boot_keyboard(ifaces: &[HidIface]) -> Option<&HidIface> {
    ifaces
        .iter()
        .find(|iface| is_k3_pro(iface) && iface.usage_page == 0x01 && iface.usage == 0x06)
        .or_else(|| ifaces.iter().find(|iface| is_k3_pro(iface)))
}

fn is_k3_pro(iface: &HidIface) -> bool {
    iface.vid == KEYCHRON_VID && iface.pid == K3_PRO_PID
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iface(usage_page: u16, usage: u16, path: &str) -> HidIface {
        HidIface {
            vid: KEYCHRON_VID,
            pid: K3_PRO_PID,
            usage_page,
            usage,
            path: path.to_string(),
            product: "Keychron K3 Pro".to_string(),
        }
    }

    #[test]
    fn prefers_boot_keyboard_over_vendor_raw() {
        let ifaces = [
            iface(0xFF60, 0x61, "raw"),
            iface(0x01, 0x06, "boot"),
            iface(0x01, 0x02, "mouse"),
        ];
        let picked = pick_boot_keyboard(&ifaces).unwrap();
        assert_eq!(picked.path, "boot");
    }

    #[test]
    fn falls_back_to_any_k3_when_boot_missing() {
        let ifaces = [iface(0xFF60, 0x61, "raw")];
        let picked = pick_boot_keyboard(&ifaces).unwrap();
        assert_eq!(picked.path, "raw");
    }

    #[test]
    fn ignores_other_vendors() {
        let other = HidIface {
            vid: 0x046D,
            pid: 0xC52B,
            usage_page: 0x01,
            usage: 0x06,
            path: "logitech".into(),
            product: "other".into(),
        };
        assert!(pick_boot_keyboard(&[other]).is_none());
    }
}
