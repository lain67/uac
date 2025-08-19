use super::*;
use std::collections::HashMap;

pub struct PowerPC64CodeGen {
    register_map: HashMap<String, String>,
}

impl PowerPC64CodeGen {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();

        // Function argument registers (PowerPC64 ABI)
        register_map.insert("r0".to_string(), "r3".to_string()); // 1st arg/return value
        register_map.insert("r1".to_string(), "r4".to_string()); // 2nd arg
        register_map.insert("r2".to_string(), "r5".to_string()); // 3rd arg
        register_map.insert("r3".to_string(), "r6".to_string()); // 4th arg
        register_map.insert("r4".to_string(), "r7".to_string()); // 5th arg
        register_map.insert("r5".to_string(), "r8".to_string()); // 6th arg
        register_map.insert("r6".to_string(), "r9".to_string()); // 7th arg
        register_map.insert("r7".to_string(), "r10".to_string()); // 8th arg

        // Volatile registers (caller-saved)
        register_map.insert("r8".to_string(), "r11".to_string()); // Volatile
        register_map.insert("r9".to_string(), "r12".to_string()); // Volatile
        register_map.insert("r10".to_string(), "r0".to_string()); // Special volatile (often used for syscalls)
        register_map.insert("r11".to_string(), "r31".to_string()); // Non-volatile (callee-saved)
        register_map.insert("r12".to_string(), "r30".to_string()); // Non-volatile
        register_map.insert("r13".to_string(), "r29".to_string()); // Non-volatile
        register_map.insert("r14".to_string(), "r28".to_string()); // Non-volatile
        register_map.insert("r15".to_string(), "r27".to_string()); // Non-volatile

        // Non-volatile registers
        register_map.insert("r19".to_string(), "r14".to_string());
        register_map.insert("r20".to_string(), "r15".to_string());
        register_map.insert("r21".to_string(), "r16".to_string());
        register_map.insert("r22".to_string(), "r17".to_string());

        // Special purpose registers
        register_map.insert("sp".to_string(), "r1".to_string()); // Stack pointer
        register_map.insert("sb".to_string(), "r31".to_string()); // Frame pointer (if used)
        register_map.insert("ip".to_string(), "lr".to_string()); // Link register

        PowerPC64CodeGen { register_map }
    }

    fn emit_load_imm(&self, rd: &str, imm: i64) -> String {
        if imm >= -32768 && imm <= 32767 {
            return format!("    addi {rd}, r0, {imm}\n");
        }
        let upper = ((imm as i64 >> 16) & 0xFFFF) as i64;
        let lower = (imm as i64 & 0xFFFF) as i64;
        format!("    addis {rd}, r0, {upper}\n    ori {rd}, {rd}, {lower}\n")
    }

    fn emit_load_addr_sym(&self, rd: &str, sym: &str) -> String {
        format!("    addis {rd}, r0, {sym}@ha\n    addi {rd}, {rd}, {sym}@l\n")
    }

    fn emit_reg_move(&self, rd: &str, rs: &str) -> String {
        format!("    or {rd}, {rs}, {rs}\n")
    }
}
impl ArchCodeGen for PowerPC64CodeGen {
    fn get_register_map(&self) -> HashMap<String, String> {
        self.register_map.clone()
    }

    fn get_syntax_header(&self) -> String {
        ".text\n.align 2\n\n".to_string()
    }

