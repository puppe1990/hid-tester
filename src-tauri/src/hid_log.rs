use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_LOG_PATH: &str = "/tmp/hid-tester.jsonl";

pub fn log_event_json(fields: &Value) -> String {
    let mut object = fields.clone();
    if let Some(map) = object.as_object_mut() {
        map.entry("ts".to_string())
            .or_insert_with(|| json!(iso_now()));
    }
    object.to_string()
}

pub fn append_jsonl(path: &Path, line: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("não abriu log {}: {e}", path.display()))?;
    writeln!(file, "{line}").map_err(|e| format!("não gravou log {}: {e}", path.display()))
}

fn iso_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (year, month, day) = civil_from_unix_days((secs / 86400) as i64);
    let rem = secs % 86400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Howard Hinnant civil-from-days (days since 1970-01-01).
fn civil_from_unix_days(unix_days: i64) -> (i32, u32, u32) {
    let z = unix_days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

#[cfg(test)]
mod tests {
    use super::{append_jsonl, log_event_json};
    use serde_json::json;
    use std::fs;

    #[test]
    fn report_event_includes_hex_keys_and_path() {
        let line = log_event_json(&json!({
            "kind": "report",
            "hex": "02 00 04 00 00 00 00 00",
            "keys": ["a"],
            "modifiers": ["lshift"],
            "slots_used": 1
        }));
        let parsed: serde_json::Value = serde_json::from_str(&line).unwrap();
        assert_eq!(parsed["kind"], "report");
        assert_eq!(parsed["hex"], "02 00 04 00 00 00 00 00");
        assert_eq!(parsed["keys"][0], "a");
        assert!(parsed["ts"].as_str().unwrap().len() >= 19, "{}", parsed["ts"]);
    }

    #[test]
    fn append_jsonl_writes_one_line_with_newline() {
        let dir = std::env::temp_dir().join(format!("hid-tester-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("hid-tester.jsonl");
        let _ = fs::remove_file(&path);
        append_jsonl(&path, r#"{"kind":"hello"}"#).unwrap();
        append_jsonl(&path, r#"{"kind":"world"}"#).unwrap();
        let body = fs::read_to_string(&path).unwrap();
        assert_eq!(body, "{\"kind\":\"hello\"}\n{\"kind\":\"world\"}\n");
        let _ = fs::remove_file(&path);
    }
}
