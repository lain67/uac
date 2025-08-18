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
    pub fn linux_amd64() -> Self {
        TargetTriple {
            architecture: Architecture::AMD64,
            platform: Platform::Linux,
            format: Format::ELF,
        }
    }

    pub fn linux_arm64() -> Self {
        TargetTriple {
            architecture: Architecture::ARM64,
            platform: Platform::Linux,
            format: Format::ELF,
        }
    }

    pub fn linux_riscv64() -> Self {
        TargetTriple {
            architecture: Architecture::RISCV,
            platform: Platform::Linux,
            format: Format::ELF,
        }
    }

    pub fn windows_amd64() -> Self {
        TargetTriple {
            architecture: Architecture::AMD64,
            platform: Platform::Windows,
            format: Format::COFF,
        }
    }

    pub fn windows_arm64() -> Self {
        TargetTriple {
            architecture: Architecture::ARM64,
            platform: Platform::Windows,
            format: Format::COFF,
        }
    }

    pub fn windows_riscv64() -> Self {
        TargetTriple {
            architecture: Architecture::RISCV,
            platform: Platform::Windows,
            format: Format::COFF,
        }
    }

    pub fn macos_amd64() -> Self {
        TargetTriple {
            architecture: Architecture::AMD64,
            platform: Platform::MacOS,
            format: Format::MachO,
        }
    }

    pub fn macos_arm64() -> Self {
        TargetTriple {
            architecture: Architecture::ARM64,
            platform: Platform::MacOS,
            format: Format::MachO,
        }
    }

    pub fn macos_riscv64() -> Self {
        TargetTriple {
            architecture: Architecture::RISCV,
            platform: Platform::MacOS,
            format: Format::MachO,
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
