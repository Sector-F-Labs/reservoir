#!/bin/sh
# Builds the wasm app into admin-ui/dist, which the reservoir binary embeds.
# Needs the wasm32-unknown-unknown target and wasm-bindgen-cli matching the
# wasm-bindgen version in Cargo.lock.
set -eu
cd "$(dirname "$0")"
cargo build --release --target wasm32-unknown-unknown
rm -rf dist
mkdir -p dist/pkg
wasm-bindgen --target web --no-typescript --out-dir dist/pkg \
    target/wasm32-unknown-unknown/release/reservoir_admin_ui.wasm
cp web/index.html dist/index.html
