#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "==> Building Dioxus frontend release bundle (WASM)..."
cd "$ROOT_DIR/crates/frontend"

if command -v dx &> /dev/null; then
    dx build --release
else
    echo "Warning: 'dx' CLI not found. Falling back to cargo check..."
    cargo check -p frontend
fi

echo "==> Building Axum backend release binary..."
cd "$ROOT_DIR"
cargo build --release -p backend

echo "==> Build complete! Output located at target/release/backend"

