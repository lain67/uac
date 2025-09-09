#!/bin/bash
set -euo pipefail

OUTDIR="./bin"
mkdir -p "$OUTDIR"

cargo build --release

TARGETS=(
  "amd64_linux"
  "amd64_macos"
  "amd64_windows"
  "amd32_linux"
  "amd32_windows"
  "arm64_linux"
  "arm32_linux"
  "arm64_macos"
  "arm64_windows"
 # "riscv64_linux"
 # "ppc64_linux"
)

# Find all .ua files in examples directory
UA_FILES=()
while IFS= read -r -d '' file; do
  UA_FILES+=("$file")
done < <(find ./examples -name "*.ua" -print0)

if [ ${#UA_FILES[@]} -eq 0 ]; then
  echo "No .ua files found in examples directory"
  exit 1
fi

count=0

for ua_file in "${UA_FILES[@]}"; do
  base_name=$(basename "$ua_file" .ua)

  echo "=== Processing $base_name.ua ==="

  for target in "${TARGETS[@]}"; do
    echo "  Compiling for $target"
    asm_file="$OUTDIR/${base_name}_${target}.s"
    ./target/release/uac "$ua_file" -o "$asm_file" -t "$target"
    count=$((count + 1))
    rm -rf "$asm_file"
  done
done

echo "All $count tests done."
