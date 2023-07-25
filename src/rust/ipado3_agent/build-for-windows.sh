#!/usr/bin/env bash

set -e
dir=$(dirname "$0")
cd "$dir/.."

rustup target add x86_64-pc-windows-gnu
sudo apt install -y mingw-w64
cargo build --target x86_64-pc-windows-gnu --release -p ipado3_agent
ls -lAh target/x86_64-pc-windows-gnu/release/ipado3_agent.exe
