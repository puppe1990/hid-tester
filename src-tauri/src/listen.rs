use hid_tester::boot_report::parse_boot_report;
use hid_tester::hid_log::{append_jsonl, log_event_json, DEFAULT_LOG_PATH};
use hid_tester::hid_pick::{pick_boot_keyboard, HidIface, K3_PRO_PID, KEYCHRON_VID};
use hidapi::HidApi;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn default_log_path() -> &'static str {
    DEFAULT_LOG_PATH
}

pub fn start_probe(app: AppHandle, stop: Arc<AtomicBool>) -> Result<String, String> {
    let log_path = Path::new(DEFAULT_LOG_PATH);
    write_event(log_path, &json!({"kind":"probe-start"}))?;
    thread::spawn(move || {
        if let Err(err) = probe_loop(&app, &stop, log_path) {
            let payload = json!({"kind":"error","message":err});
            let _ = write_event(log_path, &payload);
            let _ = app.emit("hid-event", payload);
        }
    });
    Ok(DEFAULT_LOG_PATH.to_string())
}

fn probe_loop(app: &AppHandle, stop: &AtomicBool, log_path: &Path) -> Result<(), String> {
    let api = HidApi::new().map_err(|e| format!("HidApi: {e}"))?;
    let ifaces = inventory(&api);
    write_event(
        log_path,
        &json!({"kind":"inventory","devices": iface_json(&ifaces)}),
    )?;
    let Some(target) = pick_boot_keyboard(&ifaces) else {
        write_event(
            log_path,
            &json!({"kind":"error","message":"Keychron K3 Pro 3434:0230 ausente"}),
        )?;
        return Err("Keychron K3 Pro 3434:0230 ausente".into());
    };
    let chosen = target.clone();
    write_event(
        log_path,
        &json!({"kind":"open","path":chosen.path,"usage_page":chosen.usage_page,"usage":chosen.usage}),
    )?;
    let c_path = std::ffi::CString::new(chosen.path.clone()).map_err(|e| {
        format!("path HID inválido {}: {e}", chosen.path)
    })?;
    let device = api.open_path(&c_path).map_err(|e| {
        format!(
            "não abriu HID {} (usage {:04x}:{:04x}): {e}",
            chosen.path, chosen.usage_page, chosen.usage
        )
    })?;
    let mut buf = [0u8; 64];
    let mut last = Vec::new();
    while !stop.load(Ordering::Relaxed) {
        match device.read_timeout(&mut buf, 80) {
            Ok(0) => {}
            Ok(n) => {
                let frame = buf[..n].to_vec();
                if frame == last {
                    continue;
                }
                last = frame.clone();
                let payload = report_payload(&chosen, &frame);
                let _ = write_event(log_path, &payload);
                let _ = app.emit("hid-event", payload);
            }
            Err(err) => {
                let payload = json!({"kind":"error","message":format!("read: {err}")});
                let _ = write_event(log_path, &payload);
                let _ = app.emit("hid-event", payload);
                thread::sleep(Duration::from_millis(400));
            }
        }
    }
    write_event(log_path, &json!({"kind":"probe-stop"}))
}

fn inventory(api: &HidApi) -> Vec<HidIface> {
    api.device_list()
        .map(|dev| HidIface {
            vid: dev.vendor_id(),
            pid: dev.product_id(),
            usage_page: dev.usage_page(),
            usage: dev.usage(),
            path: String::from_utf8_lossy(dev.path().to_bytes()).into_owned(),
            product: dev.product_string().unwrap_or_default().to_string(),
        })
        .collect()
}

fn iface_json(ifaces: &[HidIface]) -> Value {
    json!(ifaces
        .iter()
        .map(|iface| json!({
            "vid": format!("{:04x}", iface.vid),
            "pid": format!("{:04x}", iface.pid),
            "usage_page": format!("{:04x}", iface.usage_page),
            "usage": format!("{:04x}", iface.usage),
            "path": iface.path,
            "product": iface.product,
            "keychron": iface.vid == KEYCHRON_VID && iface.pid == K3_PRO_PID,
            "dfu": iface.vid == 0x0483 && iface.pid == 0xDF11,
        }))
        .collect::<Vec<_>>())
}

fn report_payload(iface: &HidIface, frame: &[u8]) -> Value {
    match parse_boot_report(frame) {
        Ok(report) => json!({
            "kind": "report",
            "hex": report.hex,
            "keys": report.keys,
            "modifiers": report.modifiers,
            "slots_used": report.slots_used,
            "rollover": report.rollover,
            "usage_page": iface.usage_page,
            "usage": iface.usage,
            "bytes": frame.len(),
        }),
        Err(_) => json!({
            "kind": "raw",
            "hex": frame.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" "),
            "keys": [],
            "modifiers": [],
            "slots_used": 0,
            "rollover": false,
            "usage_page": iface.usage_page,
            "usage": iface.usage,
            "bytes": frame.len(),
        }),
    }
}

fn write_event(path: &Path, fields: &Value) -> Result<(), String> {
    append_jsonl(path, &log_event_json(fields))
}
