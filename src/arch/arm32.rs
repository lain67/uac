use super::*;
use crate::core::Section;
use std::collections::HashMap;

pub struct ARM32CodeGen {
    register_map: HashMap<String, String>,
}

impl ARM32CodeGen {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();

        // ARM32 general purpose registers
        register_map.insert("r0".to_string(), "r0".to_string()); // 1st arg/return
        register_map.insert("r1".to_string(), "r1".to_string()); // 2nd arg
        register_map.insert("r2".to_string(), "r2".to_string()); // 3rd arg
        register_map.insert("r3".to_string(), "r3".to_string()); // 4th arg
        register_map.insert("r4".to_string(), "r4".to_string()); // Callee-saved
        register_map.insert("r5".to_string(), "r5".to_string()); // Callee-saved
        register_map.insert("r6".to_string(), "r6".to_string()); // Callee-saved
        register_map.insert("r7".to_string(), "r7".to_string()); // Callee-saved
        register_map.insert("r8".to_string(), "r8".to_string()); // Callee-saved
        register_map.insert("r9".to_string(), "r9".to_string()); // Platform register
        register_map.insert("r10".to_string(), "r10".to_string()); // Callee-saved
        register_map.insert("r11".to_string(), "r11".to_string()); // Frame pointer
        register_map.insert("r12".to_string(), "r12".to_string()); // IP (scratch)
        register_map.insert("r13".to_string(), "r13".to_string()); // SP
        register_map.insert("r14".to_string(), "r14".to_string()); // LR
        register_map.insert("r15".to_string(), "r15".to_string()); // PC

        // Special purpose register aliases
        register_map.insert("sp".to_string(), "sp".to_string()); // Stack pointer (r13)
        register_map.insert("lr".to_string(), "lr".to_string()); // Link register (r14)
        register_map.insert("pc".to_string(), "pc".to_string()); // Program counter (r15)
        register_map.insert("ip".to_string(), "ip".to_string()); // Intra-procedure call (r12)
        register_map.insert("fp".to_string(), "r11".to_string()); // Frame pointer (r11)

        // Legacy mappings for compatibility
        register_map.insert("r16".to_string(), "r4".to_string()); // Map to available regs
        register_map.insert("r17".to_string(), "r5".to_string());
        register_map.insert("r18".to_string(), "r6".to_string());
        register_map.insert("r19".to_string(), "r7".to_string());
        register_map.insert("r20".to_string(), "r8".to_string());
        register_map.insert("r21".to_string(), "r9".to_string());
        register_map.insert("r22".to_string(), "r10".to_string());

        ARM32CodeGen { register_map }
    }
}

impl ArchCodeGen for ARM32CodeGen {
    fn get_register_map(&self) -> HashMap<String, String> {
        self.register_map.clone()
    }

    fn get_syntax_header(&self) -> String {
        ".syntax unified\n.arch armv7-a\n.text\n\n".to_string()
    }

    fn generate_mov(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let value: i64 = src_op.parse().unwrap_or(0);
            if value >= 0 && value <= 255 {
                return format!("    mov {}, #{}\n", dst_reg, src_op);
            } else if value >= 0 && value <= 65535 {
                let low = value & 0xFFFF;
                return format!("    mov {}, #{}\n", dst_reg, low);
            } else {
                let low = value & 0xFFFF;
                let high = (value >> 16) & 0xFFFF;
                if high == 0 {
                    return format!("    mov {}, #{}\n", dst_reg, low);
                } else {
                    return format!(
                        "    mov {}, #{}\n    movt {}, #{}\n",
                        dst_reg, low, dst_reg, high
                    );
                }
            }
        }

        if src_op.starts_with('r') || src_op == "sp" || src_op == "lr" || src_op == "pc" {
            return format!("    mov {}, {}\n", dst_reg, src_op);
        }

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

        if let Some(mapped_reg) = self.register_map.get(&inner.to_string()) {
            return format!("    ldr {}, [{}]\n", dst_reg, mapped_reg);
        }

