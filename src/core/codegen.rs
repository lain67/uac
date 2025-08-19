use crate::{
    arch::{ArchCodeGen, create_arch_codegen},
    platform::{PlatformCodeGen, create_platform_codegen},
};

use super::*;

/// Configuration options for code generation and optimization.
#[derive(Debug, Clone)]
pub struct CodeGenConfig {
    /// Enables peephole optimizations: small, local instruction-level transformations
    /// that reduce instruction count or improve performance without changing program behavior.
    pub enable_peephole_optimization: bool,

    /// Enables instruction reordering and sectioning to improve CPU pipeline efficiency
    /// and instruction cache utilization.
    pub enable_instruction_section: bool,

    /// Enables target-specific optimizations: architecture-aware transformations
    /// that leverage special instructions or features of the target CPU.
    pub enable_target_specific_optimizations: bool,

    /// Enables code size minimization optimizations: transforms code to use fewer bytes
    /// while maintaining correctness, useful for embedded systems or memory-constrained targets.
    pub enable_size_minimization: bool,

    /// Enables constant folding: computes constant expressions at compile time
    /// to reduce runtime computation.
    pub enable_constant_folding: bool,

    /// Enables dead code elimination: removes code that is never executed or whose results
    /// are never used, reducing binary size and improving performance.
    pub enable_dead_code_elimination: bool,

    /// Enables common subexpression elimination: detects repeated computations
    /// and reuses the result to avoid redundant instructions.
    pub enable_common_subexpression_elimination: bool,

    /// Enables loop unrolling: expands loops into repeated sequences of instructions
    /// to reduce loop overhead and increase instruction-level parallelism.
    pub enable_loop_unrolling: bool,

    /// Enables branch prediction hints: rearranges branches or inserts hints to improve
    /// CPU branch prediction accuracy.
    pub enable_branch_prediction_optimization: bool,

    /// Enables register allocation optimization: optimizes the use of CPU registers
    /// to reduce memory accesses and improve execution speed.
    pub enable_register_allocation_optimization: bool,

    /// Enables instruction scheduling: rearranges instructions to minimize stalls
    /// due to data dependencies or pipeline hazards.
    pub enable_instruction_scheduling: bool,

    /// Enables inlining of small functions or macros to avoid function call overhead
    /// where beneficial.
    pub enable_function_inlining: bool,

