# Rusty Twinkle Tray

A small utility for quickly adjusting the brightness of external monitors using the DDC/CI protocol.

Rusty Twinkle Tray is a work-in-progress rewrite of Twinkle Tray in Rust. A central goal of this rewrite is to start much faster so that monitor brightness can be adjusted as soon as possible after logging in.

## Screenshot
![image](https://github.com/user-attachments/assets/7f4c05d9-865d-4cdf-ae66-8f649b262912)

## Features
- Can automatically restore the last set brightness after¹:
  - Changing display settings
  - Waking up from sleep
- Small (~900kb) standalone executable
- Built using native OS controls instead of electron
- As inactive as possible when not in use
- Minimal dependencies

¹*Many monitors tend to "forget" settings set over DDC/CI after temporarily losing power*

## Precompiled Binaries
| [**DOWNLOAD**](https://github.com/sidit77/rusty-twinkle-tray/releases/latest) |
|-----------------------------------------------------------------------------------------------------------------|

## Building
This program is written in [Rust](https://www.rust-lang.org/) and requires a working Rust installation if you want to compile it yourself.

This program can be built by simply running the following:
```shell
cargo build --release
```

## Contributing

### Toolkit
This program uses the default [UWP Controls](https://learn.microsoft.com/en-us/uwp/api/windows.ui.xaml.controls?view=winrt-22621) hosted inside an XAML island.

Unfortunately, the Microsoft-provided [`windows-rs` crate](https://microsoft.github.io/windows-docs-rs/doc/windows/) does not include bindings for the `Window.UI.Xaml` namespace. To nevertheless use it with Rust, this project directly uses [`windows-bindgen`](https://crates.io/crates/windows-bindgen) to manually generate the missing bindings (`libs/windows-ext`). However, the generated bindings are quite large (~1.5m LOC; ~1m of that in a single file). To keep the IDE happy, this project additionally uses a post-processor (`/libs/codegen`) that deletes many unused functions. The file `libs/windows-ext/Codegen.toml` controls which functions are kept. To regenerate the bindings after editing this file, run `just update-bindings` (requires [just](https://github.com/casey/just)).

### Todos
- [x] Settings window
  - [x] Toggle autostart
  - [x] Toggle brightness restore
  - [ ] Rename monitors
  - [ ] Reorder monitors
  - [ ] Change various delays/timeouts
- [ ] Fluent design for Windows 11+
- [x] Respect Dark/Light system setting
- [x] Attempt to restore brightness after waking up from sleep
- [ ] Support integrated laptop screens connected over I2C
- [x] Support monitor hot plugging
- [ ] Handle auto-hiding taskbar
- [ ] Improve themes
- [x] Adapt icon to system theme
- [x] Correctly handle display scaling


## License

MIT License
