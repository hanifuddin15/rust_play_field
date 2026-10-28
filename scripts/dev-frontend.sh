#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../crates/frontend"

if ! command -v dx &> /dev/null; then
    echo "Dioxus CLI ('dx') is not installed."
    echo "Install it with:"
    echo "  cargo install dioxus-cli"
    echo "  # or faster with cargo-binstall:"
    echo "  cargo binstall dioxus-cli"
    exit 1
fi

echo "==> Starting Dioxus frontend dev server with hot-reload..."
dx serve

