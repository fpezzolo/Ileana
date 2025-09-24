#!/bin/bash

echo "== Compilazione dell'intero workspace (sviluppo) =="
cargo build

echo -e "\n== Compilazione ileana-wasm per WebAssembly =="
cd ileana-wasm
wasm-pack build --target web
cd ..

echo -e "\n== Compilazione completata. Pronto per test locale! =="
