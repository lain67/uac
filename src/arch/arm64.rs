use super::*;
use std::collections::HashMap;

pub struct ARM64CodeGen {
    register_map: HashMap<String, String>,
}

impl ARM64CodeGen {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();

        // Function argument registers (AAPCS64)
        register_map.insert("r0".to_string(), "x0".to_string()); // 1st arg
        register_map.insert("r1".to_string(), "x1".to_string()); // 2nd arg
        register_map.insert("r2".to_string(), "x2".to_string()); // 3rd arg
        register_map.insert("r3".to_string(), "x3".to_string()); // 4th arg
        register_map.insert("r4".to_string(), "x4".to_string()); // 5th arg
        register_map.insert("r5".to_string(), "x5".to_string()); // 6th arg
        register_map.insert("r6".to_string(), "x6".to_string()); // 7th arg
        register_map.insert("r7".to_string(), "x7".to_string()); // 8th arg

        // General-purpose registers (avoiding procedure call standard conflicts)
        register_map.insert("r8".to_string(), "x8".to_string()); // Indirect result location
        register_map.insert("r9".to_string(), "x9".to_string()); // Temporary
        register_map.insert("r10".to_string(), "x10".to_string()); // Temporary
        register_map.insert("r11".to_string(), "x11".to_string()); // Temporary
        register_map.insert("r12".to_string(), "x12".to_string()); // Temporary
        register_map.insert("r13".to_string(), "x13".to_string()); // Temporary
        register_map.insert("r14".to_string(), "x14".to_string()); // Temporary
        register_map.insert("r15".to_string(), "x15".to_string()); // Temporary

        // Callee-saved registers
        register_map.insert("r19".to_string(), "x19".to_string());
        register_map.insert("r20".to_string(), "x20".to_string());
        register_map.insert("r21".to_string(), "x21".to_string());
        register_map.insert("r22".to_string(), "x22".to_string());

        // Special purpose registers
        register_map.insert("sp".to_string(), "sp".to_string());
        register_map.insert("sb".to_string(), "x29".to_string()); // frame pointer (FP)
        register_map.insert("ip".to_string(), "x30".to_string()); // link register (LR)

        ARM64CodeGen { register_map }
    }
}

impl ArchCodeGen for ARM64CodeGen {
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
            if value >= 0 && value <= 65535 {
                return format!("    mov {}, #{}\n", dst_reg, src_op);
            } else {
                let low = value & 0xFFFF;
                let high = (value >> 16) & 0xFFFF;
                if high == 0 {
                    return format!("    mov {}, #{}\n", dst_reg, low);
                } else {
                    return format!(
                        "    movz {}, #{}\n    movk {}, #{}, lsl #16\n",
                        dst_reg, low, dst_reg, high
                    );
                }
            }
        }

        if src_op.starts_with('x') || src_op.starts_with('w') || src_op == "sp" {
            return format!("    mov {}, {}\n", dst_reg, src_op);
        }

        // format!("    mov {}, #{}\n", dst_reg, src_op)
        format!("    ldr {}, ={}\n", dst_reg, src_op)
    }

    fn generate_lea(&self, dst: &str, src: &str) -> String {
        let src_clean = if src.starts_with('[') && src.ends_with(']') {
            &src[1..src.len() - 1]
        } else {
            src
        };
        format!("    adr {}, {}\n", self.map_operand(dst), src_clean)
    }

    fn generate_load(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);

        if !src.starts_with('[') && !src.ends_with(']') {
            return format!("    ldr {}, ={}\n", dst_reg, src);
        }

        format!("    ldr {}, {}\n", dst_reg, self.map_memory_operand(src))
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        let dst_mem = self.map_memory_operand(dst);
        let src_reg = self.map_operand(src);

        if src_reg.chars().all(|c| c.is_ascii_digit()) {
            return format!("    // ERROR: str requires a register, got {}\n", src_reg);
        }

        format!("    str {}, {}\n", src_reg, dst_mem)
    }

    fn generate_add(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    add {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    add {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_sub(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    sub {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    sub {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_mul(&self, dst: &str, src: &str) -> String {
        format!(
            "    mul {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_div(&self, dst: &str, src: &str) -> String {
        format!(
            "    sdiv {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_inc(&self, dst: &str) -> String {
        format!(
            "    add {}, {}, #1\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_dec(&self, dst: &str) -> String {
        format!(
            "    sub {}, {}, #1\n",
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
            format!("    and {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    and {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_or(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    orr {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    orr {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_xor(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    eor {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    eor {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_not(&self, dst: &str) -> String {
        format!(
            "    mvn {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_shl(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    lsl {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    lsl {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_shr(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    lsr {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    lsr {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_cmp(&self, op1: &str, op2: &str) -> String {
        let op1_reg = self.map_operand(op1);
        let op2_op = self.map_operand(op2);

        if op2_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    cmp {}, #{}\n", op1_reg, op2_op)
        } else {
            format!("    cmp {}, {}\n", op1_reg, op2_op)
        }
    }

    fn generate_test(&self, op1: &str, op2: &str) -> String {
        let op1_reg = self.map_operand(op1);
        let op2_op = self.map_operand(op2);

        if op2_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    tst {}, #{}\n", op1_reg, op2_op)
        } else {
            format!("    tst {}, {}\n", op1_reg, op2_op)
        }
    }

    fn generate_jmp(&self, label: &str) -> String {
        format!("    b {}\n", label)
    }

    fn generate_je(&self, label: &str) -> String {
        format!("    b.eq {}\n", label)
    }

    fn generate_jne(&self, label: &str) -> String {
        format!("    b.ne {}\n", label)
    }

    fn generate_jg(&self, label: &str) -> String {
        format!("    b.gt {}\n", label)
    }

    fn generate_jl(&self, label: &str) -> String {
        format!("    b.lt {}\n", label)
    }

    fn generate_jge(&self, label: &str) -> String {
        format!("    b.ge {}\n", label)
    }

    fn generate_jle(&self, label: &str) -> String {
        format!("    b.le {}\n", label)
    }

    fn generate_call(&self, func: &str) -> String {
        format!("    bl {}\n", func)
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
                    "    // Unknown syscall: {}\n    mov x8, #0\n    svc #0\n",
                    name
                );
            }
        };
        format!("    mov x8, #{}\n    svc #0\n", syscall_num)
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
                        return format!("[{}, #{}]", base, parts[1]);
                    } else {
                        let offset = if let Some(mapped) = self.register_map.get(parts[1]) {
                            mapped.clone()
                        } else {
                            parts[1].to_string()
                        };
                        return format!("[{}, {}]", base, offset);
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
                        return format!("[{}, #-{}]", base, parts[1]);
                    }
                }
            }

            if let Some(mapped) = self.register_map.get(&inner.to_string()) {
                return format!("[{}]", mapped);
            }
            return format!("[{}]", inner);
        } else {
            operand.to_string()
        }
    }
}
