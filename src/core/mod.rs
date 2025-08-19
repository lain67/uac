use crate::{arch::Architecture, platform::{Format, Platform}};
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
    Label(String),
    Mov(String, String),
    Lea(String, String),
    Load(String, String),
    Store(String, String),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Inc(String),
    Dec(String),
    Neg(String),
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Not(String),
    Shl(String, String),
    Shr(String, String),
    Cmp(String, String),
    Test(String, String),
    Jmp(String),
    Je(String),
    Jne(String),
    Jg(String),
    Jl(String),
    Jge(String),
    Jle(String),
    Call(String),
    Ret,
    Syscall(String),
    Global(String),
    Extern(String),
    DataByte(String, Vec<String>),
    DataWord(String, Vec<String>),
    DataDword(String, Vec<String>),
    DataQword(String, Vec<String>),
    ReserveByte(String, String),
    Equ(String, String),
    Section(Section),
}
