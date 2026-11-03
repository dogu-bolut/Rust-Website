#!/bin/sh
# Run the site locally at http://localhost:8080 with live reload.
# Uses rustup's toolchain (which honours rust-toolchain.toml and has the wasm target)
# instead of Homebrew's `rust`, and the Trunk binary in .bin/.
set -e
cd "$(dirname "$0")"
export PATH="/opt/homebrew/opt/rustup/bin:$HOME/.cargo/bin:$PATH"
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
exec ./.bin/trunk serve "$@"
