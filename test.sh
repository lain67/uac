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
  # Extract just the filename without path and extension
  base_name=$(basename "$ua_file" .ua)

  echo "=== Processing $base_name.ua ==="

  for target in "${TARGETS[@]}"; do
    echo "  Building for $target"

    asm_file="$OUTDIR/${base_name}_${target}.s"
    obj_file="$OUTDIR/${base_name}_${target}.o"
     
    ./target/release/uac "$ua_file" -o "$asm_file" -t "$target"
    count=$((count + 1))

    case "$target" in
      amd64_linux)
        as --64 "$asm_file" -o "$obj_file"
        ;;

      amd32_linux)
           if command -v i686-linux-gnu-as >/dev/null 2>&1; then
             i686-linux-gnu-as "$asm_file" -o "$obj_file"
           else
             llvm-mc --triple=i686-linux-gnu -arch=x86 \
                     -filetype=obj -o "$obj_file" "$asm_file"
           fi
      ;;

      arm64_linux)
        if command -v aarch64-linux-gnu-as >/dev/null 2>&1; then
          aarch64-linux-gnu-as "$asm_file" -o "$obj_file"
        else
          llvm-mc --triple=aarch64-linux-gnu -arch=aarch64 \
                  -filetype=obj -o "$obj_file" "$asm_file"
        fi
        ;;

      arm32_linux)
        if command -v aarch32-linux-gnu-as >/dev/null 2>&1; then
          aarch32-linux-gnu-as "$asm_file" -o "$obj_file"
        else
            llvm-mc --triple=armv7-none-linux-gnueabihf -arch=arm \
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

      amd32_windows)
        llvm-mc -arch=$( [[ $target == amd32* ]] && echo x86 || echo aarch32 ) \
                -filetype=obj -o "$obj_file" "$asm_file" --triple=$( [[ $target == amd32* ]] && echo x86-windows || echo aarch32-windows )
      ;;

      *)
        echo "Unknown target: $target" >&2
        exit 1
        ;;
    esac

    echo "  ✅ Success $obj_file"
    rm -rf "$obj_file"
    rm -rf "$asm_file"
  done
done

echo "All $count tests done."
