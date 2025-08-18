use crate::{
    arch::{ArchCodeGen, create_arch_codegen},
    platform::{PlatformCodeGen, create_platform_codegen},
};

use super::*;

pub struct CodeGenerator {
    arch_codegen: Box<dyn ArchCodeGen>,
    platform_codegen: Box<dyn PlatformCodeGen>,
    target: TargetTriple,
}

impl CodeGenerator {
    pub fn new(target: TargetTriple) -> Self {
        let arch_codegen = create_arch_codegen(&target.architecture);
        let platform_codegen = create_platform_codegen(&target.platform);

        CodeGenerator {
            arch_codegen,
            platform_codegen,
            target,
        }
    }

    pub fn generate(&self, instructions: &[Instruction]) -> String {
        let mut output = String::new();
        output.push_str(&self.arch_codegen.get_syntax_header());

        for instruction in instructions {
            match instruction {
                Instruction::Section(section) => {
                    output.push_str(&self.platform_codegen.get_section_prefix(section));
                }
                Instruction::Label(name) => {
                    output.push_str(&format!("{}:\n", name));
                }
                Instruction::Mov(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_mov(dst, src));
                }
                Instruction::Lea(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_lea(dst, src));
                }
                Instruction::Load(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_load(dst, src));
                }
                Instruction::Store(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_store(dst, src));
                }
                Instruction::Add(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_add(dst, src));
                }
                Instruction::Sub(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_sub(dst, src));
                }
                Instruction::Mul(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_mul(dst, src));
                }
                Instruction::Div(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_div(dst, src));
                }
                Instruction::Inc(dst) => {
                    output.push_str(&self.arch_codegen.generate_inc(dst));
                }
                Instruction::Dec(dst) => {
                    output.push_str(&self.arch_codegen.generate_dec(dst));
                }
                Instruction::Neg(dst) => {
                    output.push_str(&self.arch_codegen.generate_neg(dst));
                }
                Instruction::And(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_and(dst, src));
                }
                Instruction::Or(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_or(dst, src));
                }
                Instruction::Xor(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_xor(dst, src));
                }
                Instruction::Not(dst) => {
                    output.push_str(&self.arch_codegen.generate_not(dst));
                }
                Instruction::Shl(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_shl(dst, src));
                }
                Instruction::Shr(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_shr(dst, src));
                }
                Instruction::Cmp(op1, op2) => {
                    output.push_str(&self.arch_codegen.generate_cmp(op1, op2));
                }
                Instruction::Test(op1, op2) => {
                    output.push_str(&self.arch_codegen.generate_test(op1, op2));
                }
                Instruction::Jmp(label) => {
                    output.push_str(&self.arch_codegen.generate_jmp(label));
                }
                Instruction::Je(label) => {
                    output.push_str(&self.arch_codegen.generate_je(label));
                }
                Instruction::Jne(label) => {
                    output.push_str(&self.arch_codegen.generate_jne(label));
                }
                Instruction::Jg(label) => {
                    output.push_str(&self.arch_codegen.generate_jg(label));
                }
                Instruction::Jl(label) => {
                    output.push_str(&self.arch_codegen.generate_jl(label));
                }
                Instruction::Jge(label) => {
                    output.push_str(&self.arch_codegen.generate_jge(label));
                }
                Instruction::Jle(label) => {
                    output.push_str(&self.arch_codegen.generate_jle(label));
                }
                Instruction::Call(func) => {
                    output.push_str(&self.arch_codegen.generate_call(func));
                }
                Instruction::Ret => {
                    output.push_str(&self.arch_codegen.generate_ret());
                }
                Instruction::Syscall(name) => {
                    output.push_str(&self.arch_codegen.generate_syscall(name));
                }
                Instruction::Global(symbol) => {
                    output.push_str(&self.platform_codegen.get_global_directive(symbol));
                }
                Instruction::Extern(symbol) => {
                    output.push_str(&self.platform_codegen.get_extern_directive(symbol));
                }
                Instruction::DataByte(name, values) => {
                    let processed_values = self.process_data_values(values);
                    output.push_str(&self.platform_codegen.format_data_directive(
                        DataSize::Byte,
                        name,
                        &processed_values,
                    ));
                }
                Instruction::DataWord(name, values) => {
                    let processed_values = self.process_data_values(values);
                    output.push_str(&self.platform_codegen.format_data_directive(
                        DataSize::Word,
                        name,
                        &processed_values,
                    ));
                }
                Instruction::DataDword(name, values) => {
                    let processed_values = self.process_data_values(values);
                    output.push_str(&self.platform_codegen.format_data_directive(
                        DataSize::Dword,
                        name,
                        &processed_values,
                    ));
                }
                Instruction::DataQword(name, values) => {
                    let processed_values = self.process_data_values(values);
                    output.push_str(&self.platform_codegen.format_data_directive(
                        DataSize::Qword,
                        name,
                        &processed_values,
                    ));
                }
                Instruction::ReserveByte(name, size) => {
                    output.push_str(&self.platform_codegen.format_reserve_directive(name, size));
                }
                Instruction::Equ(name, value) => {
                    output.push_str(&self.platform_codegen.format_equ_directive(name, value));
                }
            }
        }

        output
    }

    fn process_data_values(&self, values: &[String]) -> Vec<String> {
        let mut processed = Vec::new();
        for value in values {
            processed.extend(self.format_data_value(value));
        }
        processed
    }

    fn format_data_value(&self, value: &str) -> Vec<String> {
        let trimmed = value.trim();
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            let string_content = &trimmed[1..trimmed.len() - 1];
            let mut result = Vec::new();
            let mut chars = string_content.chars();

            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(next_char) = chars.next() {
                        match next_char {
                            'n' => result.push("10".to_string()),
                            't' => result.push("9".to_string()),
                            'r' => result.push("13".to_string()),
                            '\\' => result.push("92".to_string()),
                            '"' => result.push("34".to_string()),
                            _ => {
                                result.push((c as u8).to_string());
                                result.push((next_char as u8).to_string());
                            }
                        }
                    } else {
                        result.push((c as u8).to_string());
                    }
                } else {
                    result.push((c as u8).to_string());
                }
            }
            result
        } else {
            vec![trimmed.to_string()]
        }
    }

    pub fn get_target(&self) -> &TargetTriple {
        &self.target
    }
}
