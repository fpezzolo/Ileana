#!/bin/bash

echo "Avvio test con cargo test..."
cargo test --workspace -- #--nocapture
