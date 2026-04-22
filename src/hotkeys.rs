use evdev::{EventType, Key};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};
use std::thread;

// all hotkey event types go here
pub enum HotkeyEvent {
    AltBackspace,
}

pub fn start_listener(tx: Sender<HotkeyEvent>) {
    // shared atomic bool so all device threads agree on whether alt is held.
    // needed as evdev treats each device separatey
    let alt_held = Arc::new(AtomicBool::new(false));

    // filter to only devices that have a backspace key
    let keyboards: Vec<_> = evdev::enumerate()
        .filter(|(_, dev)| {
            dev.supported_keys()
                .map_or(false, |keys| keys.contains(Key::KEY_BACKSPACE))
        })
        .collect();

    if keyboards.is_empty() {
        eprintln!("No keyboard devices found. Are you in the input group?");
        return;
    }

    for (path, mut device) in keyboards {
        let tx = tx.clone();
        let alt_held = Arc::clone(&alt_held);

        thread::spawn(move || {
            println!("Listening on: {:?}", path);

            loop {
                // fetch_events() blocks until there are events — no busy loop
                match device.fetch_events() {
                    Ok(events) => {
                        for event in events {
                            if event.event_type() != EventType::KEY {
                                continue;
                            }

                            let key = Key::new(event.code());
                            let value = event.value();
                            // value: 1 = keydown, 0 = keyup, 2 = autorepeat

                            // track alt state
                            if key == Key::KEY_LEFTALT || key == Key::KEY_RIGHTALT {
                                alt_held.store(value == 1, Ordering::SeqCst);
                            }

                            // fire on Alt+Backspace keydown only (not autorepeat)
                            if key == Key::KEY_BACKSPACE
                                && value == 1
                                && alt_held.load(Ordering::SeqCst)
                            {
                                // ignore send errors — main thread may have exited
                                let _ = tx.send(HotkeyEvent::AltBackspace);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Lost device {:?}: {}", path, e);
                        break; // device disconnected etc., kill this thread
                    }
                }
            }
        });
    }
}
