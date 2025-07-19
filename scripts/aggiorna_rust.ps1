Write-Host "== Verifica toolchain Rust =="
rustup show

Write-Host "`n== Verifica dipendenze del progetto nel workspace =="
cargo outdated --workspace

Write-Host "`n== Verifica strumenti installati con 'cargo install' =="
if (-not (Get-Command cargo-install-update -ErrorAction SilentlyContinue)) {
    Write-Host "'cargo-update' non trovato. Lo installo ora..."
    cargo install cargo-update
}

cargo install-update -l

Write-Host "`n== Verifica versione di wasm-pack =="
if (Get-Command wasm-pack -ErrorAction SilentlyContinue) {
    wasm-pack --version
} else {
    Write-Host "'wasm-pack' non è installato."
}
