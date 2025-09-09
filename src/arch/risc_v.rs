use super::*;
use std::collections::HashMap;

pub struct RISCVCodeGen {
    register_map: HashMap<String, String>,
}

impl RISCVCodeGen {
    pub fn new() -> Self {
        let mut register_map = HashMap::with_capacity(32);

        // Function argument registers (RISC-V ABI)
        register_map.insert("r0".to_string(), "a0".to_string()); // 1st arg/return value
        register_map.insert("r1".to_string(), "a1".to_string()); // 2nd arg/return value
        register_map.insert("r2".to_string(), "a2".to_string()); // 3rd arg
        register_map.insert("r3".to_string(), "a3".to_string()); // 4th arg
        register_map.insert("r4".to_string(), "a4".to_string()); // 5th arg
        register_map.insert("r5".to_string(), "a5".to_string()); // 6th arg
        register_map.insert("r6".to_string(), "a6".to_string()); // 7th arg
        register_map.insert("r7".to_string(), "a7".to_string()); // 8th arg/syscall number

        // Temporary registers
        register_map.insert("r8".to_string(), "t0".to_string()); // Temporary
        register_map.insert("r9".to_string(), "t1".to_string()); // Temporary
        register_map.insert("r10".to_string(), "t2".to_string()); // Temporary
        register_map.insert("r11".to_string(), "t3".to_string()); // Temporary
        register_map.insert("r12".to_string(), "t4".to_string()); // Temporary
        register_map.insert("r13".to_string(), "t5".to_string()); // Temporary
        register_map.insert("r14".to_string(), "t6".to_string()); // Temporary

        // Saved registers
        register_map.insert("r19".to_string(), "s0".to_string()); // Saved/frame pointer
        register_map.insert("r20".to_string(), "s1".to_string()); // Saved
        register_map.insert("r21".to_string(), "s2".to_string()); // Saved
        register_map.insert("r22".to_string(), "s3".to_string()); // Saved

        // Special purpose registers
        register_map.insert("sp".to_string(), "sp".to_string()); // Stack pointer
        register_map.insert("sb".to_string(), "s0".to_string()); // Frame pointer
        register_map.insert("ip".to_string(), "ra".to_string()); // Return address

        RISCVCodeGen { register_map }
    }
}

impl ArchCodeGen for RISCVCodeGen {
    fn get_register_map(&self) -> HashMap<String, String> {
        self.register_map.clone()
    }

    fn get_syntax_header(&self) -> String {
        ".text\n.align 2\n\n".to_string()
    }

    fn generate_mov(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let value: i64 = src_op.parse().unwrap_or(0);
            if value >= -2048 && value <= 2047 {
                return format!("    addi {}, zero, {}\n", dst_reg, src_op);
            } else {
                let upper = (value + 0x800) >> 12;
                let lower = value & 0xfff;
                if lower == 0 {
                    return format!("    lui {}, {}\n", dst_reg, upper);
                } else {
                    return format!(
                        "    lui {}, {}\n    addi {}, {}, {}\n",
                        dst_reg, upper, dst_reg, dst_reg, lower as i32 as i16
                    );
                }
            }
        }

        if src_op.starts_with('x')
            || src_op == "sp"
            || src_op.starts_with('a')
            || src_op.starts_with('t')
            || src_op.starts_with('s')
            || src_op == "ra"
        {
            return format!("    mv {}, {}\n", dst_reg, src_op);
        }

