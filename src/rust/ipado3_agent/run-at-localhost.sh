#!/usr/bin/env bash
set -e
dir=$(dirname "$0")
cd "$dir"
pwd=$(pwd)
cd ".."
RUST_LOG=trace cargo run -p ipado3_agent -- --op-mode ${1:-dev} "$pwd/dat"
# RUST_LOG=trace cargo run -p ipado3_agent -- --op-mode demo ~/abc/src/rust/ipado3_agent/dat
