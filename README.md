# alexo.io

Full-stack Rust personal website — Dioxus/WASM frontend + axum server.

![](https://github.com/alexylon/alexo.io/actions/workflows/rust.yml/badge.svg)

Live at [alexo.io](https://alexo.io), hosted on a Raspberry Pi.

## Features

- **Gruvbox dark/light theme** — system preference detection + manual toggle
- **Scroll-aware navigation** — direction-sensitive active section highlighting
- **Accessibility** — toolbar keyboard pattern (Tab into nav, arrow keys between buttons, Escape to leave), Enter/Space to activate, focus-visible rings, aria labels
- **SEO** — Open Graph meta tags, semantic HTML
- **Performance** — WASM frontend, axum HTTP response compression, content-hashed asset caching, non-blocking font loading

## Project structure

```
frontend/   Dioxus WASM app (UI, components, assets)
server/     axum static file server (compression, SPA fallback, live reload, security headers)
```

## Getting started

### Prerequisites

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. Install Dioxus CLI:

```bash
cargo install cargo-binstall
cargo binstall dioxus-cli
```

### Development

```bash
dx serve --package alexo-io
```

### Production

```bash
./deploy.sh        # build frontend + server, start on port 7777
./deploy.sh stop   # stop the server
```

## Server options

```
server --port 7777 --dir ./site_public --no-reload
```

| Flag | Default | Description |
|------|---------|-------------|
| `--port`, `-p` | `3030` | Port to listen on |
| `--dir`, `-d` | `.` | Directory to serve |
| `--no-reload` | off | Disable live reload and file watching |

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)
