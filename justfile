set windows-shell := ["powershell.exe", "-c"]

check-bindings:
    cargo check --package windows-ext

update-bindings: && check-bindings
    cargo run --package codegen .\lib\windows-ext\

format:
    cargo +nightly fmt --package rusty-twinkle-tray