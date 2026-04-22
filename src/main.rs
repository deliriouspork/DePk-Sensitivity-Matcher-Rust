slint::include_modules!();

mod hotkeys;
mod mouse;

use hotkeys::{start_listener, HotkeyEvent};
use mouse::start_mouse_thread;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

const YAW_PRESETS: &[&str] = &["0.022", "0.0066", "0.02", "0.002201", "0.002"];

// runtime state used for hotkey calculations
struct RuntimeSettings {
    sens: f64,
    yaw: f64,
    speed: f64,
    updating_from_preset: bool,
}

// mirrors pythons DEFAULTS
impl Default for RuntimeSettings {
    fn default() -> Self {
        Self { sens: 4.0, yaw: 0.022, speed: 1.0, updating_from_preset: false }
    }
}

// mirrors pythons _save_settings / _load_settings
#[derive(Serialize, Deserialize)]
struct SavedSettings {
    sens: String,
    yaw: String,
    speed: String,
    preset_index: i32,
}

impl Default for SavedSettings {
    fn default() -> Self {
        Self {
            sens: "4.0".into(),
            yaw: "0.022".into(),
            speed: "1".into(),
            preset_index: 0,
        }
    }
}

fn settings_path() -> PathBuf {
    let mut path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("depk-sensitivity-matcher");
    path.push("settings.json");
    path
}

fn load_settings() -> SavedSettings {
    let path = settings_path();
    if !path.exists() {
        return SavedSettings::default();
    }
    match std::fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
            eprintln!("settings.json corrupted ({}), loading defaults.", e);
            SavedSettings::default()
        }),
        Err(e) => {
            eprintln!("Could not read settings ({}), loading defaults.", e);
            SavedSettings::default()
        }
    }
}

fn save_settings(saved: &SavedSettings) {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Failed to create config dir: {}", e);
            return;
        }
    }
    match serde_json::to_string_pretty(saved) {
        Ok(contents) => {
            if let Err(e) = std::fs::write(&path, contents) {
                eprintln!("Failed to save settings: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to serialize settings: {}", e),
    }
}

fn compute_increment(yaw: f64, sens: f64) -> slint::SharedString {
    if yaw == 0.0 || sens == 0.0 {
        return "?".into();
    }
    let s = format!("{:.10}", yaw * sens);
    s.trim_end_matches('0').trim_end_matches('.').into()
}

fn main() {
    let saved = load_settings();

    // parse saved strings into runtime values, fall back to defaults on error
    let defaults = RuntimeSettings::default();
    let initial = RuntimeSettings {
        sens: saved.sens.parse().unwrap_or(defaults.sens),
        yaw: saved.yaw.parse().unwrap_or(defaults.yaw),
        speed: saved.speed.parse().unwrap_or(defaults.speed),
        updating_from_preset: false,
    };

    let settings = Arc::new(Mutex::new(initial));

    let (hk_tx, hk_rx) = std::sync::mpsc::channel::<HotkeyEvent>();
    start_listener(hk_tx);

    let mouse_tx = start_mouse_thread();

    let ui = MainWindow::new().unwrap();

    ui.window().set_identifier("depk-sensitivity-matcher"); // two windows fix

    // apply loaded settings to UI
    ui.set_sens_text(saved.sens.as_str().into());
    ui.set_yaw_text(saved.yaw.as_str().into());
    ui.set_speed_text(saved.speed.as_str().into());
    ui.set_preset_index(saved.preset_index);
    {
        let s = settings.lock().unwrap();
        ui.set_increment_text(compute_increment(s.yaw, s.sens));
    }

    // sens changed
    let s_sens = Arc::clone(&settings);
    let weak_sens = ui.as_weak();
    ui.on_sens_changed(move |val| {
        let mut s = s_sens.lock().unwrap();
        // avoid feedback loop
        match val.parse::<f64>() {
            Ok(v) => {
                s.sens = v;
                let inc = compute_increment(s.yaw, v);
                drop(s);
                if let Some(ui) = weak_sens.upgrade() {
                    ui.set_increment_text(inc);
                }
            }
            Err(_) => {
                drop(s);
                if let Some(ui) = weak_sens.upgrade() {
                    ui.set_increment_text("?".into());
                }
            }
        }
    });

    // yaw changed
    let s_yaw = Arc::clone(&settings);
    let weak_yaw = ui.as_weak();
    ui.on_yaw_changed(move |val| {
        let mut s = s_yaw.lock().unwrap();
        if s.updating_from_preset {
            return;
        }
        match val.parse::<f64>() {
            Ok(v) => {
                s.yaw = v;
                let inc = compute_increment(v, s.sens);
                drop(s);
                if let Some(ui) = weak_yaw.upgrade() {
                    ui.set_increment_text(inc);
                    let matched = YAW_PRESETS
                        .iter()
                        .position(|&p| p.parse::<f64>().ok() == Some(v));
                    ui.set_preset_index(matched.unwrap_or(5) as i32);
                }
            }
            Err(_) => {
                drop(s);
                if let Some(ui) = weak_yaw.upgrade() {
                    ui.set_increment_text("?".into());
                    ui.set_preset_index(5);
                }
            }
        }
    });

    // preset changed
    let s_preset = Arc::clone(&settings);
    let weak_preset = ui.as_weak();
    ui.on_preset_changed(move |index| {
        if let Some(&yaw_str) = YAW_PRESETS.get(index as usize) {
            if let Ok(yaw_val) = yaw_str.parse::<f64>() {
                let mut s = s_preset.lock().unwrap();
                s.yaw = yaw_val;
                s.updating_from_preset = true;
                let inc = compute_increment(yaw_val, s.sens);
                drop(s);
                if let Some(ui) = weak_preset.upgrade() {
                    ui.set_yaw_text(yaw_str.into());
                    ui.set_increment_text(inc);
                }
                s_preset.lock().unwrap().updating_from_preset = false;
            }
        }
        // index 5 = Custom: leave yaw as-is
    });

    // speed changed
    let s_speed = Arc::clone(&settings);
    ui.on_speed_changed(move |val| {
        if let Ok(v) = val.parse::<f64>() {
            s_speed.lock().unwrap().speed = v;
        }
    });

    // Hotkey receiver thread
    let s_hk = Arc::clone(&settings);
    thread::spawn(move || {
        for event in hk_rx {
            match event {
                HotkeyEvent::AltBackspace => {
                    let s = s_hk.lock().unwrap();
                    let (yaw, sens, speed) = (s.yaw, s.sens, s.speed);
                    drop(s);

                    if yaw == 0.0 || sens == 0.0 {
                        eprintln!("Error: Yaw and Sensitivity cannot be zero.");
                        continue;
                    }
                    let _ = mouse_tx.send((360.0 / (yaw * sens), speed));
                }
            }
        }
    });

    ui.run().unwrap();

    save_settings(&SavedSettings {
        sens: ui.get_sens_text().to_string(),
        yaw: ui.get_yaw_text().to_string(),
        speed: ui.get_speed_text().to_string(),
        preset_index: ui.get_preset_index(),
    });
}
