use crate::{
    arch::Architecture,
    platform::{Format, Platform},
};
use std::collections::HashMap;

pub mod codegen;
pub mod parser;

#[derive(Debug, Clone)]
pub struct TargetTriple {
    pub architecture: Architecture,
    pub platform: Platform,
    pub format: Format,
}

impl TargetTriple {
    /// Generic constructor that picks the correct binary format
    pub fn new(architecture: Architecture, platform: Platform) -> Self {
        let format = match platform {
            Platform::Linux => Format::ELF,
            Platform::BSD => Format::ELF,
            Platform::Solaris => Format::ELF,
            Platform::Windows => Format::COFF,
            Platform::MacOS => Format::MachO,
            Platform::DOS => Format::MZ,
            Platform::Embedded => Format::ELF, // most toolchains emit ELF for bare metal
        };

        TargetTriple {
            architecture,
            platform,
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Section {
    Text,
    Data,
    Bss,
    Rodata,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum DataSize {
    Byte,
    Word,
    Dword,
    Qword,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    /// Define code locations and jump targets.
    ///
    /// Example:
    /// ```asm
    /// main:
    /// loop_start:
    /// end_loop:
    /// ```
    Label(String),

    //
    // Data Movement
    //
    /// Move register or immediate to register
    ///
    /// Example:
    /// ```asm
    /// mov r0, r1
    /// mov r0, 42
    /// ```
    Mov((String, String)),

    /// Load effective address of memory into register
    ///
    /// Example:
    /// ```asm
    /// lea r0, [r1 + 4]
    /// lea r1, [msg]
    /// ```
    Lea((String, String)),

    /// Load from memory address
    ///
    /// Example:
    /// ```asm
    /// load r0, [r1]
    /// load r1, [msg]
    /// ```
    Load((String, String)),

    /// Store to memory address
    ///
    /// Example:
    /// ```asm
    /// store [r1], r0
    /// store [msg], r1
    /// ```
    Store((String, String)),

    //
    // Conditional Moves
    //
    /// Move if equal/zero
    ///
    /// Example:
    /// ```asm
    /// cmoveq r0, r1
    /// ```
    CmovEq((String, String)),

    /// Move if not equal/not zero
    ///
    /// Example:
    /// ```asm
    /// cmovne r0, r1
    /// ```
    CmovNe((String, String)),

    /// Move if less
    ///
    /// Example:
    /// ```asm
    /// cmovlt r0, r1
    /// ```
    CmovLt((String, String)),

    /// Move if less or equal
    ///
    /// Example:
    /// ```asm
    /// cmovle r0, r1
    /// ```
    CmovLe((String, String)),

    /// Move if greater
    ///
    /// Example:
    /// ```asm
    /// cmovgt r0, r1
    /// ```
    CmovGt((String, String)),

    /// Move if greater or equal
    ///
    /// Example:
    /// ```asm
    /// cmovge r0, r1
    /// ```
    CmovGe((String, String)),

    /// Move if overflow
    ///
    /// Example:
    /// ```asm
    /// cmovov r0, r1
    /// ```
    CmovOv((String, String)),

    /// Move if not overflow
    ///
    /// Example:
    /// ```asm
    /// cmovno r0, r1
    /// ```
    CmovNo((String, String)),

    /// Move if sign
    ///
    /// Example:
    /// ```asm
    /// cmovs r0, r1
    /// ```
    CmovS((String, String)),

    /// Move if not sign
    ///
    /// Example:
    /// ```asm
    /// cmovns r0, r1
    /// ```
    CmovNs((String, String)),

    /// Move if parity
    ///
    /// Example:
    /// ```asm
    /// cmovp r0, r1
    /// ```
    CmovP((String, String)),

    /// Move if not parity
    ///
    /// Example:
    /// ```asm
    /// cmovnp r0, r1
    /// ```
    CmovNp((String, String)),

    /// Move if above (unsigned)
    ///
    /// Example:
    /// ```asm
    /// cmova r0, r1
    /// ```
    CmovA((String, String)),

    /// Move if above or equal
    ///
    /// Example:
    /// ```asm
    /// cmovae r0, r1
    /// ```
    CmovAe((String, String)),

    /// Move if below (unsigned)
    ///
    /// Example:
    /// ```asm
    /// cmovb r0, r1
    /// ```
    CmovB((String, String)),

    /// Move if below or equal
    ///
    /// Example:
    /// ```asm
    /// cmovbe r0, r1
    /// ```
    CmovBe((String, String)),

    //
    // Stack Operations
    //
    /// Push value to stack
    ///
    /// Example:
    /// ```asm
    /// push r0
    /// push 42
    /// ```
    Push(String),

    /// Pop value from stack
    ///
    /// Example:
    /// ```asm
    /// pop r0
    /// ```
    Pop(String),

    /// Push all general-purpose registers
    ///
    /// Example:
    /// ```asm
    /// pusha
    /// ```
    Pusha,

    /// Pop all general-purpose registers
    ///
    /// Example:
    /// ```asm
    /// popa
    /// ```
    Popa,

    /// Create stack frame
    ///
    /// Example:
    /// ```asm
    /// enter 16, 0
    /// ```
    Enter((String, String)),

    /// Delete stack frame
    ///
    /// Example:
    /// ```asm
    /// leave
    /// ```
    Leave,

    //
    // Arithmetic Operations
    //
    /// Addition
    ///
    /// Example:
    /// ```asm
    /// add r0, r1
    /// add r0, 5
    /// ```
    Add((String, String)),

    /// Subtraction
    ///
    /// Example:
    /// ```asm
    /// sub r0, r1
    /// sub r0, 10
    /// ```
    Sub((String, String)),

    /// Multiplication
    ///
    /// Example:
    /// ```asm
    /// mul r0, r1
    /// ```
    Mul((String, String)),

    /// Integer multiplication
    ///
    /// Example:
    /// ```asm
    /// imul r0, r1
    /// ```
    Imul((String, String)),

    /// Division
    ///
    /// Example:
    /// ```asm
    /// div r0, r1
    /// ```
    Div((String, String)),

    /// Integer division
    ///
    /// Example:
    /// ```asm
    /// idiv r0, r1
    /// ```
    Idiv((String, String)),

    /// Modulo
    ///
    /// Example:
    /// ```asm
    /// mod r0, r1
    /// ```
    Mod((String, String)),

    /// Increment
    ///
    /// Example:
    /// ```asm
    /// inc r0
    /// ```
    Inc(String),

    /// Decrement
    ///
    /// Example:
    /// ```asm
    /// dec r0
    /// ```
    Dec(String),

    /// Negation
    ///
    /// Example:
    /// ```asm
    /// neg r0
    /// ```
    Neg(String),

    //
    // Logical & Bitwise Operations
    //
    /// Bitwise AND
    ///
    /// Example:
    /// ```asm
    /// and r0, r1
    /// and r0, 0xFF
    /// ```
    And((String, String)),

    /// Bitwise OR
    ///
    /// Example:
    /// ```asm
    /// or r0, r1
    /// or r0, 0x80
    /// ```
    Or((String, String)),

    /// Bitwise XOR
    ///
    /// Example:
    /// ```asm
    /// xor r0, r1
    /// xor r0, r0  ; Clear register
    /// ```
    Xor((String, String)),

    /// Bitwise NOT
    ///
    /// Example:
    /// ```asm
    /// not r0
    /// ```
    Not(String),

    /// Bitwise AND NOT
    ///
    /// Example:
    /// ```asm
    /// andn r0, r1
    /// ```
    Andn((String, String)),

    /// Shift left
    ///
    /// Example:
    /// ```asm
    /// shl r0, 2
    /// shl r1, r2
    /// ```
    Shl((String, String)),

    /// Shift right
    ///
    /// Example:
    /// ```asm
    /// shr r0, 1
    /// shr r1, r2
    /// ```
    Shr((String, String)),

    /// Arithmetic shift left
    ///
    /// Example:
    /// ```asm
    /// sal r0, 2
    /// ```
    Sal((String, String)),

    /// Arithmetic shift right
    ///
    /// Example:
    /// ```asm
    /// sar r0, 1
    /// ```
    Sar((String, String)),

    /// Rotate left
    ///
    /// Example:
    /// ```asm
    /// rol r0, 3
    /// ```
    Rol((String, String)),

    /// Rotate right
    ///
    /// Example:
    /// ```asm
    /// ror r0, 2
    /// ```
    Ror((String, String)),

    /// Rotate through carry left
    ///
    /// Example:
    /// ```asm
    /// rcl r0, 1
    /// ```
    Rcl((String, String)),

    /// Rotate through carry right
    ///
    /// Example:
    /// ```asm
    /// rcr r0, 1
    /// ```
    Rcr((String, String)),

    /// Bit extract
    ///
    /// Example:
    /// ```asm
    /// bextr r0, r1, 8
    /// ```
    Bextr((String, String, String)),

    /// Bit scan forward
    ///
    /// Example:
    /// ```asm
    /// bsf r0, r1
    /// ```
    Bsf((String, String)),

    /// Bit scan reverse
    ///
    /// Example:
    /// ```asm
    /// bsr r0, r1
    /// ```
    Bsr((String, String)),

    //
    // Comparison & Conditional Sets
    //
    /// Compare two values
    ///
    /// Example:
    /// ```asm
    /// cmp r0, r1
    /// cmp r0, 42
    /// ```
    Cmp((String, String)),

    /// Bitwise test
    ///
    /// Example:
    /// ```asm
    /// test r0, r1
    /// test r0, 0xFF
    /// ```
    Test((String, String)),

    /// Test bit
    ///
    /// Example:
    /// ```asm
    /// bt r0, 7
    /// ```
    Bt((String, String)),

    /// Test bit and reset
    ///
    /// Example:
    /// ```asm
    /// btr r0, 3
    /// ```
    Btr((String, String)),

    /// Test bit and set
    ///
    /// Example:
    /// ```asm
    /// bts r0, 5
    /// ```
    Bts((String, String)),

    /// Test bit and complement
    ///
    /// Example:
    /// ```asm
    /// btc r0, 2
    /// ```
    Btc((String, String)),

    /// Set if equal/zero
    ///
    /// Example:
    /// ```asm
    /// seteq r0
    /// ```
    SetEq(String),

    /// Set if not equal/not zero
    ///
    /// Example:
    /// ```asm
    /// setne r0
    /// ```
    SetNe(String),

    /// Set if less
    ///
    /// Example:
    /// ```asm
    /// setlt r0
    /// ```
    SetLt(String),

    /// Set if less or equal
    ///
    /// Example:
    /// ```asm
    /// setle r0
    /// ```
    SetLe(String),

    /// Set if greater
    ///
    /// Example:
    /// ```asm
    /// setgt r0
    /// ```
    SetGt(String),

    /// Set if greater or equal
    ///
    /// Example:
    /// ```asm
    /// setge r0
    /// ```
    SetGe(String),

    /// Set if overflow
    ///
    /// Example:
    /// ```asm
    /// setov r0
    /// ```
    SetOv(String),

    /// Set if not overflow
    ///
    /// Example:
    /// ```asm
    /// setno r0
    /// ```
    SetNo(String),

    /// Set if sign
    ///
    /// Example:
    /// ```asm
    /// sets r0
    /// ```
    SetS(String),

    /// Set if not sign
    ///
    /// Example:
    /// ```asm
    /// setns r0
    /// ```
    SetNs(String),

    /// Set if parity
    ///
    /// Example:
    /// ```asm
    /// setp r0
    /// ```
    SetP(String),

    /// Set if not parity
    ///
    /// Example:
    /// ```asm
    /// setnp r0
    /// ```
    SetNp(String),

    /// Set if above (unsigned)
    ///
    /// Example:
    /// ```asm
    /// seta r0
    /// ```
    SetA(String),

    /// Set if above or equal
    ///
    /// Example:
    /// ```asm
    /// setae r0
    /// ```
    SetAe(String),

    /// Set if below (unsigned)
    ///
    /// Example:
    /// ```asm
    /// setb r0
    /// ```
    SetB(String),

    /// Set if below or equal
    ///
    /// Example:
    /// ```asm
    /// setbe r0
    /// ```
    SetBe(String),

    //
    // String Operations
    //
    /// Compare strings
    ///
    /// Example:
    /// ```asm
    /// cmps r0, r1
    /// ```
    Cmps((String, String)),

    /// Scan string
    ///
    /// Example:
    /// ```asm
    /// scas r0, 0
    /// ```
    Scas((String, String)),

    /// Store string
    ///
    /// Example:
    /// ```asm
    /// stos [r0], r1
    /// ```
    Stos((String, String)),

    /// Load string
    ///
    /// Example:
    /// ```asm
    /// lods r0, [r1]
    /// ```
    Lods((String, String)),

    /// Move string
    ///
    /// Example:
    /// ```asm
    /// movs [r0], [r1]
    /// ```
    Movs((String, String)),

    //
    // Data Conversion
    //
    /// Convert byte to word
    ///
    /// Example:
    /// ```asm
    /// cbw r0
    /// ```
    Cbw(String),

    /// Convert word to double word
    ///
    /// Example:
    /// ```asm
    /// cwd r0
    /// ```
    Cwd(String),

    /// Convert double word to quad word
    ///
    /// Example:
    /// ```asm
    /// cdq r0
    /// ```
    Cdq(String),

    /// Convert quad word to oct word
    ///
    /// Example:
    /// ```asm
    /// cqo r0
    /// ```
    Cqo(String),

    /// Convert word to double word (extended)
    ///
    /// Example:
    /// ```asm
    /// cwde r0
    /// ```
    Cwde(String),

    /// Convert double word to quad word (extended)
    ///
    /// Example:
    /// ```asm
    /// cdqe r0
    /// ```
    Cdqe(String),

    //
    // Control Flow
    //
    /// Unconditional jump
    ///
    /// Example:
    /// ```asm
    /// jmp label
    /// jmp main
    /// ```
    Jmp(String),

    /// Jump if equal/zero
    ///
    /// Example:
    /// ```asm
    /// je label
    /// ```
    Je(String),

    /// Jump if not equal/not zero
    ///
    /// Example:
    /// ```asm
    /// jne label
    /// ```
    Jne(String),

    /// Jump if less
    ///
    /// Example:
    /// ```asm
    /// jl label
    /// ```
    Jl(String),

    /// Jump if less or equal
    ///
    /// Example:
    /// ```asm
    /// jle label
    /// ```
    Jle(String),

    /// Jump if greater
    ///
    /// Example:
    /// ```asm
    /// jg label
    /// ```
    Jg(String),

    /// Jump if greater or equal
    ///
    /// Example:
    /// ```asm
    /// jge label
    /// ```
    Jge(String),

    /// Jump if overflow
    ///
    /// Example:
    /// ```asm
    /// jo label
    /// ```
    Jo(String),

    /// Jump if not overflow
    ///
    /// Example:
    /// ```asm
    /// jno label
    /// ```
    Jno(String),

    /// Jump if sign
    ///
    /// Example:
    /// ```asm
    /// js label
    /// ```
    Js(String),

    /// Jump if not sign
    ///
    /// Example:
    /// ```asm
    /// jns label
    /// ```
    Jns(String),

    /// Jump if parity
    ///
    /// Example:
    /// ```asm
    /// jp label
    /// ```
    Jp(String),

    /// Jump if not parity
    ///
    /// Example:
    /// ```asm
    /// jnp label
    /// ```
    Jnp(String),

    /// Jump if above (unsigned)
    ///
    /// Example:
    /// ```asm
    /// ja label
    /// ```
    Ja(String),

    /// Jump if above or equal
    ///
    /// Example:
    /// ```asm
    /// jae label
    /// ```
    Jae(String),

    /// Jump if below (unsigned)
    ///
    /// Example:
    /// ```asm
    /// jb label
    /// ```
    Jb(String),

    /// Jump if below or equal
    ///
    /// Example:
    /// ```asm
    /// jbe label
    /// ```
    Jbe(String),

    /// Loop with condition equal
    ///
    /// Example:
    /// ```asm
    /// loopeq label
    /// ```
    LoopEq(String),

    /// Loop with condition not equal
    ///
    /// Example:
    /// ```asm
    /// loopne label
    /// ```
    LoopNe(String),

    /// Call function
    ///
    /// Example:
    /// ```asm
    /// call function_name
    /// call printf
    /// ```
    Call(String),

    /// Return from function
    ///
    /// Example:
    /// ```asm
    /// ret
    /// ```
    Ret,

    //
    // I/O Operations
    //
    /// Input from port
    ///
    /// Example:
    /// ```asm
    /// in r0, 0x3F8
    /// ```
    In((String, String)),

    /// Output to port
    ///
    /// Example:
    /// ```asm
    /// out 0x3F8, r0
    /// ```
    Out((String, String)),

    /// Input string from port
    ///
    /// Example:
    /// ```asm
    /// ins [r0], 0x3F8
    /// ```
    Ins((String, String)),

    /// Output string to port
    ///
    /// Example:
    /// ```asm
    /// outs 0x3F8, [r0]
    /// ```
    Outs((String, String)),

    //
    // System & CPU Operations
    //
    /// CPU identification
    ///
    /// Example:
    /// ```asm
    /// cpuid
    /// ```
    Cpuid,

    /// Load fence
    ///
    /// Example:
    /// ```asm
    /// lfence
    /// ```
    Lfence,

    /// Store fence
    ///
    /// Example:
    /// ```asm
    /// sfence
    /// ```
    Sfence,

    /// Memory fence
    ///
    /// Example:
    /// ```asm
    /// mfence
    /// ```
    Mfence,

    /// Prefetch data into cache
    ///
    /// Example:
    /// ```asm
    /// prefetch [r0 + 64]
    /// ```
    Prefetch(String),

    /// Flush cache line
    ///
    /// Example:
    /// ```asm
    /// clflush [r0]
    /// ```
    Clflush(String),

    /// Writeback cache line
    ///
    /// Example:
    /// ```asm
    /// clwb [r0]
    /// ```
    Clwb(String),

    //
    // System Calls
    //
    /// System call
    ///
    /// Example:
    /// ```asm
    /// ; Arguments passed in r0, r1, r2, ...
    /// ; Return value in r0
    /// syscall write       ; Write to file descriptor
    /// syscall read        ; Read from file descriptor
    /// syscall exit        ; Exit program
    /// syscall open        ; Open file
    /// syscall close       ; Close file descriptor
    /// ```
    Syscall(String),

    //
    // Directives
    //
    /// Export symbol globally
    ///
    /// Example:
    /// ```asm
    /// global _start
    /// global main
    /// ```
    Global(String),

    /// Import symbol from another module
    ///
    /// Example:
    /// ```asm
    /// extern printf
    /// extern malloc
    /// ```
    Extern(String),

    /// Align next data to n-byte boundary
    ///
    /// Example:
    /// ```asm
    /// align 16
    /// align 8
    /// ```
    Align(String),

    //
    // Data definition
    //
    /// Define byte data
    ///
    /// Example:
    /// ```asm
    /// msg db "Hello, world!", 0xA, 0
    /// bytes db 0x41, 0x42, 0x43
    /// ```
    DataByte(String, Vec<String>),

    /// Define word data (16-bit)
    ///
    /// Example:
    /// ```asm
    /// numbers dw 1, 2, 3, 4
    /// values dw 0x1234, 0x5678
    /// ```
    DataWord(String, Vec<String>),

    /// Define double word data (32-bit)
    ///
    /// Example:
    /// ```asm
    /// matrix dd 1.0, 2.0, 3.0, 4.0
    /// integers dd 42, 84, 126
    /// ```
    DataDword(String, Vec<String>),

    /// Define quad word data (64-bit)
    ///
    /// Example:
    /// ```asm
    /// big_num dq 0x123456789ABCDEF0
    /// pointers dq addr1, addr2
    /// ```
    DataQword(String, Vec<String>),

    //
    // Memory reservation
    //
    /// Reserve bytes
    ///
    /// Example:
    /// ```asm
    /// buffer resb 256
    /// input_buf resb 1024
    /// ```
    ReserveByte(String, String),

    /// Reserve words (16-bit)
    ///
    /// Example:
    /// ```asm
    /// array resw 100
    /// temp_data resw 50
    /// ```
    ReserveWord(String, String),

    /// Reserve double words (32-bit)
    ///
    /// Example:
    /// ```asm
    /// stack resd 50
    /// float_array resd 25
    /// ```
    ReserveDword(String, String),

    /// Reserve quad words (64-bit)
    ///
    /// Example:
    /// ```asm
    /// heap resq 25
    /// long_array resq 100
    /// ```
    ReserveQword(String, String),

    /// Define named constant
    ///
    /// Example:
    /// ```asm
    /// BUFFER_SIZE equ 1024
    /// MAX_RETRY equ 3
    /// msg_len equ 14
    /// ```
    Equ(String, String),

    /// Section directive
    ///
    /// Example:
    /// ```asm
    /// section .text
    /// section .data
    /// section .bss
    /// section .rodata
    /// ```
    Section(Section),
}