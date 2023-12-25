# Rusty Twinkle Tray

Rusty Twinkle Tray is a work-in-progress (WIP) rewrite of Twinkle Tray in Rust.

## Overview

The project is organized into several modules, including ```monitors```, ```utils```, ```window```, and others. It uses the windows crate for Windows API bindings and the log crate for logging.

The main entry point of the application is ```src/main.rs```.

The application uses a custom window class XamlWindow and handles various window events.

The project also includes a justfile for task running.

## Codegen

The project uses a custom code generation tool located in ```lib/codegen```.

This tool generates Rust bindings for specified Windows classes and features.

The configuration for this tool is located in ```lib/windows-ext/Codegen.toml```.

## Dependencies

The project has several dependencies, including ```windows```, ```log```, and ```windows-ext```. 

The windows-ext is a local package that extends the ```windows``` crate with additional bindings.

## License

MIT License
