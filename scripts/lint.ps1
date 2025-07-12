Write-Host " Avvio linting con Clippy..."
cargo clippy --workspace --all-targets -- -D warnings
