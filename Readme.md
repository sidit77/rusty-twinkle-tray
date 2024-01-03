# Rusty Twinkle Tray

Rusty Twinkle Tray is a work-in-progress (WIP) rewrite of Twinkle Tray in Rust. A central goal of this rewrite is to start much faster so that monitor brightness can be adjusted as soon as possible after logging in.

## Screenshot
![image](https://github.com/sidit77/rusty-twinkle-tray/assets/5053369/d4b3d3b3-fd2f-4e06-844e-dbae24b73046)

## Precompiled Binaries
*Coming soon*

## Building
This program is written in [Rust](https://www.rust-lang.org/) and therefore requires a working Rust installation if you want to compile it yourself.

This program can be built by simply running:
```shell
cargo build --release
```

## Contributing

### Toolkit
This program uses the default [UWP Controls](https://learn.microsoft.com/en-us/uwp/api/windows.ui.xaml.controls?view=winrt-22621) hosted inside an XAML island.

Unfortunately, the Microsoft-provided [`windows-rs` crate](https://microsoft.github.io/windows-docs-rs/doc/windows/) does not include bindings for the `Window.UI.Xaml` namespace. To nevertheless use it with Rust, this project directly uses [`windows-bindgen`](https://crates.io/crates/windows-bindgen) to manually generate the missing bindings (`libs/windows-ext`). However, the generated bindings are quite large (~1.5m LOC; ~1m of that in a single file). To keep the IDE happy, this project additionally used a post-processor (`/libs/codegen`) that deletes many unused functions. The file `libs/windows-ext/Codegen.toml` controls which functions are kept. To regenerate the bindings after editing this file, simply run `just update-bindings` (requires [just](https://github.com/casey/just)).

### Todos
- [ ] Settings window
- [ ] Fluent design for Windows 11+
- [ ] Respect Dark/Light system setting
- [ ] Attempt to restore brightness after waking up from sleep
- [ ] Support integrated laptop screens connected over I2C
- [ ] Support monitor hot plugging


## License

MIT License
