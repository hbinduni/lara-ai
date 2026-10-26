#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
cargo install cargo-audit --version 0.22.2 --locked
cargo audit