    fn generate_mov(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);

        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let v: i64 = s.parse().unwrap_or(0);
            return self.emit_load_imm(&rd, v);
        }

        if s.starts_with('r') || s == "lr" || s == "cr" {
            return self.emit_reg_move(&rd, &s);
        }

        self.emit_load_addr_sym(&rd, &s)
    }

    fn generate_lea(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let src_clean = if src.starts_with('[') && src.ends_with(']') {
            &src[1..src.len() - 1]
        } else {
            src
        };

        if !(src_clean.starts_with('r') || src_clean == "lr" || src_clean == "cr")
            && !src_clean.contains('+')
            && !src_clean.contains('-')
        {
            return self.emit_load_addr_sym(&rd, src_clean);
        }

        if let Some((base, off)) = src_clean.split_once('+') {
            let base = base.trim();
            let off = off.trim();
            let base_reg = self.map_operand(base);
            if off.chars().all(|c| c.is_ascii_digit() || c == '-') {
                return format!(
                    "    or {rd}, {base_reg}, {base_reg}\n    addi {rd}, {rd}, {off}\n"
                );
            }
        }
        let base_reg = self.map_operand(src_clean);
        self.emit_reg_move(&rd, &base_reg)
    }

    fn generate_load(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);

        if !src.starts_with('[') && !src.ends_with(']') {
            return format!(
                "{}    ld {rd}, {sym}@l({rd})\n",
                self.emit_load_addr_sym(&rd, src),
                sym = src
            );
        }

        let inner = &src[1..src.len() - 1].trim();

        if !(inner.starts_with('r') || inner == &"lr" || inner == &"cr")
            && !inner.contains('+')
            && !inner.contains('-')
        {
            return format!(
                "{}    ld {rd}, {sym}@l({rd})\n",
                self.emit_load_addr_sym(&rd, inner),
                sym = inner
            );
        }

        format!("    ld {rd}, {}\n", self.map_memory_operand(src))
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        let rs = self.map_operand(src);
        if rs.chars().all(|c| c.is_ascii_digit()) {
            return format!("    // ERROR: std requires a register, got {}\n", rs);
        }

        if dst.starts_with('[') && dst.ends_with(']') {
            let inner = &dst[1..dst.len() - 1].trim();
            if !(inner.starts_with('r') || inner == &"lr" || inner == &"cr")
                && !inner.contains('+')
                && !inner.contains('-')
            {
                return format!(
                    "{}    std {rs}, {sym}@l(r11)\n",
                    self.emit_load_addr_sym("r11", inner),
                    sym = inner
                );
            }
            return format!("    std {rs}, {}\n", self.map_memory_operand(dst));
        }

        format!(
            "{}    std {rs}, {sym}@l(r11)\n",
            self.emit_load_addr_sym("r11", dst),
            sym = dst
        )
    }

    fn generate_add(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);

        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let v: i64 = s.parse().unwrap_or(0);
            if v >= -32768 && v <= 32767 {
                return format!("    addi {rd}, {rd}, {v}\n");
            }
            return format!("{}    add {rd}, {rd}, r11\n", self.emit_load_imm("r11", v));
        }
        format!("    add {rd}, {rd}, {s}\n")
    }

    fn generate_sub(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);

        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let v: i64 = s.parse().unwrap_or(0);
            if v >= -32768 && v <= 32767 {
                return format!("    addi {rd}, {rd}, {}\n", -v);
            }
            return format!("{}    sub {rd}, {rd}, r11\n", self.emit_load_imm("r11", v));
        }
        format!("    sub {rd}, {rd}, {s}\n")
    }

    fn generate_mul(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let rs = self.map_operand(src);
        format!("    mulld {rd}, {rd}, {rs}\n")
    }

    fn generate_div(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let rs = self.map_operand(src);
        format!("    divd {rd}, {rd}, {rs}\n")
    }

    fn generate_inc(&self, dst: &str) -> String {
        let rd = self.map_operand(dst);
        format!("    addi {rd}, {rd}, 1\n")
    }

    fn generate_dec(&self, dst: &str) -> String {
        let rd = self.map_operand(dst);
        format!("    addi {rd}, {rd}, -1\n")
    }

    fn generate_neg(&self, dst: &str) -> String {
        let rd = self.map_operand(dst);
        format!("    neg {rd}, {rd}\n")
    }

    fn generate_and(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);

        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let v: u32 = s.parse().unwrap_or(0);
            if v <= 65535 {
                return format!("    andi. {rd}, {rd}, {v}\n");
            }
            return format!(
                "{}    and {rd}, {rd}, r11\n",
                self.emit_load_imm("r11", v as i64)
            );
        }
        format!("    and {rd}, {rd}, {s}\n")
    }

    fn generate_or(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);

        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let v: u32 = s.parse().unwrap_or(0);
            if v <= 65535 {
                return format!("    ori {rd}, {rd}, {v}\n");
            }
            return format!(
                "{}    or {rd}, {rd}, r11\n",
                self.emit_load_imm("r11", v as i64)
            );
        }
        format!("    or {rd}, {rd}, {s}\n")
    }

    fn generate_xor(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);

        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let v: u32 = s.parse().unwrap_or(0);
            if v <= 65535 {
                return format!("    xori {rd}, {rd}, {v}\n");
            }
            return format!(
                "{}    xor {rd}, {rd}, r11\n",
                self.emit_load_imm("r11", v as i64)
            );
        }
        format!("    xor {rd}, {rd}, {s}\n")
    }

    fn generate_not(&self, dst: &str) -> String {
        let rd = self.map_operand(dst);
        format!("    nor {rd}, {rd}, r0\n")
    }

    fn generate_shl(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);
        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return format!("    sldi {rd}, {rd}, {s}\n");
        }
        format!("    sld {rd}, {rd}, {s}\n")
    }

    fn generate_shr(&self, dst: &str, src: &str) -> String {
        let rd = self.map_operand(dst);
        let s = self.map_operand(src);
        if s.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return format!("    srdi {rd}, {rd}, {s}\n");
        }
        format!("    srd {rd}, {rd}, {s}\n")
    }

    fn generate_cmp(&self, op1: &str, op2: &str) -> String {
        let r1 = self.map_operand(op1);
        let s2 = self.map_operand(op2);
        if s2.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return format!("    cmpdi {r1}, {s2}\n");
        }
        format!("    cmpd {r1}, {s2}\n")
    }

    fn generate_test(&self, op1: &str, op2: &str) -> String {
        let r1 = self.map_operand(op1);
        let s2 = self.map_operand(op2);
        if s2.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let v: i64 = s2.parse().unwrap_or(0);
            if v >= 0 && v <= 65535 {
                return format!("    andi. r11, {r1}, {v}\n");
            }
            return format!("{}    and. r11, {r1}, r11\n", self.emit_load_imm("r11", v));
        }
        format!("    and. r11, {r1}, {s2}\n")
    }

    fn generate_jmp(&self, label: &str) -> String {
        format!("    b {label}\n")
    }
    fn generate_je(&self, label: &str) -> String {
        format!("    beq {label}\n")
    }
    fn generate_jne(&self, label: &str) -> String {
        format!("    bne {label}\n")
    }
    fn generate_jg(&self, label: &str) -> String {
        format!("    bgt {label}\n")
    }
    fn generate_jl(&self, label: &str) -> String {
        format!("    blt {label}\n")
    }
    fn generate_jge(&self, label: &str) -> String {
        format!("    bge {label}\n")
    }
    fn generate_jle(&self, label: &str) -> String {
        format!("    ble {label}\n")
    }

    fn generate_call(&self, func: &str) -> String {
        format!("    bl {func}\n")
    }
    fn generate_ret(&self) -> String {
        "    blr\n".to_string()
    }

    fn generate_syscall(&self, name: &str) -> String {
        let nr = match name {
            "read" => "3",
            "write" => "4",
            "exit" => "1",
            "open" => "5",
            "close" => "6",
            "mmap" => "90",
            "munmap" => "91",
            "brk" => "45",
            _ => "0",
        };
        format!("    addi r0, r0, {nr}\n    sc\n")
    }

    fn map_operand(&self, operand: &str) -> String {
        if operand.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return operand.to_string();
        }
        if operand.starts_with('[') && operand.ends_with(']') {
            return self.map_memory_operand(operand);
        }
        if let Some(m) = self.register_map.get(operand) {
            m.clone()
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
                    let base = self
                        .register_map
                        .get(parts[0])
                        .cloned()
                        .unwrap_or_else(|| parts[0].to_string());
                    if parts[1].chars().all(|c| c.is_ascii_digit() || c == '-') {
                        return format!("{}({})", parts[1], base);
                    } else {
                        return format!("0({})", base);
                    }
                }
            } else if inner.contains('-') {
                let parts: Vec<&str> = inner.split('-').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let base = self
                        .register_map
                        .get(parts[0])
                        .cloned()
                        .unwrap_or_else(|| parts[0].to_string());
                    if parts[1].chars().all(|c| c.is_ascii_digit()) {
                        return format!("-{}({})", parts[1], base);
                    }
                }
            }

            if let Some(mapped) = self.register_map.get(inner.clone()) {
                return format!("0({})", mapped);
            }
            return format!("0({})", inner);
        }
        operand.to_string()
    }
}
