# 1. Compilazione WebAssembly
Write-Host "== Compilazione ileana-wasm per il web =="
cd ileana-wasm
wasm-pack build --target web

# 2. Avvio server HTTP (in background)
Write-Host "`n== Avvio server HTTP su localhost:4000 =="
Start-Process powershell -ArgumentList "-NoExit", "-Command", "basic-http-server . --addr 127.0.0.1:4000"

# 3. Apertura browser
Start-Sleep -Seconds 2  # aspetta che il server si avvii
$pagina = "http://localhost:4000/index.html"
Write-Host "`n== Apertura browser sulla pagina di test =="
Start-Process $pagina

# 4. Ritorna alla root del progetto
cd ..
