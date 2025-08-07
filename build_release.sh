#!/bin/bash
set -e

targets=(
  "x86_64-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
)

for target in "${targets[@]}"; do
  echo "🔧 Building for $target"
  cargo build --release --target "$target"
done
