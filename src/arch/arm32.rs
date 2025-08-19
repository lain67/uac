use super::*;
use std::collections::HashMap;

pub struct ARM32CodeGen {
    register_map: HashMap<String, String>,
}

impl ARM32CodeGen {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();

        // AAPCS: r0-r3 args, r0 return, r12 scratch, r4-r11 callee-saved, r13 sp, r14 lr, r15 pc
        for i in 0..13 {
            register_map.insert(format!("r{}", i), format!("r{}", i));
        }
        register_map.insert("sp".to_string(), "sp".to_string()); // r13
        register_map.insert("lr".to_string(), "lr".to_string()); // r14
        register_map.insert("pc".to_string(), "pc".to_string()); // r15
        register_map.insert("ip".to_string(), "r12".to_string());
        register_map.insert("fp".to_string(), "r11".to_string());
        register_map.insert("sb".to_string(), "r9".to_string());

        ARM32CodeGen { register_map }
    }

    fn cond_suffix(cond: &str) -> &str {
        match cond {
            "eq" => "eq",
            "ne" => "ne",
            "lt" => "lt",
            "le" => "le",
            "gt" => "gt",
            "ge" => "ge",
            "mi" => "mi",
            "pl" => "pl",
            "vs" => "vs",
            "vc" => "vc",
            "cs" => "cs",
            "cc" => "cc",
            "hi" => "hi",
            "ls" => "ls",
            "al" => "", // always
            _ => "",
        }
    }

    fn map_operand(&self, operand: &str) -> String {
        if operand.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return format!("#{}", operand);
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
        // ARM syntax: [Rn, #imm]
        if operand.starts_with('[') && operand.ends_with(']') {
            let inner = &operand[1..operand.len() - 1].trim();
            if let Some((base, off)) = inner.split_once('+') {
                let base = self.map_operand(base.trim());
                let off = off.trim();
                if off.chars().all(|c| c.is_ascii_digit() || c == '-') {
                    format!("[{}, #{}]", base, off)
                } else {
                    format!("[{}, {}]", base, self.map_operand(off))
                }
            } else if let Some((base, off)) = inner.split_once('-') {
                let base = self.map_operand(base.trim());
                let off = off.trim();
                if off.chars().all(|c| c.is_ascii_digit()) {
                    format!("[{}, #-{}]", base, off)
                } else {
                    format!("[{}, -{}]", base, self.map_operand(off))
                }
            } else {
                format!("[{}]", self.map_operand(inner))
            }
        } else {
            operand.to_string()
        }
    }
}

impl ArchCodeGen for ARM32CodeGen {
    fn get_register_map(&self) -> HashMap<String, String> {
        self.register_map.clone()
    }

    fn get_syntax_header(&self) -> String {
        ".syntax unified\n.arm\n.text\n\n".to_string()
    }

    fn generate_mov(&self, dst: &str, src: &str) -> String {
        let dst = self.map_operand(dst);
        let src = self.map_operand(src);
        format!("    mov {}, {}\n", dst, src)
    }

    fn generate_lea(&self, dst: &str, src: &str) -> String {
        // ARM does not have lea, use add with PC or MOV for literals.
        if src.starts_with('[') && src.ends_with(']') {
            let inner = &src[1..src.len() - 1];
            if let Some((base, off)) = inner.split_once('+') {
                return format!(
                    "    add {}, {}, #{}\n",
                    self.map_operand(dst),
                    self.map_operand(base.trim()),
                    off.trim()
                );
            } else if let Some((base, off)) = inner.split_once('-') {
                return format!(
                    "    sub {}, {}, #{}\n",
                    self.map_operand(dst),
                    self.map_operand(base.trim()),
                    off.trim()
                );
            } else {
                return format!(
                    "    mov {}, {}\n",
                    self.map_operand(dst),
                    self.map_operand(inner)
                );
            }
        }
        // For named label
        format!("    ldr {}, ={}\n", self.map_operand(dst), src)
    }

    fn generate_load(&self, dst: &str, src: &str) -> String {
        // Only support ldr reg, [mem]
        let dst = self.map_operand(dst);
        let mem = self.map_memory_operand(src);
        format!("    ldr {}, {}\n", dst, mem)
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        // Only support str reg, [mem]
        let mem = self.map_memory_operand(dst);
        let src = self.map_operand(src);
        format!("    str {}, {}\n", src, mem)
    }