    /// Enables strength reduction: replaces expensive operations with cheaper equivalents
    /// (e.g., replacing multiplication with addition or shifts where possible).
    pub enable_strength_reduction: bool,
}

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
                // Conditional move instructions
                Instruction::CmovEq(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_eq(dst, src));
                }
                Instruction::CmovNe(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_ne(dst, src));
                }
                Instruction::CmovLt(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_lt(dst, src));
                }
                Instruction::CmovLe(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_le(dst, src));
                }
                Instruction::CmovGt(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_gt(dst, src));
                }
                Instruction::CmovGe(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_ge(dst, src));
                }
                Instruction::CmovOv(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_ov(dst, src));
                }
                Instruction::CmovNo(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_no(dst, src));
                }
                Instruction::CmovS(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_s(dst, src));
                }
                Instruction::CmovNs(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_ns(dst, src));
                }
                Instruction::CmovP(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_p(dst, src));
                }
                Instruction::CmovNp(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_np(dst, src));
                }
                Instruction::CmovA(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_a(dst, src));
                }
                Instruction::CmovAe(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_ae(dst, src));
                }
                Instruction::CmovB(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_b(dst, src));
                }
                Instruction::CmovBe(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_cmov_be(dst, src));
                }
                // Stack operations
                Instruction::Push(src) => {
                    output.push_str(&self.arch_codegen.generate_push(src));
                }
                Instruction::Pop(dst) => {
                    output.push_str(&self.arch_codegen.generate_pop(dst));
                }
                Instruction::Pusha => {
                    output.push_str(&self.arch_codegen.generate_pusha());
                }
                Instruction::Popa => {
                    output.push_str(&self.arch_codegen.generate_popa());
                }
                Instruction::Enter(frame_size, nesting) => {
                    output.push_str(&self.arch_codegen.generate_enter(frame_size, nesting));
                }
                Instruction::Leave => {
                    output.push_str(&self.arch_codegen.generate_leave());
                }
                // Additional arithmetic operations
                Instruction::Imul(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_imul(dst, src));
                }
                Instruction::Idiv(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_idiv(dst, src));
                }
                Instruction::Mod(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_mod(dst, src));
                }
                Instruction::Andn(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_andn(dst, src));
                }
                // Shift and rotate operations
                Instruction::Sal(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_sal(dst, src));
                }
                Instruction::Sar(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_sar(dst, src));
                }
                Instruction::Rol(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_rol(dst, src));
                }
                Instruction::Ror(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_ror(dst, src));
                }
                Instruction::Rcl(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_rcl(dst, src));
                }
                Instruction::Rcr(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_rcr(dst, src));
                }
                // Bit manipulation operations
                Instruction::Bextr(dst, src, imm) => {
                    output.push_str(&self.arch_codegen.generate_bextr(dst, src, imm));
                }
                Instruction::Bsf(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_bsf(dst, src));
                }
                Instruction::Bsr(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_bsr(dst, src));
                }
                Instruction::Bt(dst, bit) => {
                    output.push_str(&self.arch_codegen.generate_bt(dst, bit));
                }
                Instruction::Btr(dst, bit) => {
                    output.push_str(&self.arch_codegen.generate_btr(dst, bit));
                }
                Instruction::Bts(dst, bit) => {
                    output.push_str(&self.arch_codegen.generate_bts(dst, bit));
                }
                Instruction::Btc(dst, bit) => {
                    output.push_str(&self.arch_codegen.generate_btc(dst, bit));
                }
                // Set condition code operations
                Instruction::SetEq(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_eq(dst));
                }
                Instruction::SetNe(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_ne(dst));
                }
                Instruction::SetLt(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_lt(dst));
                }
                Instruction::SetLe(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_le(dst));
                }
                Instruction::SetGt(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_gt(dst));
                }
                Instruction::SetGe(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_ge(dst));
                }
                Instruction::SetOv(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_ov(dst));
                }
                Instruction::SetNo(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_no(dst));
                }
                Instruction::SetS(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_s(dst));
                }
                Instruction::SetNs(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_ns(dst));
                }
                Instruction::SetP(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_p(dst));
                }
                Instruction::SetNp(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_np(dst));
                }
                Instruction::SetA(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_a(dst));
                }
                Instruction::SetAe(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_ae(dst));
                }
                Instruction::SetB(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_b(dst));
                }
                Instruction::SetBe(dst) => {
                    output.push_str(&self.arch_codegen.generate_set_be(dst));
                }
                // String operations
                Instruction::Cmps(src1, src2) => {
                    output.push_str(&self.arch_codegen.generate_cmps(src1, src2));
                }
                Instruction::Scas(src, val) => {
                    output.push_str(&self.arch_codegen.generate_scas(src, val));
                }
                Instruction::Stos(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_stos(dst, src));
                }
                Instruction::Lods(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_lods(dst, src));
                }
                Instruction::Movs(dst, src) => {
                    output.push_str(&self.arch_codegen.generate_movs(dst, src));
                }
                // Data conversion operations
                Instruction::Cbw(dst) => {
                    output.push_str(&self.arch_codegen.generate_cbw(dst));
                }
                Instruction::Cwd(dst) => {
                    output.push_str(&self.arch_codegen.generate_cwd(dst));
                }
                Instruction::Cdq(dst) => {
                    output.push_str(&self.arch_codegen.generate_cdq(dst));
                }
                Instruction::Cqo(dst) => {
                    output.push_str(&self.arch_codegen.generate_cqo(dst));
                }
                Instruction::Cwde(dst) => {
                    output.push_str(&self.arch_codegen.generate_cwde(dst));
                }
                Instruction::Cdqe(dst) => {
                    output.push_str(&self.arch_codegen.generate_cdqe(dst));
                }
                // Additional jump instructions
                Instruction::Jo(label) => {
                    output.push_str(&self.arch_codegen.generate_jo(label));
                }
                Instruction::Jno(label) => {
                    output.push_str(&self.arch_codegen.generate_jno(label));
                }
                Instruction::Js(label) => {
                    output.push_str(&self.arch_codegen.generate_js(label));
                }
                Instruction::Jns(label) => {
                    output.push_str(&self.arch_codegen.generate_jns(label));
                }
                Instruction::Jp(label) => {
                    output.push_str(&self.arch_codegen.generate_jp(label));
                }
                Instruction::Jnp(label) => {
                    output.push_str(&self.arch_codegen.generate_jnp(label));
                }
                Instruction::Ja(label) => {
                    output.push_str(&self.arch_codegen.generate_ja(label));
                }
                Instruction::Jae(label) => {
                    output.push_str(&self.arch_codegen.generate_jae(label));
                }
                Instruction::Jb(label) => {
                    output.push_str(&self.arch_codegen.generate_jb(label));
                }
                Instruction::Jbe(label) => {
                    output.push_str(&self.arch_codegen.generate_jbe(label));
                }
                Instruction::LoopEq(label) => {
                    output.push_str(&self.arch_codegen.generate_loop_eq(label));
                }
                Instruction::LoopNe(label) => {
                    output.push_str(&self.arch_codegen.generate_loop_ne(label));
                }
                // I/O operations
                Instruction::In(dst, port) => {
                    output.push_str(&self.arch_codegen.generate_in(dst, port));
                }
                Instruction::Out(port, src) => {
                    output.push_str(&self.arch_codegen.generate_out(port, src));
                }
                Instruction::Ins(dst, port) => {
                    output.push_str(&self.arch_codegen.generate_ins(dst, port));
                }
                Instruction::Outs(port, src) => {
                    output.push_str(&self.arch_codegen.generate_outs(port, src));
                }
                // System and memory operations
                Instruction::Cpuid => {
                    output.push_str(&self.arch_codegen.generate_cpuid());
                }
                Instruction::Lfence => {
                    output.push_str(&self.arch_codegen.generate_lfence());
                }
                Instruction::Sfence => {
                    output.push_str(&self.arch_codegen.generate_sfence());
                }
                Instruction::Mfence => {
                    output.push_str(&self.arch_codegen.generate_mfence());
                }
                Instruction::Prefetch(addr) => {
                    output.push_str(&self.arch_codegen.generate_prefetch(addr));
                }
                Instruction::Clflush(addr) => {
                    output.push_str(&self.arch_codegen.generate_clflush(addr));
                }
                Instruction::Clwb(addr) => {
                    output.push_str(&self.arch_codegen.generate_clwb(addr));
                }
                // Directive operations
                Instruction::Align(n) => {
                    output.push_str(&self.arch_codegen.generate_align(n));
                }
                Instruction::ReserveWord(name, size) => {
                    output.push_str(&self.arch_codegen.generate_reserve_word(name, size));
                }
                Instruction::ReserveDword(name, size) => {
                    output.push_str(&self.arch_codegen.generate_reserve_dword(name, size));
                }
                Instruction::ReserveQword(name, size) => {
                    output.push_str(&self.arch_codegen.generate_reserve_qword(name, size));
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
