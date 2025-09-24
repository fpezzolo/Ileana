#!/bin/bash

echo "Linting con Clippy..."
cargo clippy --workspace --all-targets -- -D warnings

echo "Avvio test..."
cargo test --workspace -- --nocapture
