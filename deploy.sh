#!/usr/bin/env bash
# Deploy alexan.dev — build frontend + server, then run on port 7777
#
# Usage:
#   ./deploy.sh          Build and start the server
#   ./deploy.sh stop     Stop the running server

# -------------------------
# Config (edit as needed)
# -------------------------
PORT=7777
SERVICE_NAME="alexo"
STAGE_DIR="site_public"
PID_FILE="${SERVICE_NAME}.pid"
LOG_FILE="${SERVICE_NAME}.log"
SERVER_BIN="target/release/server"

# -------------------------
# Pretty output helpers
# -------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status()   { echo -e "${BLUE}[INFO]${NC} $1"; }
print_success()  { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
print_error()    { echo -e "${RED}[ERROR]${NC} $1"; }

set -euo pipefail

stop_server() {
  if [[ -f "${PID_FILE}" ]]; then
    PID=$(cat "${PID_FILE}")
    if kill -0 "${PID}" 2>/dev/null; then
      kill "${PID}"
      print_success "Server stopped (pid ${PID})."
    else
      print_status "No running server at pid ${PID}."
    fi
    rm -f "${PID_FILE}"
  else
    print_status "No pid file found. Server is not running."
  fi
}

# -------------------------
# Handle "stop" command
# -------------------------
if [[ "${1:-}" == "stop" ]]; then
  stop_server
  exit 0
fi

# -------------------------
# Check prerequisites
# -------------------------
if ! command -v dx >/dev/null 2>&1; then
  print_error "dioxus CLI 'dx' not found. Install with: cargo install dioxus-cli"
  exit 1
fi

# -------------------------
# Build frontend (WASM)
# -------------------------
print_status "Bundling Dioxus frontend..."
if dx bundle --release --package alexo-io; then
  print_success "Frontend bundle completed."
else
  print_error "dx bundle failed."
  exit 1
fi

PUBDIR="$(find target/dx -type d -path '*/release/web/public' 2>/dev/null | head -n1 || true)"
if [[ -z "${PUBDIR}" ]]; then
  print_error "Could not find bundled public dir under target/dx/*/release/web/public"
  exit 1
fi

# -------------------------
# Build server (native)
# -------------------------
print_status "Building server..."
if cargo build --release -p server; then
  print_success "Server build completed."
else
  print_error "Server build failed."
  exit 1
fi

if [[ ! -f "${SERVER_BIN}" ]]; then
  print_error "Server binary not found at ${SERVER_BIN}"
  exit 1
fi

# -------------------------
# Stage static files
# -------------------------
rm -rf "${STAGE_DIR}"
mkdir -p "${STAGE_DIR}"
cp -R "${PUBDIR}/." "${STAGE_DIR}/"

# Inject FOUC fix into index.html (must load before WASM)
FOUC_STYLE='<style>html{background:#f5eed6}@media(prefers-color-scheme:dark){html{background:#1d2021}}body{opacity:0}body.ready{opacity:1;transition:opacity 80ms ease}</style>'
TMP="${STAGE_DIR}/index.html.tmp"
sed "s|<head>|<head>${FOUC_STYLE}|" "${STAGE_DIR}/index.html" > "${TMP}" && mv "${TMP}" "${STAGE_DIR}/index.html"

print_success "Staged static files to ./${STAGE_DIR}"

# -------------------------
# Restart server
# -------------------------
stop_server

print_status "Starting server on port ${PORT}..."
nohup "${SERVER_BIN}" --port "${PORT}" --dir "${STAGE_DIR}" --no-reload \
  > "${LOG_FILE}" 2>&1 &
echo $! > "${PID_FILE}"

sleep 1
if kill -0 "$(cat "${PID_FILE}")" 2>/dev/null; then
  print_success "Site is live at http://localhost:${PORT}"
  print_status "Logs: tail -f ${LOG_FILE}"
  print_status "Stop: ./deploy.sh stop"
else
  print_error "Server failed to start. Check ${LOG_FILE}"
  rm -f "${PID_FILE}"
  exit 1
fi
