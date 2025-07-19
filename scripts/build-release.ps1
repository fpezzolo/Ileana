Write-Host "== Compilazione dell'intero workspace in modalità rilascio =="
cargo build --release

Write-Host "`n== Compilazione ileana-wasm per WebAssembly ottimizzato =="
cd ileana-wasm
wasm-pack build --release --target web
cd ..

Write-Host "`n== Rilascio completato. Modulo ottimizzato pronto! =="
