# UAC - UASM Compiler

UAC (UASM Compiler) is a modular assembly language compiler that translates UASM assembly into native assembly code for multiple architectures and platforms.

> **⚠️ Development Status**: This compiler is currently under heavy development and not ready for production use.

## Quick Start

### Building from Source

1. **Clone the repository**

   ```bash
   git clone https://github.com/absurdish/uac.git
   cd uac
   ```

2. **Build the project**

   ```bash
   cargo build --release
   ```

3. **Run the compiler**
   ```bash
   ./target/release/uac hello.ua -o hello.s -t x86_64_linux
   ```

## Supported Targets

| Architecture | Aliases            | Platforms             | Example Target |
| :----------: | :----------------- | :-------------------- | :------------- |
|  **AMD64**   | `amd64`, `x86_64`  | Linux, macOS, Windows | `x86_64_macos` |
|  **ARM64**   | `arm64`, `aarch64` | Linux, macOS, Windows | `arm64_linux`  |

_Roadmap: Up to 20 architectures planned across multiple platforms._

---

## UASM Language Reference

### File Structure

UASM files are organized into sections that define different types of content:

```asm
section .text    ; Executable code
section .data    ; Initialized data
section .bss     ; Uninitialized data
section .rodata  ; Read-only data
```

### Labels

Define code locations and jump targets with a trailing colon:

```asm
main:
loop_start:
end_loop:
```

### Registers

UASM uses virtual registers that map to target architecture registers:

#### General Purpose

```asm
r0, r1, r2, ..., r31    ; Virtual general-purpose registers
```

#### Special Purpose

```asm
sp     ; Stack pointer
sb     ; Base/frame pointer
ip     ; Instruction pointer (read-only)
flags  ; Condition flags register
```

### Data Definition

Define and reserve memory with various data sizes:

```asm
; Data definition
msg     db "Hello, World!", 0xA, 0    ; Define bytes
numbers dw 1, 2, 3, 4                 ; Define words (16-bit)
matrix  dd 1.0, 2.0, 3.0, 4.0        ; Define double words (32-bit)
big_num dq 0x123456789ABCDEF0         ; Define quad words (64-bit)

; Memory reservation
buffer  resb 256    ; Reserve 256 bytes
array   resw 100    ; Reserve 100 words
stack   resd 50     ; Reserve 50 double words
heap    resq 25     ; Reserve 25 quad words
```

### Constants and Symbols

Define constants using the `equ` directive:

```asm
BUFFER_SIZE equ 1024
MAX_RETRY   equ 3
msg_len     equ 14
```

---

## Instruction Set

### Data Movement

```asm
mov   r0, r1        ; Move register to register
mov   r0, 42        ; Move immediate to register
lea   r1, [msg]     ; Load effective address
load  r0, [r1]      ; Load from memory address
store [r1], r0      ; Store to memory address
```

### Arithmetic Operations

```asm
add   r0, r1        ; Addition
sub   r0, r1        ; Subtraction
mul   r0, r1        ; Multiplication
div   r0, r1        ; Division
mod   r0, r1        ; Modulo
inc   r0            ; Increment
dec   r0            ; Decrement
neg   r0            ; Negate
```

### Logical & Bitwise Operations

```asm
and   r0, r1        ; Bitwise AND
or    r0, r1        ; Bitwise OR
xor   r0, r1        ; Bitwise XOR
not   r0            ; Bitwise NOT
shl   r0, 2         ; Shift left by 2
shr   r0, 1         ; Shift right by 1
```

### Comparison & Conditional Sets

```asm
cmp   r0, r1        ; Compare two values
test  r0, r1        ; Bitwise test
sete  r0            ; Set if equal
setne r0            ; Set if not equal
setl  r0            ; Set if less
setle r0            ; Set if less or equal
setg  r0            ; Set if greater
setge r0            ; Set if greater or equal
```

### Control Flow

```asm
jmp   label         ; Unconditional jump
je    label         ; Jump if equal
jne   label         ; Jump if not equal
jg    label         ; Jump if greater
jl    label         ; Jump if less
jge   label         ; Jump if greater or equal
jle   label         ; Jump if less or equal
call  function      ; Call function
ret                 ; Return from function
```

### System Calls

System calls use registers for arguments and return values:

```asm
; Arguments passed in r0, r1, r2, ...
; Return value in r0
syscall write       ; Write to file descriptor
syscall read        ; Read from file descriptor
syscall exit        ; Exit program
syscall open        ; Open file
syscall close       ; Close file descriptor
```

### Directives

```asm
global main         ; Export symbol globally
extern printf       ; Import external symbol
align 16            ; Align next data to 16-byte boundary
equ NAME, 123       ; Define named constant
```

### Comments

```asm
; This is a single-line comment
mov r0, 42    ; Inline comment
```

---

## Example Program

Here's a complete "Hello, World!" program in UASM:

```asm
section .data
    msg     db "Hello, World!", 0xA, 0
    msg_len equ 14

section .text
    global _start

_start:
    ; Write system call
    mov r0, 1          ; File descriptor: stdout
    lea r1, [msg]      ; Message address
    mov r2, msg_len    ; Message length
    syscall write

    ; Exit system call
    mov r0, 0          ; Exit code: success
    syscall exit
```

### Compilation

```bash
uac hello.ua -o hello.s -t x86_64_linux
```

This generates native assembly that can be assembled and linked:

```bash
# For Linux x86_64
as -64 hello.s -o hello.o
ld hello.o -o hello
./hello
```

---

## Contributing

UAC is in active development. Contributions, bug reports, and feature requests are welcome!

## License

MIT
