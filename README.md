# DePk Sensitivity Matcher
A Rust-based tool for matching mouse sensitivity between 3D games on Linux using evdev and uinput. Works on both X11 and Wayland.

Heavily inspired by [Kovaak's Sensitivity Matcher](https://github.com/KovaaK/SensitivityMatcher/).

## Installation
Only *officially* supported on Arch as I haven't the means (nor the inclination) to test on other versions. Should work just swimmingly on *most* other distributions.

I have also provided a binary.

### Arch:
```shell
```
Note: Or use AUR helper of choice.

### Other:
```shell
```
Note: Requires

### Binary:
* Download the [binary](https://github.com/deliriouspork/DePk-Sensitivity-Matcher/releases).
* chmod +x DePkSensMatch
* ./DePkSensMatch

## Usage
Run the tool and enter your sensitivity and game/engine. Open the game you wish to set your sensitivity in and press `ALT+BACKSPACE`. Fiddle with your sensitivity in-game until `ALT+BACKSPACE` performs a perfect (or close to a) 360 degree rotation.

### TODO (depending on if anyone actually uses this garbled together POS python code written by an inexperienced Forestry student in his free time)
* Actually make it
