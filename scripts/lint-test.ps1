Write-Host " Linting con Clippy..."
cargo clippy --workspace --all-targets -- -D warnings

Write-Host " Avvio test..."
cargo test --workspace -- --nocapture
