mod hotkeys;
mod mouse;
use hotkeys::{start_listener, HotkeyEvent};
use mouse::VirtualMouse;
use std::sync::mpsc;

// defaults
const DEFAULT_SENS: f64 = 4.0;
const DEFAULT_YAW: f64 = 0.022;
const DEFAULT_SPEED: f64 = 1.0;

fn main() {
    let (tx, rx) = mpsc::channel::<HotkeyEvent>();

    start_listener(tx);

    let mut vmouse = match VirtualMouse::new() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to create virtual mouse: {}\nAre you in the input group?", e);
            return;
        }
    };

    println!("Listening for Alt+Backspace...");

    for event in rx {
        match event {
            HotkeyEvent::AltBackspace => {
                // same as python _handle_hotkey
                let total_counts = 360.0 / (DEFAULT_YAW * DEFAULT_SENS);
                vmouse.move_relative(total_counts, DEFAULT_SPEED);
                println!("360 triggerd - counts: {:.2}", total_counts);
            }
        }
    }
}