        if inner.contains('+') || inner.contains('-') {
            return format!("    ldr {}, {}\n", dst_reg, self.map_memory_operand(src));
        }

        format!("    adr r12, {}\n    ldr {}, [r12]\n", inner, dst_reg)
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        let src_reg = self.map_operand(src);

        if src_reg.chars().all(|c| c.is_ascii_digit()) {
            if dst.starts_with('[') && dst.ends_with(']') {
                let inner = &dst[1..dst.len() - 1].trim();
                if let Some(mapped_reg) = self.register_map.get(&inner.to_string()) {
                    return format!("    mov r12, #{}\n    str r12, [{}]\n", src_reg, mapped_reg);
                } else if inner.contains('+') || inner.contains('-') {
                    let dst_mem = self.map_memory_operand(dst);
                    return format!("    mov r12, #{}\n    str r12, {}\n", src_reg, dst_mem);
                } else {
                    return format!(
                        "    adr r12, {}\n    mov lr, #{}\n    str lr, [r12]\n",
                        inner, src_reg
                    );
                }
            } else {
                return format!(
                    "    adr r12, {}\n    mov lr, #{}\n    str lr, [r12]\n",
                    dst, src_reg
                );
            }
        }

        if dst.starts_with('[') && dst.ends_with(']') {
            let inner = &dst[1..dst.len() - 1].trim();

            if let Some(mapped_reg) = self.register_map.get(&inner.to_string()) {
                return format!("    str {}, [{}]\n", src_reg, mapped_reg);
            }

            if inner.contains('+') || inner.contains('-') {
                let dst_mem = self.map_memory_operand(dst);
                return format!("    str {}, {}\n", src_reg, dst_mem);
            }

            return format!("    adr r12, {}\n    str {}, [r12]\n", inner, src_reg);
        }

        format!("    adr r12, {}\n    str {}, [r12]\n", dst, src_reg)
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

