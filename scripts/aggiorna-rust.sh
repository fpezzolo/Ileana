#!/bin/bash

echo "== Verifica toolchain Rust =="
rustup show

echo -e "\n== Verifica dipendenze del progetto nel workspace =="
cargo outdated --workspace

echo -e "\n== Verifica strumenti installati con 'cargo install' =="
if ! command -v cargo-install-update &> /dev/null; then
    echo "'cargo-update' non trovato. Lo installo ora..."
    cargo install cargo-update
fi

cargo install-update -l

echo -e "\n== Verifica versione di wasm-pack =="
if command -v wasm-pack &> /dev/null; then
    wasm-pack --version
else
    echo "'wasm-pack' non è installato."
fi
