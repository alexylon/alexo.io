### My personal web resume — written in Rust, powered by Dioxus, compiled to WebAssembly (WASM)

![](https://github.com/alexylon/alexo.io/actions/workflows/rust.yml/badge.svg)

Live at [alexo.io](https://alexo.io), hosted on a Raspberry Pi.

### Prerequisites (one-time setup)

1. Install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Install `cargo-binstall`:

```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
```

3. Install Dioxus CLI:

```bash
cargo binstall dioxus-cli
```

### Run locally

```bash
dx serve
```

Open http://localhost:8080 to view the app.

### Generate static site

```bash
dx bundle
```

Output: `target/dx/alexo-io/release/web/public`

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
