default:
  @just --list

init:
    @echo "Initialising development environment"
    cargo install tauri-cli@^2.0.0-rc
    cargo install --locked trunk
    # ARM-based Macs must install wasm-bindget manually
    cargo install --locked wasm-bindgen-cli

[macos]
init-ios:
    @echo "Initialising development environment for iOS"
    ln -s $(which pnpm) /usr/local/bin/pnpm
    ln -s $(which node) /usr/local/bin/node
    ln -s $(which cargo) /usr/local/bin/cargo

dev target='desktop':
    @echo "Running dev instance for {{target}}..."
    pushd application
    if [ "{{target}}" = "ios" ]; then \
        cargo tauri dev ios --open; \
    elif [ "{{target}}" = "desktop" ]; then \
        cargo tauri dev; \
    fi

test $RUST_BACKTRACE="1":
    cargo test --all
