# UAC - UASM Compiler

UAC (UASM Compiler) is a modular assembly language compiler that translates UASM assembly into native assembly code for multiple architectures and platforms.

> **⚠️ Development Status**: This compiler is currently under development and not ready for production use. Contributions are Welcome!

## Quick Start

### Building from Source

1. **Clone the repository**

   ```bash
   git clone https://github.com/lain67/uac.git
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

| Architecture | Aliases                          | Platforms                              | Example Target  |
| :----------: | :------------------------------- | :------------------------------------- | :-------------- |
|  **AMD64**   | `amd64`, `x86_64`, `amd`, ...    | Linux, macOS, Windows                  | `x86_64_macos`  |
|  **ARM64**   | `arm64`, `aarch64`, `arm`, ...   | Linux, macOS, Windows                  | `arm64_linux`   |
|  **AMD32**   | `amd32`, `x86`, `i386`, ...      | Linux, windows                         | `x86_windows`   |
|  **ARM32**   | `arm32`, `aarch32`, `armv7`, ... | Linux                                  | `arm32_linux`   |
|  Unstable:   |                                  |                                        |                 |
|  **RISC-V**  | `riscv64`, `riscv`, `riscv64gc`  | Linux (mainstream), BSD (experimental) | `riscv64_linux` |
|  **PPC64**   | `ppc64`, `ppc64le`, `powerpc64`  | Linux, BSD, AIX                        | `ppc64_linux`   |

_Roadmap: Up to 20 architectures planned across multiple platforms._

## Compilation

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

## Contributing

UAC is in active development. Contributions, bug reports, and feature requests are welcome!

## License

MIT

## UASM Language Reference

### File Structure

UASM files are divided into **sections**:

```
section .text     ; Executable code
section .data     ; Initialized data
section .bss      ; Uninitialized data
section .rodata   ; Read-only data
```

---

### Labels

Use labels to mark **locations in code**:

```
label_name:       ; Define a label
```

Example:

```
main:
loop_start:
end_loop:
```

---

### Registers

#### General Purpose

```
r0, r1, r2, ..., r31    ; Virtual GPRs
```

#### Special Purpose

```
sp      ; Stack pointer
sb      ; Base/frame pointer
ip      ; Instruction pointer (read-only)
flags   ; Condition flags register
```

---

### Data Definition

#### Define Memory with Initial Values

```
db      ; Define bytes
dw      ; Define words (16-bit)
dd      ; Define double words (32-bit)
dq      ; Define quad words (64-bit)
```

Example:

```
msg      db "Hello, World!", 0xA, 0
numbers  dw 1, 2, 3, 4
matrix   dd 1.0, 2.0, 3.0, 4.0
big_num  dq 0x123456789ABCDEF0
```

#### Reserve Memory without Initial Values

```
resb    n      ; Reserve n bytes
resw    n      ; Reserve n words
resd    n      ; Reserve n double words
resq    n      ; Reserve n quad words
```

Example:

```
buffer  resb 256
array   resw 100
stack   resd 50
heap    resq 25
```

---

### Constants and Symbols

Define **named constants**:

```
equ NAME, value
```

Example:

```
BUFFER_SIZE equ 1024
MAX_RETRY   equ 3
msg_len     equ 14
```

---

### Data Movement

```
mov    dest, src       ; Move value from src to dest
lea    dest, addr      ; Load effective address
load   dest, [addr]    ; Load from memory
store  [addr], src     ; Store to memory
```

#### Conditional Moves

```
cmovCC dest, src       ; Move if condition CC is met
```

Conditions:

| CC  | Meaning              |
| --- | -------------------- |
| EQ  | Equal / Zero         |
| NE  | Not equal / Not zero |
| LT  | Less                 |
| LE  | Less or equal        |
| GT  | Greater              |
| GE  | Greater or equal     |
| OV  | Overflow             |
| NO  | Not overflow         |
| S   | Sign                 |
| NS  | Not sign             |
| P   | Parity               |
| NP  | Not parity           |
| A   | Above (unsigned)     |
| AE  | Above or equal       |
| B   | Below (unsigned)     |
| BE  | Below or equal       |

---

### Stack Operations

```
push   src        ; Push value to stack
pop    dest       ; Pop value from stack
pusha             ; Push all general-purpose registers
popa              ; Pop all general-purpose registers
enter  frameSize, nestingLevel  ; Create stack frame
leave                     ; Delete stack frame
```

---

### Arithmetic Operations

```
add    dest, src       ; Addition
sub    dest, src       ; Subtraction
mul    dest, src       ; Multiplication
imul   dest, src       ; Integer multiplication
div    dest, src       ; Division
idiv   dest, src       ; Integer division
mod    dest, src       ; Modulo
inc    dest            ; Increment
dec    dest            ; Decrement
neg    dest            ; Negate
```

---

### Logical & Bitwise Operations

```
and    dest, src       ; Bitwise AND
or     dest, src       ; Bitwise OR
xor    dest, src       ; Bitwise XOR
not    dest            ; Bitwise NOT
andn   dest, src       ; Bitwise AND NOT
shl    dest, imm       ; Shift left
shr    dest, imm       ; Shift right
sal    dest, imm       ; Arithmetic shift left
sar    dest, imm       ; Arithmetic shift right
rol    dest, imm       ; Rotate left
ror    dest, imm       ; Rotate right
rcl    dest, imm       ; Rotate through carry left
rcr    dest, imm       ; Rotate through carry right
bextr  dest, src, imm  ; Bit extract
bsf    dest, src       ; Bit scan forward
bsr    dest, src       ; Bit scan reverse
```

---

### Comparison & Conditional Sets

```
cmp    dest, src       ; Compare two values
test   dest, src       ; Bitwise test
bt     dest, bit       ; Test bit
btr    dest, bit       ; Test bit and reset
bts    dest, bit       ; Test bit and set
btc    dest, bit       ; Test bit and complement
setCC  dest            ; Set if condition CC is met
```

---

### String Operations

```
cmps   src1, src2      ; Compare strings
scas   src, val        ; Scan string
stos   dest, src       ; Store string
lods   dest, src       ; Load string
movs   dest, src       ; Move string
```

---

### Data Conversion

```
cbw   dest             ; Convert byte to word
cwd   dest             ; Convert word to double word
cdq   dest             ; Convert double word to quad word
cqo   dest             ; Convert quad word to oct word
cwde  dest             ; Convert word to double word
cdqe  dest             ; Convert double word to quad word
```

---

### Control Flow

```
jmp    label           ; Unconditional jump
jCC    label           ; Conditional jump
loopCC label           ; Loop with condition
call   label           ; Call function
ret                     ; Return from function
```

---

### I/O

```
in     dest, port       ; Input from port
out    port, src        ; Output to port
ins    dest, port       ; Input string from port
outs   port, src        ; Output string to port
```

---

### System & CPU Operations

```
cpuid                   ; CPU identification
syscall name            ; System call with registers r0, r1... for arguments
lfence                  ; Load fence
sfence                  ; Store fence
mfence                  ; Memory fence
prefetch addr           ; Prefetch data into cache
clflush addr            ; Flush cache line
clwb addr               ; Writeback cache line
```

---

### Directives

```
global symbol           ; Export symbol globally
extern symbol           ; Import external symbol
align n                 ; Align next data to n-byte boundary
equ name, value         ; Define named constant
```

---

### Comments

```
; Single-line comment
mov r0, 42   ; Inline comment
```

---

### Example Program

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
