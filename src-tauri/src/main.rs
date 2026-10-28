#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod listen;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::Manager;

struct Probe {
    stop: Mutex<Option<Arc<AtomicBool>>>,
}

impl Default for Probe {
    fn default() -> Self {
        Self {
            stop: Mutex::new(None),
        }
    }
}

#[tauri::command]
fn log_path() -> String {
    listen::default_log_path().to_string()
}

#[tauri::command]
fn start_probe(app: tauri::AppHandle, probe: tauri::State<Probe>) -> Result<String, String> {
    halt(&probe)?;
    let stop = Arc::new(AtomicBool::new(false));
    let path = listen::start_probe(app, Arc::clone(&stop))?;
    *probe.stop.lock().map_err(|e| e.to_string())? = Some(stop);
    Ok(path)
}

#[tauri::command]
fn stop_probe(probe: tauri::State<Probe>) -> Result<(), String> {
    halt(&probe)
}

fn halt(probe: &Probe) -> Result<(), String> {
    if let Some(flag) = probe.stop.lock().map_err(|e| e.to_string())?.take() {
        flag.store(true, Ordering::Relaxed);
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(Probe::default())
        .invoke_handler(tauri::generate_handler![log_path, start_probe, stop_probe])
        .setup(|app| {
            let handle = app.handle().clone();
            let probe = app.state::<Probe>();
            let _ = start_probe(handle, probe);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("tauri");
}