    fn generate_add(&self, dst: &str, src: &str) -> String {
        format!(
            "    add {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_sub(&self, dst: &str, src: &str) -> String {
        format!(
            "    sub {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_mul(&self, dst: &str, src: &str) -> String {
        // ARM: mul <Rd>,<Rm>,<Rs>   (Rd = Rm * Rs)
        format!(
            "    mul {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_div(&self, dst: &str, src: &str) -> String {
        // ARMv7-M+ has SDIV or UDIV
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
            "    rsb {}, {}, #0\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_and(&self, dst: &str, src: &str) -> String {
        format!(
            "    and {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_or(&self, dst: &str, src: &str) -> String {
        format!(
            "    orr {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_xor(&self, dst: &str, src: &str) -> String {
        format!(
            "    eor {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_not(&self, dst: &str) -> String {
        format!(
            "    mvn {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }

    fn generate_shl(&self, dst: &str, src: &str) -> String {
        // ARM: lsl (logical shift left)
        format!(
            "    lsl {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_shr(&self, dst: &str, src: &str) -> String {
        // ARM: lsr (logical shift right)
        format!(
            "    lsr {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_cmp(&self, op1: &str, op2: &str) -> String {
        format!(
            "    cmp {}, {}\n",
            self.map_operand(op1),
            self.map_operand(op2)
        )
    }

    fn generate_test(&self, op1: &str, op2: &str) -> String {
        format!(
            "    tst {}, {}\n",
            self.map_operand(op1),
            self.map_operand(op2)
        )
    }

    fn generate_jmp(&self, label: &str) -> String {
        format!("    b {}\n", label)
    }

    fn generate_je(&self, label: &str) -> String {
        format!("    beq {}\n", label)
    }

    fn generate_jne(&self, label: &str) -> String {
        format!("    bne {}\n", label)
    }

    fn generate_jg(&self, label: &str) -> String {
        format!("    bgt {}\n", label)
    }

    fn generate_jl(&self, label: &str) -> String {
        format!("    blt {}\n", label)
    }

    fn generate_jge(&self, label: &str) -> String {
        format!("    bge {}\n", label)
    }

    fn generate_jle(&self, label: &str) -> String {
        format!("    ble {}\n", label)
    }

    fn generate_call(&self, func: &str) -> String {
        format!("    bl {}\n", func)
    }

    fn generate_ret(&self) -> String {
        "    bx lr\n".to_string()
    }

    fn generate_syscall(&self, name: &str) -> String {
        // Linux (EABI): r7 = syscall#, r0..r6 = args, swi 0
        let syscall_num = match name {
            "read" => "3",
            "write" => "4",
            "open" => "5",
            "close" => "6",
            "exit" => "1",
            "mmap2" => "192",
            "munmap" => "91",
            "brk" => "45",
            "fstat64" => "197",
            _ => {
                return format!(
                    "    // Unknown syscall: {}\n    mov r7, #0\n    swi 0\n",
                    name
                );
            }
        };
        format!("    mov r7, #{}\n    swi 0\n", syscall_num)
    }

    // --- Synthesize "conditional move" by using MOV{cond} ---

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
    fn generate_cmov_p(&self, dst: &str, src: &str) -> String {
        "// ARM has no parity flag, cannot synthesize cmov_p\n".to_string()
    }
    fn generate_cmov_np(&self, dst: &str, src: &str) -> String {
        "// ARM has no parity flag, cannot synthesize cmov_np\n".to_string()
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

    // Stack push/pop
    fn generate_push(&self, src: &str) -> String {
        // STMFD/STMDB for full descending stack
        format!("    push {{{}}}\n", self.map_operand(src))
    }

    fn generate_pop(&self, dst: &str) -> String {
        format!("    pop {{{}}}\n", self.map_operand(dst))
    }

    fn generate_pusha(&self) -> String {
        // ARM has no pusha/popa, emulate full register push
        "    push {r0-r12,lr}\n".to_string()
    }

    fn generate_popa(&self) -> String {
        // ARM has no pusha/popa, emulate full register pop (excluding PC for return)
        "    pop {r0-r12,lr}\n".to_string()
    }

    fn generate_enter(&self, frame_size: &str, _nesting: &str) -> String {
        // Prologue: push fp, set fp, sub sp for locals
        format!(
            "    push {{fp, lr}}\n    add fp, sp, #4\n    sub sp, sp, #{}\n",
            frame_size
        )
    }

    fn generate_leave(&self) -> String {
        // Epilogue: restore fp, lr (and optionally deallocate locals)
        "    mov sp, fp\n    pop {fp, pc}\n".to_string()
    }

    fn generate_imul(&self, dst: &str, src: &str) -> String {
        self.generate_mul(dst, src)
    }

    fn generate_idiv(&self, dst: &str, src: &str) -> String {
        self.generate_div(dst, src)
    }

    fn generate_mod(&self, dst: &str, src: &str) -> String {
        // rX % rY: sdiv rA, rX, rY; mls rA, rA, rY, rX
        // result = rX - (rX / rY) * rY
        let t = self.map_operand("r12"); // Use scratch register r12 (ip)
        let dst = self.map_operand(dst);
        let src = self.map_operand(src);
        format!(
            "    sdiv {t}, {dst}, {src}\n    mls {dst}, {t}, {src}, {dst}\n",
            t = t,
            dst = dst,
            src = src
        )
    }

    fn generate_andn(&self, dst: &str, src: &str) -> String {
        // ANDN dst, src: dst = dst & ~src
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
        // Arithmetic shift right
        format!(
            "    asr {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_rol(&self, dst: &str, src: &str) -> String {
        // ROR: rotate right, for left: ROR with (32-src)
        format!(
            "    ror {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            32 - src.parse::<u32>().unwrap_or(0)
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
        "// ARM has no direct RCL (rotate through carry)\n".to_string()
    }

    fn generate_rcr(&self, _dst: &str, _src: &str) -> String {
        "// ARM has no direct RCR (rotate through carry)\n".to_string()
    }

    fn generate_bextr(&self, dst: &str, src: &str, imm: &str) -> String {
        // Bit extract: ARM32 use UBFX
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
            "// ARM32: bextr expects imm as lsb,width\n".to_string()
        }
    }

    fn generate_bsf(&self, dst: &str, src: &str) -> String {
        // Count trailing zeros (CLZ, then sub from 31): ARM32 has CLZ for leading zeros
        format!(
            "    clz {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        ) // For leading-zero, not bit-scan-forward!
    }

    fn generate_bsr(&self, dst: &str, src: &str) -> String {
        self.generate_bsf(dst, src)
    }

    fn generate_bt(&self, dst: &str, bit: &str) -> String {
        // Test bit: tst dst, #(1 << bit)
        format!(
            "    tst {}, #{}\n",
            self.map_operand(dst),
            1 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    fn generate_btr(&self, dst: &str, bit: &str) -> String {
        // Bit reset: bic dst, dst, #(1 << bit)
        format!(
            "    bic {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            1 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    fn generate_bts(&self, dst: &str, bit: &str) -> String {
        // Bit set: orr dst, dst, #(1 << bit)
        format!(
            "    orr {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            1 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    fn generate_btc(&self, dst: &str, bit: &str) -> String {
        // Bit toggle: eor dst, dst, #(1 << bit)
        format!(
            "    eor {}, {}, #{}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            1 << bit.parse::<u32>().unwrap_or(0)
        )
    }

    // --- Condition codes: Synthesize using MOV{cond} after CMP/TST ---

    fn generate_set_eq(&self, dst: &str) -> String {
        // Set dst=1 if ZF; dst=0 otherwise. Use MOVxx after cmp.
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
    fn generate_set_p(&self, dst: &str) -> String {
        "// ARM32 has no parity flag, cannot synthesize set_p\n".to_string()
    }
    fn generate_set_np(&self, dst: &str) -> String {
        "// ARM32 has no parity flag, cannot synthesize set_np\n".to_string()
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

    // --- String/rep-style ops: ARM lacks direct analogues ---

    fn generate_cmps(&self, src1: &str, src2: &str) -> String {
        format!(
            "    ldr r12, {} \n    ldr r11, {}\n    cmp r12, r11\n",
            self.map_memory_operand(src1),
            self.map_memory_operand(src2)
        )
    }

    fn generate_scas(&self, src: &str, val: &str) -> String {
        format!(
            "    ldr r12, {} \n    cmp r12, {}\n",
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

    // --- Data/sign/zero extend ---
    fn generate_cbw(&self, dst: &str) -> String {
        // Sign-extend byte to word: sxtb <dst>, <dst>
        format!(
            "    sxtb {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }
    fn generate_cwd(&self, dst: &str) -> String {
        // Sign-extend word to doubleword: sxth <dst>, <dst>
        format!(
            "    sxth {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst)
        )
    }
    fn generate_cdq(&self, dst: &str) -> String {
        format!(
            "    // No direct analog (CDQ), use cmp + instruction as needed for sign extension\n"
        )
    }
    fn generate_cqo(&self, dst: &str) -> String {
        "// No direct CQO in ARM32\n".to_string()
    }
    fn generate_cwde(&self, dst: &str) -> String {
        "// No direct CWDE in ARM32\n".to_string()
    }
    fn generate_cdqe(&self, dst: &str) -> String {
        "// No direct CDQE in ARM32\n".to_string()
    }

    // --- ARM32 branch mnemonics ---
    fn generate_jo(&self, label: &str) -> String {
        format!("    bvs {}\n", label)
    }
    fn generate_jno(&self, label: &str) -> String {
        format!("    bvc {}\n", label)
    }
    fn generate_js(&self, label: &str) -> String {
        format!("    bmi {}\n", label)
    }
    fn generate_jns(&self, label: &str) -> String {
        format!("    bpl {}\n", label)
    }
    fn generate_jp(&self, label: &str) -> String {
        "// No parity bit on ARM32\n".to_string()
    }
    fn generate_jnp(&self, label: &str) -> String {
        "// No parity bit on ARM32\n".to_string()
    }
    fn generate_ja(&self, label: &str) -> String {
        format!("    bhi {}\n", label)
    }
    fn generate_jae(&self, label: &str) -> String {
        format!("    bcs {}\n", label)
    }
    fn generate_jb(&self, label: &str) -> String {
        format!("    bcc {}\n", label)
    }
    fn generate_jbe(&self, label: &str) -> String {
        format!("    bls {}\n", label)
    }
    fn generate_loop_eq(&self, label: &str) -> String {
        "// No direct LOOPxx on ARM32 -- emulate with cmp and beq\n".to_string()
    }
    fn generate_loop_ne(&self, label: &str) -> String {
        "// No direct LOOPxx on ARM32 -- emulate with cmp and bne\n".to_string()
    }

    // --- ARM32 has no port I/O ---
    fn generate_in(&self, _dst: &str, _port: &str) -> String {
        "// ARM32 has no IN instruction, not supported.\n".to_string()
    }
    fn generate_out(&self, _port: &str, _src: &str) -> String {
        "// ARM32 has no OUT instruction, not supported.\n".to_string()
    }
    fn generate_ins(&self, _dst: &str, _port: &str) -> String {
        "// ARM32 has no INS instruction, not supported.\n".to_string()
    }
    fn generate_outs(&self, _port: &str, _src: &str) -> String {
        "// ARM32 has no OUTS instruction, not supported.\n".to_string()
    }

    // --- Misc system/fence direct ---
    fn generate_cpuid(&self) -> String {
        "// ARM does not have CPUID\n".to_string()
    }
    fn generate_lfence(&self) -> String {
        "// ARM does not have LFENCE\n".to_string()
    }
    fn generate_sfence(&self) -> String {
        "// ARM does not have SFENCE\n".to_string()
    }
    fn generate_mfence(&self) -> String {
        "// ARM does not have MFENCE\n".to_string()
    }
    fn generate_prefetch(&self, addr: &str) -> String {
        format!("    pld {}\n", self.map_memory_operand(addr))
    }
    fn generate_clflush(&self, addr: &str) -> String {
        "// ARM32 does not support clflush\n".to_string()
    }
    fn generate_clwb(&self, addr: &str) -> String {
        "// ARM32 does not support clwb\n".to_string()
    }

    // --- Data/Section/Align directives ---
    fn generate_global(&self, symbol: &str) -> String {
        format!(".globl {}\n", symbol)
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
        format!("{}: .hword {}\n", name, values.join(", ")) // 2 bytes for .hword
    }
    fn generate_data_dword(&self, name: &str, values: &[String]) -> String {
        format!("{}: .word {}\n", name, values.join(", ")) // 4 bytes for .word
    }
    fn generate_data_qword(&self, name: &str, values: &[String]) -> String {
        format!("{}: .quad {}\n", name, values.join(", ")) // .quad may not be available in old GNU as
    }
    fn generate_reserve_byte(&self, name: &str, count: &str) -> String {
        format!("{}: .skip {}\n", name, count)
    }
    fn generate_reserve_word(&self, name: &str, count: &str) -> String {
        // Each word is 2 bytes
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
