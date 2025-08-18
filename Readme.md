# UAC

UAC or UASM Compiler is a compiler of the UASM assembly language, that is designed to be modular version of the assembly
that compiles into native assembly code.

The compiler is currently under the heavy development and not ready for use.

## Usage

### Building from Local

First clone the UAC repository:

```sh
git clone https://github.com/absurdish/uac.git
```

Then build the project:

```sh
cd uac
cargo build --release
```

And run the compiled binary:

```sh
./target/release/uac hello.ua -o hello.s -t x86_64_linux
```

## Supported Architectures

| Architecture | Aliases            | Platforms             | Example        |
| :----------: | ------------------ | --------------------- | -------------- |
|    AMD64     | `amd64`, `x86_64`  | Linux, macOS, Windows | `x86_64_macos` |
|    ARM64     | `arm64`, `aarch64` | Linux, macOS, Windows | `arm64_linux`  |

Up to 20 architectures are planned to be implemented into the compiler, on multiple platforms.

## Documentation

### Sections

UASM file is divided into sections:

```asm
section .text ; executable code
section .data ; initialized data
section .bss ; uninitialized data
section .rodata ; read-only data
```

Where the code lives in `.text`, variables in `.data/.bss` and constants in `.rodata`.

### Labels

Labels are used to mark specific locations in the code. They are defined with a trailing colon (`:`):

```asm
main:
loop_start:
```

### Symbols and Registers

Symbols or the names for constants or memory offsets are defined similar to assembly languages:

```asm
len equ 67
```

Since each chip architecture supports different registers,
UASM uses virtual registers to abstract the differences between architectures:

```asm
r0, r1, r2, ..., r31
```

Which are mapped to target ISA registers. As for the special-purpose registers:

```asm
sp ; stack pointer
sb ; base/frame pointer
ip ; instruction pointer (read-only)
flags ; condition registers
```

data definition

```asm
msg db "Hello, World!", 0xA, 0
numbers dw 1, 2, 3, 4
matrix dd 1.0, 2.0, 3.0, 4.0
buffer resb 256 ; reserve 256 bytes
```

- `db` = bytes
- `dw` = word
- `dd` = double word
- `dq` = quad word
- `resX` = reserved unititialized storage

### Instructions

data movement

```asm
mov r0, r1
mov r0, 42
lea r1, [msg] ; load address of msg
load r0, [r1] ; load from memory
store [r1], r0 ; store into memory
```

arithmetic

```asm
add   r0, r1
sub   r0, r1
mul   r0, r1
div   r0, r1
mod   r0, r1
inc   r0
dec   r0
neg   r0
```

logic & bitwise

```asm
and   r0, r1
or    r0, r1
xor   r0, r1
not   r0
shl   r0, 2
shr   r0, 1
```

comparasion & flags

```asm
cmp   r0, r1
sete  r0
setne r0
setl  r0
setle r0
setg  r0
setge r0
test r0, r1
```

control flow

```asm
jmp    label
je     label      ; jump if equal
jne    label      ; jump if not equal
jg     label      ; jump if greater
jl     label      ; jump if less
jge    label      ; jump if greater/equal
jle    label      ; jump if less/equal
call   function
ret
```

system calls

```asm
; arguments in r0, r1, r2, ...
; return value in r0

syscall write
syscall read
syscall exit
syscall open
syscall close
```

directives

```asm
global main        ; export symbol
extern printf      ; import external symbol
align 16           ; align next data
equ    NAME, 123   ; constant
```

comments

```asm
; single line comment
```

### Example

```asm
section .data
msg db "Hello, World!", 0xA, 0
msg_len equ 14

section .text
global _start
_start:
    ; write syscall
    mov r0, 1          ; stdout
    lea r1, [msg]      ; message address
    mov r2, msg_len    ; message length
    syscall write
    
    ; exit syscall
    mov r0, 0          ; exit code
    syscall exit
```
