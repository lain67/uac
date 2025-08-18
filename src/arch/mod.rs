use std::{collections::HashMap, process};

use crate::arch::{amd64::AMD64CodeGen, arm64::ARM64CodeGen};

pub mod amd64;
pub mod arm64;

#[derive(Debug, Clone)]
pub enum Architecture {
    AMD64,
    ARM64,
    RISCV,
    PowerPC64,
    IA32,
    ARM32,
    MIPS32,
    SPARC64,
    IA64,
    Alpha,
    HPPA,
    K68,
    AVR,
    MSP430,
    SH,
    VAX,
    NIOSII,
    Xtensa,
    ARC,
    Z80,
}

pub trait ArchCodeGen {
    fn get_register_map(&self) -> HashMap<String, String>;
    fn get_syntax_header(&self) -> String;
    fn generate_mov(&self, dst: &str, src: &str) -> String;
    fn generate_lea(&self, dst: &str, src: &str) -> String;
    fn generate_load(&self, dst: &str, src: &str) -> String;
    fn generate_store(&self, dst: &str, src: &str) -> String;
    fn generate_add(&self, dst: &str, src: &str) -> String;
    fn generate_sub(&self, dst: &str, src: &str) -> String;
    fn generate_mul(&self, dst: &str, src: &str) -> String;
    fn generate_div(&self, dst: &str, src: &str) -> String;
    fn generate_inc(&self, dst: &str) -> String;
    fn generate_dec(&self, dst: &str) -> String;
    fn generate_neg(&self, dst: &str) -> String;
    fn generate_and(&self, dst: &str, src: &str) -> String;
    fn generate_or(&self, dst: &str, src: &str) -> String;
    fn generate_xor(&self, dst: &str, src: &str) -> String;
    fn generate_not(&self, dst: &str) -> String;
    fn generate_shl(&self, dst: &str, src: &str) -> String;
    fn generate_shr(&self, dst: &str, src: &str) -> String;
    fn generate_cmp(&self, op1: &str, op2: &str) -> String;
    fn generate_test(&self, op1: &str, op2: &str) -> String;
    fn generate_jmp(&self, label: &str) -> String;
    fn generate_je(&self, label: &str) -> String;
    fn generate_jne(&self, label: &str) -> String;
    fn generate_jg(&self, label: &str) -> String;
    fn generate_jl(&self, label: &str) -> String;
    fn generate_jge(&self, label: &str) -> String;
    fn generate_jle(&self, label: &str) -> String;
    fn generate_call(&self, func: &str) -> String;
    fn generate_ret(&self) -> String;
    fn generate_syscall(&self, name: &str) -> String;
    fn map_operand(&self, operand: &str) -> String;
    fn map_memory_operand(&self, operand: &str) -> String;
}

pub fn create_arch_codegen(architecture: &Architecture) -> Box<dyn ArchCodeGen> {
    match architecture {
        Architecture::AMD64 => Box::new(AMD64CodeGen::new()),
        Architecture::ARM64 => Box::new(ARM64CodeGen::new()),
        _ => {
            eprintln!(
                "Error: Architecture {:?} is not currently implemented",
                architecture
            );
            process::exit(1);
        }
    }
}