        let is_label = !src_op
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_');
        if is_label {
            return format!("    la {}, {}\n", dst_reg, src_op);
        } else {
            return format!("    li {}, {}\n", dst_reg, src_op);
        }
    }

    fn generate_lea(&self, dst: &str, src: &str) -> String {
        let src_clean = if src.starts_with('[') && src.ends_with(']') {
            &src[1..src.len() - 1]
        } else {
            src
        };
        format!("    la {}, {}\n", self.map_operand(dst), src_clean)
    }

    fn generate_load(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);

        if !src.starts_with('[') && !src.ends_with(']') {
            return format!(
                "    la {}, {}\n    ld {}, 0({})\n",
                dst_reg, src, dst_reg, dst_reg
            );
        }

        // Handle memory operands
        let inner = &src[1..src.len() - 1].trim();

        // Check if this looks like a label/symbol
        if !inner.starts_with('x')
            && !inner.starts_with('a')
            && !inner.starts_with('t')
            && !inner.starts_with('s')
            && inner != &"sp"
            && inner != &"ra"
            && !inner.contains('+')
            && !inner.contains('-')
        {
            return format!(
                "    la {}, {}\n    ld {}, 0({})\n",
                dst_reg, inner, dst_reg, dst_reg
            );
        }

        format!("    ld {}, {}\n", dst_reg, self.map_memory_operand(src))
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        let src_reg = self.map_operand(src);

        if src_reg.chars().all(|c| c.is_ascii_digit()) {
            return format!("    // ERROR: sd requires a register, got {}\n", src_reg);
        }

        if dst.starts_with('[') && dst.ends_with(']') {
            let inner = &dst[1..dst.len() - 1].trim();

            if !inner.starts_with('x')
                && !inner.starts_with('a')
                && !inner.starts_with('t')
                && !inner.starts_with('s')
                && inner != &"sp"
                && inner != &"ra"
                && !inner.contains('+')
                && !inner.contains('-')
            {
                return format!("    la t6, {}\n    sd {}, 0(t6)\n", inner, src_reg);
            }

            return format!("    sd {}, {}\n", src_reg, self.map_memory_operand(dst));
        }

        format!("    la t6, {}\n    sd {}, 0(t6)\n", dst, src_reg)
    }

    fn generate_add(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let value: i32 = src_op.parse().unwrap_or(0);
            if value >= -2048 && value <= 2047 {
                format!("    addi {}, {}, {}\n", dst_reg, dst_reg, src_op)
            } else {
                format!(
                    "    li t6, {}\n    add {}, {}, t6\n",
                    src_op, dst_reg, dst_reg
                )
            }
        } else {
            format!("    add {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_sub(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let value: i32 = src_op.parse().unwrap_or(0);
            if value >= -2047 && value <= 2048 {
                format!("    addi {}, {}, {}\n", dst_reg, dst_reg, -value)
            } else {
                format!(
                    "    li t6, {}\n    sub {}, {}, t6\n",
                    src_op, dst_reg, dst_reg
                )
            }
        } else if src_op.starts_with('a')
            || src_op.starts_with('t')
            || src_op.starts_with('s')
            || src_op == "sp"
            || src_op == "ra"
        {
            format!("    sub {}, {}, {}\n", dst_reg, dst_reg, src_op)
        } else {
            format!(
                "    li t6, {}\n    sub {}, {}, t6\n",
                src_op, dst_reg, dst_reg
            )
        }
    }

    fn generate_mul(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            // RISC-V mul doesn't support immediates, load into temp register
            format!(
                "    li t6, {}\n    mul {}, {}, t6\n",
                src_op, dst_reg, dst_reg
            )
        } else if src_op.starts_with('a')
            || src_op.starts_with('t')
            || src_op.starts_with('s')
            || src_op == "sp"
            || src_op == "ra"
        {
            format!("    mul {}, {}, {}\n", dst_reg, dst_reg, src_op)
        } else {
            // Handle symbolic constants by loading them first
            format!(
                "    li t6, {}\n    mul {}, {}, t6\n",
                src_op, dst_reg, dst_reg
            )
        }
    }

    fn generate_div(&self, dst: &str, src: &str) -> String {
        format!(
            "    div {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_inc(&self, dst: &str) -> String {
        format!(
            "    addi {}, {}, 1\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_dec(&self, dst: &str) -> String {
        format!(
            "    addi {}, {}, -1\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_neg(&self, dst: &str) -> String {
        format!(
            "    neg {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_and(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let value: i32 = src_op.parse().unwrap_or(0);
            if value >= -2048 && value <= 2047 {
                format!("    andi {}, {}, {}\n", dst_reg, dst_reg, src_op)
            } else {
                format!(
                    "    li t6, {}\n    and {}, {}, t6\n",
                    src_op, dst_reg, dst_reg
                )
            }
        } else {
            format!("    and {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_or(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let value: i32 = src_op.parse().unwrap_or(0);
            if value >= -2048 && value <= 2047 {
                format!("    ori {}, {}, {}\n", dst_reg, dst_reg, src_op)
            } else {
                format!(
                    "    li t6, {}\n    or {}, {}, t6\n",
                    src_op, dst_reg, dst_reg
                )
            }
        } else {
            format!("    or {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_xor(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let value: i32 = src_op.parse().unwrap_or(0);
            if value >= -2048 && value <= 2047 {
                format!("    xori {}, {}, {}\n", dst_reg, dst_reg, src_op)
            } else {
                format!(
                    "    li t6, {}\n    xor {}, {}, t6\n",
                    src_op, dst_reg, dst_reg
                )
            }
        } else {
            format!("    xor {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_not(&self, dst: &str) -> String {
        format!(
            "    not {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_shl(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    slli {}, {}, {}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    sll {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_shr(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    srli {}, {}, {}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    srl {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_cmp(&self, op1: &str, op2: &str) -> String {
        let op1_reg = self.map_operand(op1);
        let op2_op = self.map_operand(op2);

        if op2_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    li t6, {}\n    sub t6, {}, t6\n", op2_op, op1_reg)
        } else if op2_op.starts_with('a')
            || op2_op.starts_with('t')
            || op2_op.starts_with('s')
            || op2_op == "sp"
            || op2_op == "ra"
        {
            format!("    sub t6, {}, {}\n", op1_reg, op2_op)
        } else {
            format!(
                "    addi t6, zero, %lo({})\n    sub t6, {}, t6\n",
                op2_op, op1_reg
            )
        }
    }

    fn generate_test(&self, op1: &str, op2: &str) -> String {
        let op1_reg = self.map_operand(op1);
        let op2_op = self.map_operand(op2);

        if op2_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    li t6, {}\n    and t6, {}, t6\n", op2_op, op1_reg)
        } else {
            format!("    and t6, {}, {}\n", op1_reg, op2_op)
        }
    }

    fn generate_jmp(&self, label: &str) -> String {
        format!("    j {}\n", label)
    }

    fn generate_je(&self, label: &str) -> String {
        format!("    beqz t6, {}\n", label)
    }

    fn generate_jne(&self, label: &str) -> String {
        format!("    bnez t6, {}\n", label)
    }

    fn generate_jg(&self, label: &str) -> String {
        format!("    bgtz t6, {}\n", label)
    }

    fn generate_jl(&self, label: &str) -> String {
        format!("    bltz t6, {}\n", label)
    }

    fn generate_jge(&self, label: &str) -> String {
        format!("    bgez t6, {}\n", label)
    }

    fn generate_jle(&self, label: &str) -> String {
        format!("    blez t6, {}\n", label)
    }

    fn generate_call(&self, func: &str) -> String {
        format!("    call {}\n", func)
    }

    fn generate_ret(&self) -> String {
        "    ret\n".to_string()
    }

    fn generate_syscall(&self, name: &str) -> String {
        let syscall_num = match name {
            "read" => "63",
            "write" => "64",
            "exit" => "93",
            "openat" => "56",
            "close" => "57",
            "mmap" => "222",
            "munmap" => "215",
            "brk" => "214",
            _ => {
                return format!(
                    "    // Unknown syscall: {}\n    li a7, 0\n    ecall\n",
                    name
                );
            }
        };
        format!("    li a7, {}\n    ecall\n", syscall_num)
    }

    fn map_operand(&self, operand: &str) -> String {
        if operand.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return operand.to_string();
        }

        if operand.starts_with('[') && operand.ends_with(']') {
            return self.map_memory_operand(operand);
        }

        if let Some(mapped) = self.register_map.get(operand) {
            mapped.clone()
        } else {
            operand.to_string()
        }
    }

    fn map_memory_operand(&self, operand: &str) -> String {
        if operand.starts_with('[') && operand.ends_with(']') {
            let inner = &operand[1..operand.len() - 1].trim();

            if inner.contains('+') {
                let parts: Vec<&str> = inner.split('+').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let base = if let Some(mapped) = self.register_map.get(parts[0]) {
                        mapped.clone()
                    } else {
                        parts[0].to_string()
                    };

                    if parts[1].chars().all(|c| c.is_ascii_digit()) {
                        return format!("{}({})", parts[1], base);
                    } else {
                        let offset = if let Some(mapped) = self.register_map.get(parts[1]) {
                            mapped.clone()
                        } else {
                            parts[1].to_string()
                        };
                        return format!("{}({})", offset, base);
                    }
                }
            } else if inner.contains('-') {
                let parts: Vec<&str> = inner.split('-').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let base = if let Some(mapped) = self.register_map.get(parts[0]) {
                        mapped.clone()
                    } else {
                        parts[0].to_string()
                    };

                    if parts[1].chars().all(|c| c.is_ascii_digit()) {
                        return format!("-{}({})", parts[1], base);
                    }
                }
            }

            if let Some(mapped) = self.register_map.get(&inner.to_string()) {
                return format!("0({})", mapped);
            }
            return format!("0({})", inner);
        } else {
            operand.to_string()
        }
    }

    fn generate_cmov_eq(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_ne(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_lt(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_le(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_gt(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_ge(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_ov(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_no(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_s(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_ns(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_p(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_np(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_a(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_ae(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_b(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cmov_be(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_push(&self, src: &str) -> String {
        todo!()
    }

    fn generate_pop(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_pusha(&self) -> String {
        todo!()
    }

    fn generate_popa(&self) -> String {
        todo!()
    }

    fn generate_enter(&self, frame_size: &str, nesting_level: &str) -> String {
        todo!()
    }

    fn generate_leave(&self) -> String {
        todo!()
    }

    fn generate_imul(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_idiv(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_mod(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_andn(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_sal(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_sar(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_rol(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_ror(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_rcl(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_rcr(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_bextr(&self, dst: &str, src: &str, imm: &str) -> String {
        todo!()
    }

    fn generate_bsf(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_bsr(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_bt(&self, dst: &str, bit: &str) -> String {
        todo!()
    }

    fn generate_btr(&self, dst: &str, bit: &str) -> String {
        todo!()
    }

    fn generate_bts(&self, dst: &str, bit: &str) -> String {
        todo!()
    }

    fn generate_btc(&self, dst: &str, bit: &str) -> String {
        todo!()
    }

    fn generate_set_eq(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_ne(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_lt(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_le(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_gt(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_ge(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_ov(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_no(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_s(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_ns(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_p(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_np(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_a(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_ae(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_b(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_set_be(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_cmps(&self, src1: &str, src2: &str) -> String {
        todo!()
    }

    fn generate_scas(&self, src: &str, val: &str) -> String {
        todo!()
    }

    fn generate_stos(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_lods(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_movs(&self, dst: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cbw(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_cwd(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_cdq(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_cqo(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_cwde(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_cdqe(&self, dst: &str) -> String {
        todo!()
    }

    fn generate_jo(&self, label: &str) -> String {
        todo!()
    }

    fn generate_jno(&self, label: &str) -> String {
        todo!()
    }

    fn generate_js(&self, label: &str) -> String {
        todo!()
    }

    fn generate_jns(&self, label: &str) -> String {
        todo!()
    }

    fn generate_jp(&self, label: &str) -> String {
        todo!()
    }

    fn generate_jnp(&self, label: &str) -> String {
        todo!()
    }

    fn generate_ja(&self, label: &str) -> String {
        todo!()
    }

    fn generate_jae(&self, label: &str) -> String {
        todo!()
    }

    fn generate_jb(&self, label: &str) -> String {
        todo!()
    }

    fn generate_jbe(&self, label: &str) -> String {
        todo!()
    }

    fn generate_loop_eq(&self, label: &str) -> String {
        todo!()
    }

    fn generate_loop_ne(&self, label: &str) -> String {
        todo!()
    }

    fn generate_in(&self, dst: &str, port: &str) -> String {
        todo!()
    }

    fn generate_out(&self, port: &str, src: &str) -> String {
        todo!()
    }

    fn generate_ins(&self, dst: &str, port: &str) -> String {
        todo!()
    }

    fn generate_outs(&self, port: &str, src: &str) -> String {
        todo!()
    }

    fn generate_cpuid(&self) -> String {
        todo!()
    }

    fn generate_lfence(&self) -> String {
        todo!()
    }

    fn generate_sfence(&self) -> String {
        todo!()
    }

    fn generate_mfence(&self) -> String {
        todo!()
    }

    fn generate_prefetch(&self, addr: &str) -> String {
        todo!()
    }

    fn generate_clflush(&self, addr: &str) -> String {
        todo!()
    }

    fn generate_clwb(&self, addr: &str) -> String {
        todo!()
    }

    fn generate_global(&self, symbol: &str) -> String {
        todo!()
    }

    fn generate_extern(&self, symbol: &str) -> String {
        todo!()
    }

    fn generate_align(&self, n: &str) -> String {
        todo!()
    }

    fn generate_data_byte(&self, name: &str, values: &[String]) -> String {
        todo!()
    }

    fn generate_data_word(&self, name: &str, values: &[String]) -> String {
        todo!()
    }

    fn generate_data_dword(&self, name: &str, values: &[String]) -> String {
        todo!()
    }

    fn generate_data_qword(&self, name: &str, values: &[String]) -> String {
        todo!()
    }

    fn generate_reserve_byte(&self, name: &str, count: &str) -> String {
        todo!()
    }

    fn generate_reserve_word(&self, name: &str, count: &str) -> String {
        todo!()
    }

    fn generate_reserve_dword(&self, name: &str, count: &str) -> String {
        todo!()
    }

    fn generate_reserve_qword(&self, name: &str, count: &str) -> String {
        todo!()
    }

    fn generate_equ(&self, name: &str, value: &str) -> String {
        todo!()
    }

    fn generate_section(&self, section: &Section) -> String {
        todo!()
    }

    fn generate_label(&self, name: &str) -> String {
        todo!()
    }
}
