mod hotkeys;
use hotkeys::{start_listener, HotkeyEvent};
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<HotkeyEvent>();

    start_listener(tx);

    println!("Listening for Alt+Backspace...");

    for event in rx {
        match event {
            HotkeyEvent::AltBackspace => {
                println!("Alt+Backspace triggered!");
            }
        }
    }
}
