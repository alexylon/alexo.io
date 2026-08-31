#!/usr/bin/env bash
set -euo pipefail

dnf install -y curl python3 lsof

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --profile minimal

source "$HOME/.cargo/env"

rustup target add wasm32-unknown-unknown

curl -L --proto '=https' --tlsv1.2 -sSf \
  https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
  | bash

cargo binstall --no-confirm dioxus-cli@0.7.3
