#!/usr/bin/env bash
set -euo pipefail

export RUSTUP_HOME="/rust"
export CARGO_HOME="$HOME/.cargo"
export PATH="/rust/bin:$CARGO_HOME/bin:$PATH"

# Needed by prerender.sh
dnf install -y lsof

# Vercel already provides Rust/rustup
rustc --version
cargo --version
rustup --version

rustup target add wasm32-unknown-unknown

# Install cargo-binstall into /vercel/.cargo/bin
curl -L --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
  | bash

# Install the Dioxus version your project uses
cargo binstall \
  --no-confirm \
  --force \
  --root "$CARGO_HOME" \
  dioxus-cli@0.7.3

dx --version
