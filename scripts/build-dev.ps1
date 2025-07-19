Write-Host "== Compilazione dell'intero workspace (sviluppo) =="
cargo build

Write-Host "`n== Compilazione ileana-wasm per WebAssembly =="
cd ileana-wasm
wasm-pack build --target web
cd ..

Write-Host "`n== Compilazione completata. Pronto per test locale! =="
