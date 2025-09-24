#!/bin/bash

echo "== Compilazione dell'intero workspace in modalità rilascio =="
cargo build --release

echo -e "\n== Compilazione ileana-wasm per WebAssembly ottimizzato =="
cd ileana-wasm
wasm-pack build --release --target web
cd ..

echo -e "\n== Rilascio completato. Modulo ottimizzato pronto! =="
