use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, Key, RelativeAxisType};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender};

pub struct VirtualMouse {
    device: VirtualDevice,
}

impl VirtualMouse {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut axes = AttributeSet::<RelativeAxisType>::new();
        axes.insert(RelativeAxisType::REL_X);
        axes.insert(RelativeAxisType::REL_Y);

        // FOR SOME FUCKIN REASON THIS SHIT DOESNT WORK WITHOUT A BTN_LEFT THAT IS NEVER FUCKING
        // USED
        let mut keys = AttributeSet::<Key>::new();
        keys.insert(Key::BTN_LEFT);

        let device = VirtualDeviceBuilder::new()?
            .name("Sensitivity-Matcher-Virtual-Mouse")
            .with_relative_axes(&axes)?
            .with_keys(&keys)?
            .build()?;

        Ok(Self { device })
    }

    // same as python move_mouse_relative(self, total_x, speed_multiplier)
    pub fn move_relative(&mut self, total_x: f64, speed_multiplier: f64) {
        let step_size = 100.0 * speed_multiplier;
        let direction: f64 = if total_x > 0.0 { 1.0 } else { -1.0 };
        let mut remaining = total_x.abs();

        while remaining > 0.0 {
            let move_amount = remaining.min(step_size);
            let move_int = (move_amount * direction) as i32;

            // write REL_X event then sync; same as pythons vmouse.write() + vmouse.syn()
            let event = evdev::InputEvent::new(
                evdev::EventType::RELATIVE,
                RelativeAxisType::REL_X.0,
                move_int,
            );

            if let Err(e) = self.device.emit(&[event]) {
                eprintln!("emit failed: {}", e);
            }

            remaining -= move_amount;
            thread::sleep(Duration::from_millis(1));
        }
    }
}

pub fn start_mouse_thread() -> Sender<(f64, f64)> {
    let (tx, rx) = mpsc::channel::<(f64, f64)>();
    thread::spawn(move || {
        let mut vmouse = match VirtualMouse::new() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to create virtual mouse: {}", e);
                return;
            }
        };
        for (total_x, speed) in rx {
            vmouse.move_relative(total_x, speed);
        }
    });
    tx
}
