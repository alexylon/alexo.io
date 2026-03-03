### Fullstack Rust personal website — Dioxus/WASM frontend + axum server

![](https://github.com/alexylon/alexo.io/actions/workflows/rust.yml/badge.svg)

Live at [alexo.io](https://alexo.io), hosted on a Raspberry Pi.

### Project structure

```
frontend/   Dioxus WASM app (UI, components, assets)
server/     axum static file server (compression, SPA fallback, security headers)
```

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

### Run locally (dev)

```bash
dx serve --package alexo-io
```

### Deploy (production)

```bash
./deploy.sh        # build frontend + server, start on port 7777
./deploy.sh stop   # stop the server
```

### Server options

```
server --port 7777 --dir ./site_public --no-reload
```

| Flag | Default | Description |
|------|---------|-------------|
| `--port`, `-p` | `3030` | Port to listen on |
| `--dir`, `-d` | `.` | Directory to serve |
| `--no-reload` | off | Disable live reload and file watching |

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