        if src_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!(
                "    mov r12, #{}\n    mul {}, {}, r12\n",
                src_op, dst_reg, dst_reg
            )
        } else {
            format!("    mul {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_div(&self, dst: &str, src: &str) -> String {
        format!(
            "    @ Software division: {} / {}\n    mov r0, {}\n    mov r1, {}\n    bl __aeabi_idiv\n    mov {}, r0\n",
            dst,
            src,
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
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
            "    rsb {}, {}, #0\n",
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

        if src_op.chars().all(|c| c.is_ascii_digit()) {
            format!("    lsl {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    lsl {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_shr(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit()) {
            format!("    lsr {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    lsr {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_cmp(&self, left: &str, right: &str) -> String {
        let left_reg = self.map_operand(left);
        let right_op = self.map_operand(right);

        if right_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    cmp {}, #{}\n", left_reg, right_op)
        } else {
            format!("    cmp {}, {}\n", left_reg, right_op)
        }
    }

    fn generate_test(&self, left: &str, right: &str) -> String {
        let left_reg = self.map_operand(left);
        let right_op = self.map_operand(right);

        if right_op.chars().all(|c| c.is_ascii_digit() || c == '-') {
            format!("    tst {}, #{}\n", left_reg, right_op)
        } else {
            format!("    tst {}, {}\n", left_reg, right_op)
        }
    }

    fn generate_jmp(&self, target: &str) -> String {
        format!("    b {}\n", target)
    }

    fn generate_je(&self, target: &str) -> String {
        format!("    beq {}\n", target)
    }

    fn generate_jne(&self, target: &str) -> String {
        format!("    bne {}\n", target)
    }

    fn generate_jg(&self, target: &str) -> String {
        format!("    bgt {}\n", target)
    }

    fn generate_jl(&self, target: &str) -> String {
        format!("    blt {}\n", target)
    }

    fn generate_jge(&self, target: &str) -> String {
        format!("    bge {}\n", target)
    }

    fn generate_jle(&self, target: &str) -> String {
        format!("    ble {}\n", target)
    }

    fn generate_call(&self, target: &str) -> String {
        format!("    bl {}\n", target)
    }

    fn generate_ret(&self) -> String {
        "    mov pc, lr\n".to_string()
    }

    fn generate_syscall(&self, name: &str) -> String {
        // ARM32 Linux syscall numbers
        let syscall_num = match name {
            "read" => "3",
            "write" => "4",
            "open" => "5",
            "close" => "6",
            "exit" => "1",
            "mmap" => "90",
            "munmap" => "91",
            "brk" => "45",
            "fstat" => "108",
            _ => {
                return format!(
                    "    @ Unknown syscall: {}\n    mov r7, #0\n    swi 0\n",
                    name
                );
            }
        };
        format!("    mov r7, #{}\n    swi 0\n", syscall_num)
    }

    fn generate_cmov_eq(&self, dst: &str, src: &str) -> String {
        format!(
            "    moveq {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_ne(&self, dst: &str, src: &str) -> String {
        format!(
            "    movne {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_lt(&self, dst: &str, src: &str) -> String {
        format!(
            "    movlt {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_le(&self, dst: &str, src: &str) -> String {
        format!(
            "    movle {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_gt(&self, dst: &str, src: &str) -> String {
        format!(
            "    movgt {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_ge(&self, dst: &str, src: &str) -> String {
        format!(
            "    movge {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_ov(&self, dst: &str, src: &str) -> String {
        format!(
            "    movvs {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_no(&self, dst: &str, src: &str) -> String {
        format!(
            "    movvc {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_s(&self, dst: &str, src: &str) -> String {
        format!(
            "    movmi {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_ns(&self, dst: &str, src: &str) -> String {
        format!(
            "    movpl {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_p(&self, _dst: &str, _src: &str) -> String {
        "    @ Parity flag not available in ARM32\n".to_string()
    }

    fn generate_cmov_np(&self, _dst: &str, _src: &str) -> String {
        "    @ Parity flag not available in ARM32\n".to_string()
    }

    fn generate_cmov_a(&self, dst: &str, src: &str) -> String {
        format!(
            "    movhi {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_ae(&self, dst: &str, src: &str) -> String {
        format!(
            "    movcs {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_b(&self, dst: &str, src: &str) -> String {
        format!(
            "    movcc {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmov_be(&self, dst: &str, src: &str) -> String {
        format!(
            "    movls {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_push(&self, src: &str) -> String {
        let src_reg = self.map_operand(src);
        format!("    push {{{}}}\n", src_reg)
    }

    fn generate_pop(&self, dst: &str) -> String {
        format!("    pop {{{}}}\n", self.map_operand(dst))
    }

    fn generate_pusha(&self) -> String {
        "    push {r0-r12, lr}\n".to_string()
    }

    fn generate_popa(&self) -> String {
        "    pop {r0-r12, lr}\n".to_string()
    }

    fn generate_enter(&self, frame_size: &str, _nesting_level: &str) -> String {
        format!(
            "    push {{fp, lr}}\n    mov fp, sp\n    sub sp, sp, #{}\n",
            frame_size
        )
    }

    fn generate_leave(&self) -> String {
        "    mov sp, fp\n    pop {fp, lr}\n".to_string()
    }

    fn generate_imul(&self, dst: &str, src: &str) -> String {
        format!(
            "    mul {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_idiv(&self, dst: &str, src: &str) -> String {
        format!(
            "    @ Signed division: {} / {}\n    mov r0, {}\n    mov r1, {}\n    bl __aeabi_idiv\n    mov {}, r0\n",
            dst,
            src,
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst)
        )
    }

    fn generate_mod(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_reg = self.map_operand(src);
        format!(
            "    @ Modulo operation: {} % {}\n    mov r0, {}\n    mov r1, {}\n    bl __aeabi_idivmod\n    mov {}, r1\n",
            dst, src, dst_reg, src_reg, dst_reg
        )
    }

    fn generate_andn(&self, dst: &str, src: &str) -> String {
        // ARM32 doesn't have andn - emulate with bic (bit clear)
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
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit()) {
            format!("    asr {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    asr {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_rol(&self, dst: &str, src: &str) -> String {
        // ARM32 has ROR but not ROL - emulate with ROR
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit()) {
            let shift_val: u32 = src_op.parse().unwrap_or(0);
            let ror_val = 32 - (shift_val % 32);
            format!("    ror {}, {}, #{}\n", dst_reg, dst_reg, ror_val)
        } else {
            format!(
                "    rsb r12, {}, #32\n    ror {}, {}, r12\n",
                src_op, dst_reg, dst_reg
            )
        }
    }

    fn generate_ror(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);

        if src_op.chars().all(|c| c.is_ascii_digit()) {
            format!("    ror {}, {}, #{}\n", dst_reg, dst_reg, src_op)
        } else {
            format!("    ror {}, {}, {}\n", dst_reg, dst_reg, src_op)
        }
    }

    fn generate_rcl(&self, _dst: &str, _src: &str) -> String {
        "    @ RCL not available in ARM32 - would need carry flag emulation\n".to_string()
    }

    fn generate_rcr(&self, _dst: &str, _src: &str) -> String {
        "    @ RCR not available in ARM32 - would need carry flag emulation\n".to_string()
    }

    fn generate_bextr(&self, dst: &str, src: &str, imm: &str) -> String {
        // ARM32 doesn't have bit field extract - emulate
        // imm format expected: "start,length" or single value
        let dst_reg = self.map_operand(dst);
        let src_reg = self.map_operand(src);

        if let Some((start_str, length_str)) = imm.split_once(',') {
            let start = start_str.trim().parse::<u32>().unwrap_or(0);
            let length = length_str.trim().parse::<u32>().unwrap_or(0);
            format!(
                "    @ Bit field extract emulation\n    lsl {}, {}, #{}\n    lsr {}, {}, #{}\n",
                dst_reg,
                src_reg,
                32u32.saturating_sub(start + length),
                dst_reg,
                dst_reg,
                32u32.saturating_sub(length)
            )
        } else {
            format!("    @ Invalid bextr immediate format: {}\n", imm)
        }
    }

    fn generate_bsf(&self, dst: &str, _src: &str) -> String {
        // ARM32 doesn't have bit scan - would need software implementation
        format!(
            "    @ Bit scan forward - software implementation needed\n    mov {}, #-1\n",
            self.map_operand(dst)
        )
    }

    fn generate_bsr(&self, dst: &str, src: &str) -> String {
        // ARM32 has CLZ (count leading zeros) which can help
        format!(
            "    clz {}, {}\n    rsb {}, {}, #31\n",
            self.map_operand(dst),
            self.map_operand(src),
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_bt(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        format!(
            "    @ Bit test\n    mov r12, #1\n    lsl r12, r12, {}\n    tst {}, r12\n",
            src_op, dst_reg
        )
    }

    fn generate_btr(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        format!(
            "    @ Bit test and reset\n    mov r12, #1\n    lsl r12, r12, {}\n    tst {}, r12\n    bic {}, {}, r12\n",
            src_op, dst_reg, dst_reg, dst_reg
        )
    }

    fn generate_bts(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        format!(
            "    @ Bit test and set\n    mov r12, #1\n    lsl r12, r12, {}\n    tst {}, r12\n    orr {}, {}, r12\n",
            src_op, dst_reg, dst_reg, dst_reg
        )
    }

    fn generate_btc(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        format!(
            "    @ Bit test and complement\n    mov r12, #1\n    lsl r12, r12, {}\n    tst {}, r12\n    eor {}, {}, r12\n",
            src_op, dst_reg, dst_reg, dst_reg
        )
    }

    fn generate_set_eq(&self, dst: &str) -> String {
        format!(
            "    moveq {}, #1\n    movne {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_ne(&self, dst: &str) -> String {
        format!(
            "    movne {}, #1\n    moveq {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_lt(&self, dst: &str) -> String {
        format!(
            "    movlt {}, #1\n    movge {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_le(&self, dst: &str) -> String {
        format!(
            "    movle {}, #1\n    movgt {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_gt(&self, dst: &str) -> String {
        format!(
            "    movgt {}, #1\n    movle {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_ge(&self, dst: &str) -> String {
        format!(
            "    movge {}, #1\n    movlt {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_ov(&self, dst: &str) -> String {
        format!(
            "    movvs {}, #1\n    movvc {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_no(&self, dst: &str) -> String {
        format!(
            "    movvc {}, #1\n    movvs {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_s(&self, dst: &str) -> String {
        format!(
            "    movmi {}, #1\n    movpl {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_ns(&self, dst: &str) -> String {
        format!(
            "    movpl {}, #1\n    movmi {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_p(&self, _dst: &str) -> String {
        "    @ Parity flag not available in ARM32\n".to_string()
    }

    fn generate_set_np(&self, _dst: &str) -> String {
        "    @ Parity flag not available in ARM32\n".to_string()
    }

    fn generate_set_a(&self, dst: &str) -> String {
        format!(
            "    movhi {}, #1\n    movls {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_ae(&self, dst: &str) -> String {
        format!(
            "    movcs {}, #1\n    movcc {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_b(&self, dst: &str) -> String {
        format!(
            "    movcc {}, #1\n    movcs {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_set_be(&self, dst: &str) -> String {
        format!(
            "    movls {}, #1\n    movhi {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_cmps(&self, src1: &str, src2: &str) -> String {
        format!(
            "    ldr r12, {}\n    ldr lr, {}\n    cmp r12, lr\n",
            self.map_memory_operand(src1),
            self.map_memory_operand(src2)
        )
    }

    fn generate_scas(&self, src: &str, val: &str) -> String {
        format!(
            "    ldr r12, {}\n    cmp r12, {}\n",
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
            "    ldr r12, {}\n    str r12, {}\n",
            self.map_memory_operand(src),
            self.map_memory_operand(dst)
        )
    }

    fn generate_cbw(&self, dst: &str) -> String {
        format!(
            "    sxtb {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_cwd(&self, dst: &str) -> String {
        format!(
            "    sxth {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_cdq(&self, dst: &str) -> String {
        format!(
            "    @ CDQ: Sign extend 32-bit to 64-bit not directly available\n    asr {}, {}, #31\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_cqo(&self, _dst: &str) -> String {
        "    @ CQO: 64-bit operations not available in ARM32\n".to_string()
    }

    fn generate_cwde(&self, dst: &str) -> String {
        format!(
            "    sxth {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_cdqe(&self, _dst: &str) -> String {
        "    @ CDQE: 64-bit operations not available in ARM32\n".to_string()
    }

    fn generate_jo(&self, target: &str) -> String {
        format!("    bvs {}\n", target)
    }

    fn generate_jno(&self, target: &str) -> String {
        format!("    bvc {}\n", target)
    }

    fn generate_js(&self, target: &str) -> String {
        format!("    bmi {}\n", target)
    }

    fn generate_jns(&self, target: &str) -> String {
        format!("    bpl {}\n", target)
    }

    fn generate_jp(&self, _target: &str) -> String {
        "    @ Parity flag not available in ARM32\n".to_string()
    }

    fn generate_jnp(&self, _target: &str) -> String {
        "    @ Parity flag not available in ARM32\n".to_string()
    }

    fn generate_ja(&self, target: &str) -> String {
        format!("    bhi {}\n", target)
    }

    fn generate_jae(&self, target: &str) -> String {
        format!("    bcs {}\n", target)
    }

    fn generate_jb(&self, target: &str) -> String {
        format!("    bcc {}\n", target)
    }

    fn generate_jbe(&self, target: &str) -> String {
        format!("    bls {}\n", target)
    }

    fn generate_loop_eq(&self, target: &str) -> String {
        format!(
            "    @ LOOP equivalent: subs r12, r12, #1\n    beq {}\n",
            target
        )
    }

    fn generate_loop_ne(&self, target: &str) -> String {
        format!(
            "    @ LOOP equivalent: subs r12, r12, #1\n    bne {}\n",
            target
        )
    }

    fn generate_in(&self, _dst: &str, _port: &str) -> String {
        "    @ IN instruction not available in ARM32\n".to_string()
    }

    fn generate_out(&self, _port: &str, _src: &str) -> String {
        "    @ OUT instruction not available in ARM32\n".to_string()
    }

    fn generate_ins(&self, _dst: &str, _port: &str) -> String {
        "    @ INS instruction not available in ARM32\n".to_string()
    }

    fn generate_outs(&self, _port: &str, _src: &str) -> String {
        "    @ OUTS instruction not available in ARM32\n".to_string()
    }

    fn generate_cpuid(&self) -> String {
        "    @ CPUID not available in ARM32\n".to_string()
    }

    fn generate_lfence(&self) -> String {
        "    dmb\n".to_string()
    }

    fn generate_sfence(&self) -> String {
        "    dmb st\n".to_string()
    }

    fn generate_mfence(&self) -> String {
        "    dmb sy\n".to_string()
    }

    fn generate_prefetch(&self, addr: &str) -> String {
        format!("    pld {}\n", self.map_memory_operand(addr))
    }

    fn generate_clflush(&self, _addr: &str) -> String {
        "    @ Cache flush not available in ARM32\n".to_string()
    }

    fn generate_clwb(&self, _addr: &str) -> String {
        "    @ Cache writeback not available in ARM32\n".to_string()
    }

    fn generate_global(&self, symbol: &str) -> String {
        format!(".global {}\n.type {}, %function\n", symbol, symbol)
    }

    fn generate_extern(&self, symbol: &str) -> String {
        format!(".extern {}\n", symbol)
    }

    fn generate_align(&self, n: &str) -> String {
        format!(".align {}\n", n)
    }

    fn generate_data_byte(&self, name: &str, values: &[String]) -> String {
        format!(
            ".type {}, %object\n{}: .byte {}\n",
            name,
            name,
            values.join(", ")
        )
    }

    fn generate_data_word(&self, name: &str, values: &[String]) -> String {
        format!(
            ".type {}, %object\n{}: .hword {}\n",
            name,
            name,
            values.join(", ")
        )
    }

    fn generate_data_dword(&self, name: &str, values: &[String]) -> String {
        format!(
            ".type {}, %object\n{}: .word {}\n",
            name,
            name,
            values.join(", ")
        )
    }

    fn generate_data_qword(&self, _name: &str, _values: &[String]) -> String {
        "    @ 64-bit data not directly supported in ARM32\n".to_string()
    }

    fn generate_reserve_byte(&self, name: &str, count: &str) -> String {
        format!(".type {}, %object\n{}: .skip {}\n", name, name, count)
    }

    fn generate_reserve_word(&self, name: &str, count: &str) -> String {
        format!(
            ".type {}, %object\n{}: .skip {}\n",
            name,
            name,
            2 * count.parse::<usize>().unwrap_or(1)
        )
    }

    fn generate_reserve_dword(&self, name: &str, count: &str) -> String {
        format!(
            ".type {}, %object\n{}: .skip {}\n",
            name,
            name,
            4 * count.parse::<usize>().unwrap_or(1)
        )
    }

    fn generate_reserve_qword(&self, _name: &str, _count: &str) -> String {
        "    @ 64-bit reservations not directly supported in ARM32\n".to_string()
    }

    fn generate_equ(&self, name: &str, value: &str) -> String {
        format!("{} = {}\n", name, value)
    }

    fn generate_section(&self, section: &Section) -> String {
        match section {
            Section::Text => ".section .text,\"ax\",%progbits\n".to_string(),
            Section::Data => ".section .data,\"aw\",%progbits\n".to_string(),
            Section::Bss => ".section .bss,\"aw\",%nobits\n".to_string(),
            Section::Rodata => ".section .rodata,\"a\",%progbits\n".to_string(),
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
