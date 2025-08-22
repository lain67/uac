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
        ".text\n\n".to_string()
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

        let inner = &src[1..src.len() - 1].trim();

        // If it's a register reference like [r1], map it properly
        if let Some(mapped_reg) = self.register_map.get(&inner.to_string()) {
            return format!("    ldr {}, [{}]\n", dst_reg, mapped_reg);
        }

        // If it contains arithmetic like [r1 + offset]
        if inner.contains('+') || inner.contains('-') {
            return format!("    ldr {}, {}\n", dst_reg, self.map_memory_operand(src));
        }

        // If it's a symbol/label, load from that address
        format!("    adr x16, {}\n    ldr {}, [x16]\n", inner, dst_reg)
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        let src_reg = self.map_operand(src);

        if src_reg.chars().all(|c| c.is_ascii_digit()) {
            return format!("    // ERROR: str requires a register, got {}\n", src_reg);
        }

        // Handle memory operand properly
        if dst.starts_with('[') && dst.ends_with(']') {
            let inner = &dst[1..dst.len() - 1].trim();

            // If it's a register reference like [r1], map it properly
            if let Some(mapped_reg) = self.register_map.get(&inner.to_string()) {
                return format!("    str {}, [{}]\n", src_reg, mapped_reg);
            }

            // If it contains arithmetic like [r1 + offset]
            if inner.contains('+') || inner.contains('-') {
                let dst_mem = self.map_memory_operand(dst);
                return format!("    str {}, {}\n", src_reg, dst_mem);
            }

            // If it's a symbol/label, load address first then store
            return format!("    adr x16, {}\n    str {}, [x16]\n", inner, src_reg);
        }

        // Direct symbol without brackets - load address and store
        format!("    adr x16, {}\n    str {}, [x16]\n", dst, src_reg)
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
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        // ARM64 mul doesn't accept immediate values - load into register first
        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!(
                "    mov x16, #{}\n    mul {}, {}, x16\n",
                src_op, dst_reg, dst_reg
            )
        } else {
            format!("    mul {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
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
        // Linux AArch64: x8 = syscall#, x0..x7 = args, svc 0
        let syscall_num = match name {
            "read" => "63",
            "write" => "64",
            "open" => "56",
            "close" => "57",
            "exit" => "93",
            "mmap" => "222",
            "munmap" => "215",
            "brk" => "214",
            "fstat" => "80",
            _ => {
                return format!(
                    "    // Unknown syscall: {}\n    mov x8, #0\n    svc 0\n",
                    name
                );
            }
        };
        format!("    mov x8, #{}\n    svc 0\n", syscall_num)
    }

    fn generate_cmov_eq(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, eq\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_ne(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, ne\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_lt(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, lt\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_le(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, le\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_gt(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, gt\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_ge(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, ge\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_ov(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, vs\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_no(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, vc\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_s(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, mi\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_ns(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, pl\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_p(&self, dst: &str, src: &str) -> String {
        "// ARM64 has no parity flag, cannot synthesize cmov_p\n".to_string()
    }
    fn generate_cmov_np(&self, dst: &str, src: &str) -> String {
        "// ARM64 has no parity flag, cannot synthesize cmov_np\n".to_string()
    }
    fn generate_cmov_a(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, hi\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_ae(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, hs\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_b(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, lo\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }
    fn generate_cmov_be(&self, dst: &str, src: &str) -> String {
        format!(
            "    csel {}, {}, {}, ls\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }

    fn generate_push(&self, src: &str) -> String {
        // ARM64 doesn't have a direct push instruction
        format!("    str {}, [sp, #-16]!\n", self.map_operand(src))
    }

    fn generate_pop(&self, dst: &str) -> String {
        format!("    ldr {}, [sp], #16\n", self.map_operand(dst))
    }

    fn generate_pusha(&self) -> String {
        // ARM64 doesn't have pusha, save multiple registers
        "    stp x0, x1, [sp, #-16]!\n    stp x2, x3, [sp, #-16]!\n    stp x4, x5, [sp, #-16]!\n    stp x6, x7, [sp, #-16]!\n".to_string()
    }

    fn generate_popa(&self) -> String {
        "    ldp x6, x7, [sp], #16\n    ldp x4, x5, [sp], #16\n    ldp x2, x3, [sp], #16\n    ldp x0, x1, [sp], #16\n".to_string()
    }

    fn generate_enter(&self, frame_size: &str, _nesting: &str) -> String {
        format!(
            "    stp x29, x30, [sp, #-16]!\n    mov x29, sp\n    sub sp, sp, #{}\n",
            frame_size
        )
    }

    fn generate_leave(&self) -> String {
        "    mov sp, x29\n    ldp x29, x30, [sp], #16\n".to_string()
    }

    fn generate_imul(&self, dst: &str, src: &str) -> String {
        self.generate_mul(dst, src)
    }

    fn generate_idiv(&self, dst: &str, src: &str) -> String {
        self.generate_div(dst, src)
    }

    fn generate_mod(&self, dst: &str, src: &str) -> String {
        // ARM64: msub after sdiv for modulo
        let t = "x16"; // Use x16 as scratch register
        let dst = self.map_operand(dst);
        let src = self.map_operand(src);
        format!(
            "    sdiv {t}, {dst}, {src}\n    msub {dst}, {t}, {src}, {dst}\n",
            t = t,
            dst = dst,
            src = src
        )
    }

    fn generate_andn(&self, dst: &str, src: &str) -> String {
        format!(
            "    bic {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_sal(&self, dst: &str, src: &str) -> String {
        self.generate_shl(dst, src)
    }

    fn generate_sar(&self, dst: &str, src: &str) -> String {
        format!(
            "    asr {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_rol(&self, dst: &str, src: &str) -> String {
        // ARM64 has ROR, for ROL we use ROR with (64-src)
        let src_val = src.parse::<u32>().unwrap_or(0);
        format!(
            "    ror {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            64 - src_val
        )
    }

    fn generate_ror(&self, dst: &str, src: &str) -> String {
        format!(
            "    ror {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_rcl(&self, _dst: &str, _src: &str) -> String {
        "// ARM64 has no direct RCL (rotate through carry)\n".to_string()
    }

    fn generate_rcr(&self, _dst: &str, _src: &str) -> String {
        "// ARM64 has no direct RCR (rotate through carry)\n".to_string()
    }

    fn generate_bextr(&self, dst: &str, src: &str, imm: &str) -> String {
        // Bit field extract: UBFX
        let src = self.map_operand(src);
        if let Some((lsb, width)) = imm.split_once(',') {
            format!(
                "    ubfx {}, {}, #{}, #{}\n",
                self.map_operand(dst),
                src,
                lsb.trim(),
                width.trim()
            )
        } else {
            "// ARM64: bextr expects imm as lsb,width\n".to_string()
        }
    }

    fn generate_bsf(&self, dst: &str, src: &str) -> String {
        // Count trailing zeros: rbit + clz
        format!(
            "    rbit {}, {}\n    clz {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_bsr(&self, dst: &str, src: &str) -> String {
        // Count leading zeros
        format!(
            "    clz {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_bt(&self, dst: &str, bit: &str) -> String {
        // Test bit: tst dst, #(1 << bit)
        format!(
            "    tst {}, #{}\n",
            self.map_operand(dst),
            1u64 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    fn generate_btr(&self, dst: &str, bit: &str) -> String {
        // Bit reset: bic dst, dst, #(1 << bit)
        format!(
            "    bic {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            1u64 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    fn generate_bts(&self, dst: &str, bit: &str) -> String {
        // Bit set: orr dst, dst, #(1 << bit)
        format!(
            "    orr {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            1u64 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    fn generate_btc(&self, dst: &str, bit: &str) -> String {
        // Bit toggle: eor dst, dst, #(1 << bit)
        format!(
            "    eor {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            1u64 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    fn generate_set_eq(&self, dst: &str) -> String {
        format!("    cset {}, eq\n", self.map_operand(dst))
    }
    fn generate_set_ne(&self, dst: &str) -> String {
        format!("    cset {}, ne\n", self.map_operand(dst))
    }
    fn generate_set_lt(&self, dst: &str) -> String {
        format!("    cset {}, lt\n", self.map_operand(dst))
    }
    fn generate_set_le(&self, dst: &str) -> String {
        format!("    cset {}, le\n", self.map_operand(dst))
    }
    fn generate_set_gt(&self, dst: &str) -> String {
        format!("    cset {}, gt\n", self.map_operand(dst))
    }
    fn generate_set_ge(&self, dst: &str) -> String {
        format!("    cset {}, ge\n", self.map_operand(dst))
    }
    fn generate_set_ov(&self, dst: &str) -> String {
        format!("    cset {}, vs\n", self.map_operand(dst))
    }
    fn generate_set_no(&self, dst: &str) -> String {
        format!("    cset {}, vc\n", self.map_operand(dst))
    }
    fn generate_set_s(&self, dst: &str) -> String {
        format!("    cset {}, mi\n", self.map_operand(dst))
    }
    fn generate_set_ns(&self, dst: &str) -> String {
        format!("    cset {}, pl\n", self.map_operand(dst))
    }
    fn generate_set_p(&self, dst: &str) -> String {
        "// ARM64 has no parity flag, cannot synthesize set_p\n".to_string()
    }
    fn generate_set_np(&self, dst: &str) -> String {
        "// ARM64 has no parity flag, cannot synthesize set_np\n".to_string()
    }
    fn generate_set_a(&self, dst: &str) -> String {
        format!("    cset {}, hi\n", self.map_operand(dst))
    }
    fn generate_set_ae(&self, dst: &str) -> String {
        format!("    cset {}, hs\n", self.map_operand(dst))
    }
    fn generate_set_b(&self, dst: &str) -> String {
        format!("    cset {}, lo\n", self.map_operand(dst))
    }
    fn generate_set_be(&self, dst: &str) -> String {
        format!("    cset {}, ls\n", self.map_operand(dst))
    }

    fn generate_cmps(&self, src1: &str, src2: &str) -> String {
        format!(
            "    ldr x16, {} \n    ldr x17, {}\n    cmp x16, x17\n",
            self.map_memory_operand(src1),
            self.map_memory_operand(src2)
        )
    }

    fn generate_scas(&self, src: &str, val: &str) -> String {
        format!(
            "    ldr x16, {} \n    cmp x16, {}\n",
            self.map_memory_operand(src),
            self.map_operand(val)
        )
    }
    fn generate_stos(&self, dst: &str, src: &str) -> String {
        format!(
            "    str {}, {}\n",
            self.map_operand(src),
            self.map_memory_operand(dst)
        )
    }
    fn generate_lods(&self, dst: &str, src: &str) -> String {
        format!(
            "    ldr {}, {}\n",
            self.map_operand(dst),
            self.map_memory_operand(src)
        )
    }
    fn generate_movs(&self, dst: &str, src: &str) -> String {
        format!(
            "    ldr x16, {}\n    str x16, {}\n",
            self.map_memory_operand(src),
            self.map_memory_operand(dst)
        )
    }

    fn generate_cbw(&self, dst: &str) -> String {
        // Sign-extend byte to word: sxtb
        format!(
            "    sxtb {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }
    fn generate_cwd(&self, dst: &str) -> String {
        // Sign-extend word to doubleword: sxth
        format!(
            "    sxth {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }
    fn generate_cdq(&self, dst: &str) -> String {
        // Sign-extend 32-bit to 64-bit: sxtw
        format!(
            "    sxtw {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }
    fn generate_cqo(&self, dst: &str) -> String {
        "// ARM64: CQO equivalent handled by sxtw\n".to_string()
    }
    fn generate_cwde(&self, dst: &str) -> String {
        format!(
            "    sxth {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }
    fn generate_cdqe(&self, dst: &str) -> String {
        format!(
            "    sxtw {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_jo(&self, label: &str) -> String {
        format!("    b.vs {}\n", label)
    }
    fn generate_jno(&self, label: &str) -> String {
        format!("    b.vc {}\n", label)
    }
    fn generate_js(&self, label: &str) -> String {
        format!("    b.mi {}\n", label)
    }
    fn generate_jns(&self, label: &str) -> String {
        format!("    b.pl {}\n", label)
    }
    fn generate_jp(&self, label: &str) -> String {
        "// No parity bit on ARM64\n".to_string()
    }
    fn generate_jnp(&self, label: &str) -> String {
        "// No parity bit on ARM64\n".to_string()
    }
    fn generate_ja(&self, label: &str) -> String {
        format!("    b.hi {}\n", label)
    }
    fn generate_jae(&self, label: &str) -> String {
        format!("    b.hs {}\n", label)
    }
    fn generate_jb(&self, label: &str) -> String {
        format!("    b.lo {}\n", label)
    }
    fn generate_jbe(&self, label: &str) -> String {
        format!("    b.ls {}\n", label)
    }
    fn generate_loop_eq(&self, label: &str) -> String {
        "// No direct LOOPxx on ARM64 -- emulate with sub and cbz\n".to_string()
    }
    fn generate_loop_ne(&self, label: &str) -> String {
        "// No direct LOOPxx on ARM64 -- emulate with sub and cbnz\n".to_string()
    }

    fn generate_in(&self, _dst: &str, _port: &str) -> String {
        "// ARM64 has no IN instruction, not supported.\n".to_string()
    }
    fn generate_out(&self, _port: &str, _src: &str) -> String {
        "// ARM64 has no OUT instruction, not supported.\n".to_string()
    }
    fn generate_ins(&self, _dst: &str, _port: &str) -> String {
        "// ARM64 has no INS instruction, not supported.\n".to_string()
    }
    fn generate_outs(&self, _port: &str, _src: &str) -> String {
        "// ARM64 has no OUTS instruction, not supported.\n".to_string()
    }

    fn generate_cpuid(&self) -> String {
        "// ARM64 does not have CPUID\n".to_string()
    }
    fn generate_lfence(&self) -> String {
        "    dmb ld\n".to_string()
    }
    fn generate_sfence(&self) -> String {
        "    dmb st\n".to_string()
    }
    fn generate_mfence(&self) -> String {
        "    dmb sy\n".to_string()
    }
    fn generate_prefetch(&self, addr: &str) -> String {
        format!("    prfm pldl1keep, {}\n", self.map_memory_operand(addr))
    }
    fn generate_clflush(&self, addr: &str) -> String {
        "// ARM64 does not support clflush\n".to_string()
    }
    fn generate_clwb(&self, addr: &str) -> String {
        "// ARM64 does not support clwb\n".to_string()
    }

    fn generate_global(&self, symbol: &str) -> String {
        format!(".global {}\n", symbol)
    }
    fn generate_extern(&self, symbol: &str) -> String {
        format!(".extern {}\n", symbol)
    }
    fn generate_align(&self, n: &str) -> String {
        format!(".align {}\n", n)
    }

    fn generate_data_byte(&self, name: &str, values: &[String]) -> String {
        format!("{}: .byte {}\n", name, values.join(", "))
    }
    fn generate_data_word(&self, name: &str, values: &[String]) -> String {
        format!("{}: .hword {}\n", name, values.join(", "))
    }
    fn generate_data_dword(&self, name: &str, values: &[String]) -> String {
        format!("{}: .word {}\n", name, values.join(", "))
    }
    fn generate_data_qword(&self, name: &str, values: &[String]) -> String {
        format!("{}: .quad {}\n", name, values.join(", "))
    }
    fn generate_reserve_byte(&self, name: &str, count: &str) -> String {
        format!("{}: .skip {}\n", name, count)
    }
    fn generate_reserve_word(&self, name: &str, count: &str) -> String {
        format!(
            "{}: .skip {}\n",
            name,
            2 * count.parse::<usize>().unwrap_or(1)
        )
    }
    fn generate_reserve_dword(&self, name: &str, count: &str) -> String {
        format!(
            "{}: .skip {}\n",
            name,
            4 * count.parse::<usize>().unwrap_or(1)
        )
    }
    fn generate_reserve_qword(&self, name: &str, count: &str) -> String {
        format!(
            "{}: .skip {}\n",
            name,
            8 * count.parse::<usize>().unwrap_or(1)
        )
    }
    fn generate_equ(&self, name: &str, value: &str) -> String {
        format!("{} = {}\n", name, value)
    }
    fn generate_section(&self, section: &Section) -> String {
        match section {
            Section::Text => ".section .text\n".to_string(),
            Section::Data => ".section .data\n".to_string(),
            Section::Bss => ".section .bss\n".to_string(),
            Section::Rodata => ".section .rodata\n".to_string(),
            Section::Custom(s) => format!(".section {}\n", s),
        }
    }
    fn generate_label(&self, name: &str) -> String {
        format!("{}:\n", name)
    }

    fn map_operand(&self, operand: &str) -> String {
        if operand.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return operand.to_string();
        }

        if operand.starts_with('[') && operand.ends_with(']') {
            return self.map_memory_operand(operand);
        }

        // Check for register mapping first
        if let Some(mapped) = self.register_map.get(operand) {
            mapped.clone()
        } else {
            // If it's not a register, it might be a symbol - return as-is
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
