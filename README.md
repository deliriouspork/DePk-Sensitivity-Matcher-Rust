<img width="391" height="266" alt="Screenshot_20260421_223014" src="https://github.com/user-attachments/assets/5e6ff4dd-8c18-42fa-a469-15e00eed3abf" />

# DePk Sensitivity Matcher
A tool for matching mouse sensitivity between 3D games on Linux using evdev and uinput. Works on both X11 and Wayland.

Heavily inspired by [Kovaak's Sensitivity Matcher](https://github.com/KovaaK/SensitivityMatcher/).

## Installation
Only *officially* supported on Arch as I haven't the means (nor the inclination) to test on other versions. Should work just swimmingly on *most* other distributions.

I have also provided a binary.

When **NOT** installing from AUR, due to Wayland's security architecture this program **WILL NOT WORK** unless the user running the program is a member of the **input** group. If you are downloading from the AUR, this is not necessary.

**WARNING:** Having your user be a member of the input group is generally not advised as it *can* make you more susceptible to keyloggers and other attacks. What I do, and what I'd recommend you do as well, is to run `sudo -E -g input bash` in your terminal before starting the application. This adds your user to the input group **ONLY** for your current shell. 

## Usage
Run the tool and enter your sensitivity and game/engine. Open the game you wish to set your sensitivity in and press `ALT+BACKSPACE`. Fiddle with your sensitivity in-game until `ALT+BACKSPACE` performs a perfect (or close to a) 360 degree rotation.

### Arch (AUR):
```shell
yay -S depk-sensitivity-matcher
```
Note: Or use AUR helper of choice.

### From Source:
Requires `rust` (cargo) and a kernel with uinput support (virtually all distros).
```shell
git clone https://github.com/deliriouspork/DePk-Sensitivity-Matcher-Rust
cd DePk-Sensitivity-Matcher-Rust/
cargo build --release
sudo ./target/release/Depk-Sensitivity-Matcher-Rust
```
Note: Requires sudo or user in the input group.

### Binary:
* Download the [binary](https://github.com/deliriouspork/DePk-Sensitivity-Matcher-Rust/releases).
* chmod +x DePkSensMatch
* ./DePk-Sensitivity-Matcher-Rust

Note: Requires sudo or user in the input group.

## Settings
Settings are saved automatically on close to:
```
~/.config/depk-sensitivity-matcher/settings.json
```

### TODO (depending on if anyone actually uses this garbled together POS rust code written by an inexperienced Forestry student in his free time)
- More presets
- Configurable hotkey
