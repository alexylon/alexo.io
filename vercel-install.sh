#!/usr/bin/env bash
set -euo pipefail

# prerender.sh needs lsof.
# curl and python3 are already available on Vercel's Amazon Linux image.
dnf install -y lsof

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --profile minimal

source "$HOME/.cargo/env"

rustup target add wasm32-unknown-unknown

# cargo-binstall
curl -L --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
  | bash

# Dioxus CLI
cargo binstall --no-confirm dioxus-cli@0.7.3

rustc --version
cargo --version
dx --version
