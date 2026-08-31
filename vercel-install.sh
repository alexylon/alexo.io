#!/usr/bin/env bash
set -euo pipefail

export RUSTUP_HOME="/rust"
export CARGO_HOME="$HOME/.cargo"
export PATH="/rust/bin:$CARGO_HOME/bin:$PATH"

dnf install -y lsof

rustup target add wasm32-unknown-unknown

# Build dx against Vercel's glibc instead of downloading incompatible binary
cargo install \
  --locked \
  --version 0.7.3 \
  --root "$CARGO_HOME" \
  dioxus-cli

dx --version
