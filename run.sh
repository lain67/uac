#!/bin/bash

# UASM Build Script
# Usage: ./run.sh <program_name>
# Example: ./run.sh hello

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <program_name>"
    echo "Example: $0 hello"
    exit 1
fi

PROGRAM_NAME="$1"
UASM_FILE="./examples/${PROGRAM_NAME}.ua"
ASM_FILE="./bin/${PROGRAM_NAME}.s"
OBJ_FILE="./bin/${PROGRAM_NAME}.o"
EXECUTABLE="./bin/${PROGRAM_NAME}"

if [ ! -f "$UASM_FILE" ]; then
    echo "Error: Source file '$UASM_FILE' not found"
    exit 1
fi

echo "Building '$PROGRAM_NAME'..."

echo "1. Compiling UASM to x86_64 assembly..."
cargo run "$UASM_FILE" -o "$ASM_FILE" -t x86_64_linux

if [ $? -ne 0 ]; then
    echo "Error: UASM compilation failed"
    exit 1
fi

echo "2. Assembling to object file..."
as --64 "$ASM_FILE" -o "$OBJ_FILE"

if [ $? -ne 0 ]; then
    echo "Error: Assembly failed"
    exit 1
fi

echo "3. Linking executable..."
ld "$OBJ_FILE" -o "$EXECUTABLE"

if [ $? -ne 0 ]; then
    echo "Error: Linking failed"
    exit 1
fi

chmod +x "$EXECUTABLE"
rm -f "$ASM_FILE" "$OBJ_FILE"

$EXECUTABLE
