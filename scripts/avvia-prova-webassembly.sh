#!/bin/bash
set -e

# Calcola la directory assoluta dello script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 1. Compilazione WebAssembly
echo "== Compilazione ileana-wasm per il web =="
cd "$SCRIPT_DIR/../ileana-wasm"
wasm-pack build --target web

# 2. Avvio server HTTP (in background)
echo -e "\n== Avvio server HTTP su localhost:4000 =="
nohup basic-http-server . --addr 127.0.0.1:4000 > /dev/null 2>&1 &

# 3. Apertura browser
sleep 2
pagina="http://localhost:4000/index.html"
echo -e "\n== Apertura browser sulla pagina di test =="
if command -v xdg-open &> /dev/null; then
    xdg-open "$pagina"
else
    echo "Apri manualmente il browser su: $pagina"
fi

# 4. Ritorna alla root del progetto
cd "$SCRIPT_DIR/.."
