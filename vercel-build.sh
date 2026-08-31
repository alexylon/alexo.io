#!/usr/bin/env bash
set -euo pipefail

export RUSTUP_HOME="/rust"
export CARGO_HOME="$HOME/.cargo"
export PATH="/rust/bin:$CARGO_HOME/bin:$PATH"

dx build --release --web --ssg --package alexo-io

WEBDIR="$(
  find target/dx \
    -type d \
    -path '*/release/web' \
    -not -path '*/public' \
    | head -n1
)"

test -n "$WEBDIR"

./prerender.sh "$WEBDIR"

rm -rf site_public
mkdir -p site_public

cp -R "$WEBDIR/public/." site_public/
cp frontend/assets/images/og-image.png site_public/og-image.png

test -f site_public/index.html

if grep -q '<div id="main"></div>' site_public/index.html; then
  echo "ERROR: prerender failed"
  exit 1
fi
