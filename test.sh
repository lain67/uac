#!/bin/bash
set -euo pipefail

UA_EXAMPLE="./examples/print_num.ua"
OUTDIR="./bin"
mkdir -p "$OUTDIR"

cargo build --release

TARGETS=(
  "amd64_linux"
  "amd64_macos"
  "amd64_windows"
  "arm64_linux"
  "arm64_macos"
  "arm64_windows"
  "riscv64_linux"
 # "ppc64_linux"
)

for target in "${TARGETS[@]}"; do
  echo "=== Building for $target ==="

  asm_file="$OUTDIR/hello_${target}.s"
  obj_file="$OUTDIR/hello_${target}.o"

  ./target/release/uac "$UA_EXAMPLE" -o "$asm_file" -t "$target"

  case "$target" in
    amd64_linux)
      as --64 "$asm_file" -o "$obj_file"
      ;;

    arm64_linux)
      if command -v aarch64-linux-gnu-as >/dev/null 2>&1; then
        aarch64-linux-gnu-as "$asm_file" -o "$obj_file"
      else
        llvm-mc --triple=aarch64-linux-gnu -arch=aarch64 \
                -filetype=obj -o "$obj_file" "$asm_file"
      fi
      ;;

    riscv64_linux)
      if command -v riscv64-linux-gnu-as >/dev/null 2>&1; then
        riscv64-linux-gnu-as -march=rv64im "$asm_file" -o "$obj_file"
      else
        llvm-mc --triple=riscv64-linux-gnu -arch=riscv64 \
                -mattr=+m \
                -filetype=obj -o "$obj_file" "$asm_file"
      fi
      ;;

    ppc64_linux)
      if command -v powerpc64-linux-gnu-as >/dev/null 2>&1; then
        powerpc64-linux-gnu-as "$asm_file" -o "$obj_file"
      else
        llvm-mc --triple=powerpc64-linux-gnu -arch=ppc64 \
                -filetype=obj -o "$obj_file" "$asm_file"
      fi
      ;;

    amd64_macos|arm64_macos)
      llvm-mc -arch=$( [[ $target == amd64* ]] && echo x86-64 || echo arm64 ) \
              -filetype=obj -o "$obj_file" "$asm_file"
      ;;

    amd64_windows|arm64_windows)
      llvm-mc -arch=$( [[ $target == amd64* ]] && echo x86-64 || echo aarch64 ) \
              -filetype=obj -o "$obj_file" "$asm_file" --triple=$( [[ $target == amd64* ]] && echo x86_64-windows || echo aarch64-windows )
      ;;

    *)
      echo "Unknown target: $target" >&2
      exit 1
      ;;
  esac

  echo "✅ Success $obj_file"
  rm -rf "$obj_file"
  rm -rf "$asm_file"
done

echo "All tests done."
