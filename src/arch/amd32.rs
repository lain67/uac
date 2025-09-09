use super::*;
use std::collections::HashMap;

pub struct AMD32CodeGen {
    register_map: HashMap<String, String>,
}

impl AMD32CodeGen {
    pub fn new() -> Self {
        let mut register_map = HashMap::with_capacity(32);

        // Function argument registers (typically passed on stack in 32-bit)
        // But we'll map to available registers for consistency
        register_map.insert("r0".to_string(), "eax".to_string()); // 1st arg/return value
        register_map.insert("r1".to_string(), "ecx".to_string()); // 2nd arg
        register_map.insert("r2".to_string(), "edx".to_string()); // 3rd arg
        register_map.insert("r3".to_string(), "ebx".to_string()); // 4th arg
        register_map.insert("r4".to_string(), "esi".to_string()); // 5th arg
        register_map.insert("r5".to_string(), "edi".to_string()); // 6th arg

        // General-purpose registers
        register_map.insert("r6".to_string(), "eax".to_string());
        register_map.insert("r7".to_string(), "ebx".to_string());
        register_map.insert("r8".to_string(), "ecx".to_string());
        register_map.insert("r9".to_string(), "edx".to_string());
        register_map.insert("r10".to_string(), "esi".to_string());
        register_map.insert("r11".to_string(), "edi".to_string());
        register_map.insert("r12".to_string(), "eax".to_string()); // Reuse eax
        register_map.insert("r13".to_string(), "ebx".to_string()); // Reuse ebx
        register_map.insert("r14".to_string(), "ecx".to_string()); // Reuse ecx
        register_map.insert("r15".to_string(), "edx".to_string()); // Reuse edx
        register_map.insert("r16".to_string(), "esi".to_string()); // Reuse esi
        register_map.insert("r17".to_string(), "edi".to_string()); // Reuse edi
        register_map.insert("r18".to_string(), "eax".to_string()); // Reuse eax
        register_map.insert("r19".to_string(), "ebx".to_string()); // Reuse ebx
        register_map.insert("r20".to_string(), "ecx".to_string()); // Reuse ecx
        register_map.insert("r21".to_string(), "edx".to_string()); // Reuse edx
        register_map.insert("r22".to_string(), "esi".to_string()); // Reuse esi
        register_map.insert("r23".to_string(), "edi".to_string()); // Reuse edi

        // Special purpose registers
        register_map.insert("sp".to_string(), "esp".to_string());
        register_map.insert("sb".to_string(), "ebp".to_string());
        register_map.insert("ip".to_string(), "eip".to_string());

        AMD32CodeGen { register_map }
    }
}

impl ArchCodeGen for AMD32CodeGen {
    fn get_register_map(&self) -> HashMap<String, String> {
        self.register_map.clone()
    }

    fn get_syntax_header(&self) -> String {
        ".intel_syntax noprefix\n.text\n\n".to_string()
    }

    fn generate_mov(&self, dst: &str, src: &str) -> String {
        format!(
            "    mov {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_lea(&self, dst: &str, src: &str) -> String {
        format!(
            "    lea {}, {}\n",
            self.map_operand(dst),
            self.map_memory_operand(src)
        )
    }

    fn generate_load(&self, dst: &str, src: &str) -> String {
        format!(
            "    mov {}, DWORD PTR {}\n",
            self.map_operand(dst),
            self.map_memory_operand(src)
        )
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        format!(
            "    mov DWORD PTR {}, {}\n",
            self.map_memory_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_add(&self, dst: &str, src: &str) -> String {
        format!(
            "    add {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_sub(&self, dst: &str, src: &str) -> String {
        format!(
            "    sub {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_mul(&self, dst: &str, src: &str) -> String {
        format!(
            "    imul {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_div(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        let mut result = String::new();

        let need_save_edx = dst_reg != "edx" && src_op != "edx";
        if need_save_edx {
            result.push_str("    push edx\n");
        }

        if dst_reg != "eax" {
            result.push_str(&format!("    mov eax, {}\n", dst_reg));
        }
        result.push_str("    cdq\n"); // Sign extend eax to edx:eax
        result.push_str(&format!("    idiv {}\n", src_op));
        if dst_reg != "eax" {
            result.push_str(&format!("    mov {}, eax\n", dst_reg));
        }
        if need_save_edx {
            result.push_str("    pop edx\n");
        }

        result
    }

    fn generate_inc(&self, dst: &str) -> String {
        format!("    inc {}\n", self.map_operand(dst))
    }

    fn generate_dec(&self, dst: &str) -> String {
        format!("    dec {}\n", self.map_operand(dst))
    }

    fn generate_neg(&self, dst: &str) -> String {
        format!("    neg {}\n", self.map_operand(dst))
    }

    fn generate_and(&self, dst: &str, src: &str) -> String {
        format!(
            "    and {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_or(&self, dst: &str, src: &str) -> String {
        format!(
            "    or {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_xor(&self, dst: &str, src: &str) -> String {
        format!(
            "    xor {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_not(&self, dst: &str) -> String {
        format!("    not {}\n", self.map_operand(dst))
    }

    fn generate_shl(&self, dst: &str, src: &str) -> String {
        let src_op = self.map_operand(src);
        if src_op != "cl" && !src_op.chars().all(|c| c.is_ascii_digit()) {
            format!(
                "    mov cl, {}\n    shl {}, cl\n",
                src_op,
                self.map_operand(dst)
            )
        } else {
            format!("    shl {}, {}\n", self.map_operand(dst), src_op)
        }
    }

    fn generate_shr(&self, dst: &str, src: &str) -> String {
        let src_op = self.map_operand(src);
        if src_op != "cl" && !src_op.chars().all(|c| c.is_ascii_digit()) {
            format!(
                "    mov cl, {}\n    shr {}, cl\n",
                src_op,
                self.map_operand(dst)
            )
        } else {
            format!("    shr {}, {}\n", self.map_operand(dst), src_op)
        }
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
            "    test {}, {}\n",
            self.map_operand(op1),
            self.map_operand(op2)
        )
    }

    fn generate_jmp(&self, label: &str) -> String {
        format!("    jmp {}\n", label)
    }

    fn generate_je(&self, label: &str) -> String {
        format!("    je {}\n", label)
    }

    fn generate_jne(&self, label: &str) -> String {
        format!("    jne {}\n", label)
    }

    fn generate_jg(&self, label: &str) -> String {
        format!("    jg {}\n", label)
    }

    fn generate_jl(&self, label: &str) -> String {
        format!("    jl {}\n", label)
    }

    fn generate_jge(&self, label: &str) -> String {
        format!("    jge {}\n", label)
    }

    fn generate_jle(&self, label: &str) -> String {
        format!("    jle {}\n", label)
    }

    fn generate_call(&self, func: &str) -> String {
        format!("    call {}\n", func)
    }

    fn generate_ret(&self) -> String {
        "    ret\n".to_string()
    }

    fn generate_syscall(&self, name: &str) -> String {
        // 32-bit Linux syscalls use int 0x80
        let syscall_num = match name {
            "read" => "3",
            "write" => "4",
            "open" => "5",
            "close" => "6",
            "exit" => "1",
            "mmap" => "90",
            "munmap" => "91",
            "brk" => "45",
            _ => {
                return format!(
                    "    # Unknown syscall: {}\n    mov eax, 0\n    int 0x80\n",
                    name
                );
            }
        };
        format!("    mov eax, {}\n    int 0x80\n", syscall_num)
    }

    // Conditional Moves (Pentium Pro+)
    fn generate_cmov_eq(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len()) % 10000;
            format!(
                "    je .Lcmove_set_{}\n    jmp .Lcmove_end_{}\n.Lcmove_set_{}:\n    mov {}, {}\n.Lcmove_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmove {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_ne(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 1) % 10000;
            format!(
                "    jne .Lcmovne_set_{}\n    jmp .Lcmovne_end_{}\n.Lcmovne_set_{}:\n    mov {}, {}\n.Lcmovne_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovne {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_lt(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 2) % 10000;
            format!(
                "    jl .Lcmovl_set_{}\n    jmp .Lcmovl_end_{}\n.Lcmovl_set_{}:\n    mov {}, {}\n.Lcmovl_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovl {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_le(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 3) % 10000;
            format!(
                "    jle .Lcmovle_set_{}\n    jmp .Lcmovle_end_{}\n.Lcmovle_set_{}:\n    mov {}, {}\n.Lcmovle_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovle {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_gt(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 4) % 10000;
            format!(
                "    jg .Lcmovg_set_{}\n    jmp .Lcmovg_end_{}\n.Lcmovg_set_{}:\n    mov {}, {}\n.Lcmovg_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovg {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_ge(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 5) % 10000;
            format!(
                "    jge .Lcmovge_set_{}\n    jmp .Lcmovge_end_{}\n.Lcmovge_set_{}:\n    mov {}, {}\n.Lcmovge_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovge {}, {}\n", dst_reg, src_op)
        }
    }

    // Stack
    fn generate_push(&self, src: &str) -> String {
        format!("    push {}\n", self.map_operand(src))
    }
    fn generate_pop(&self, dst: &str) -> String {
        format!("    pop {}\n", self.map_operand(dst))
    }

    // Data Section
    fn generate_global(&self, symbol: &str) -> String {
        format!(".global {}\n", symbol)
    }
    fn generate_extern(&self, symbol: &str) -> String {
        format!(".extern {}\n", symbol)
    }
    fn generate_align(&self, n: &str) -> String {
        format!(".p2align {}\n", n)
    }
    fn generate_data_byte(&self, name: &str, values: &[String]) -> String {
        format!("{}: .byte {}\n", name, values.join(", "))
    }
    fn generate_data_word(&self, name: &str, values: &[String]) -> String {
        format!("{}: .word {}\n", name, values.join(", "))
    }
    fn generate_data_dword(&self, name: &str, values: &[String]) -> String {
        format!("{}: .long {}\n", name, values.join(", "))
    }
    fn generate_data_qword(&self, name: &str, values: &[String]) -> String {
        // In 32-bit, qword is still supported but less common
        format!("{}: .quad {}\n", name, values.join(", "))
    }
    fn generate_reserve_byte(&self, name: &str, count: &str) -> String {
        format!("{}: .skip {}, 0\n", name, count)
    }
    fn generate_reserve_word(&self, name: &str, count: &str) -> String {
        format!("{}: .skip {}, 0\n", name, count)
    }
    fn generate_reserve_dword(&self, name: &str, count: &str) -> String {
        // Each dword: 4 bytes
        format!(
            "{}: .skip {}, 0\n",
            name,
            4 * count.parse::<usize>().unwrap_or(1)
        )
    }
    fn generate_reserve_qword(&self, name: &str, count: &str) -> String {
        // Each qword: 8 bytes
        format!(
            "{}: .skip {}, 0\n",
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

    fn generate_in(&self, dst: &str, port: &str) -> String {
        format!(
            "    in {}, {}\n",
            self.map_operand(dst),
            self.map_operand(port)
        )
    }
    fn generate_out(&self, port: &str, src: &str) -> String {
        format!(
            "    out {}, {}\n",
            self.map_operand(port),
            self.map_operand(src)
        )
    }
    fn generate_ins(&self, dst: &str, port: &str) -> String {
        format!(
            "    insd {}, {}\n",
            self.map_operand(dst),
            self.map_operand(port)
        )
    }
    fn generate_outs(&self, port: &str, src: &str) -> String {
        format!(
            "    outsd {}, {}\n",
            self.map_operand(port),
            self.map_operand(src)
        )
    }

    fn generate_sal(&self, dst: &str, src: &str) -> String {
        // synonym for SHL
        self.generate_shl(dst, src)
    }
    fn generate_sar(&self, dst: &str, src: &str) -> String {
        let src_op = self.map_operand(src);
        if src_op != "cl" && !src_op.chars().all(|c| c.is_ascii_digit()) {
            format!(
                "    mov cl, {}\n    sar {}, cl\n",
                src_op,
                self.map_operand(dst)
            )
        } else {
            format!("    sar {}, {}\n", self.map_operand(dst), src_op)
        }
    }
    fn generate_rol(&self, dst: &str, src: &str) -> String {
        let src_op = self.map_operand(src);
        if src_op != "cl" && !src_op.chars().all(|c| c.is_ascii_digit()) {
            format!(
                "    mov cl, {}\n    rol {}, cl\n",
                src_op,
                self.map_operand(dst)
            )
        } else {
            format!("    rol {}, {}\n", self.map_operand(dst), src_op)
        }
    }
    fn generate_ror(&self, dst: &str, src: &str) -> String {
        let src_op = self.map_operand(src);
        if src_op != "cl" && !src_op.chars().all(|c| c.is_ascii_digit()) {
            format!(
                "    mov cl, {}\n    ror {}, cl\n",
                src_op,
                self.map_operand(dst)
            )
        } else {
            format!("    ror {}, {}\n", self.map_operand(dst), src_op)
        }
    }
    fn generate_rcl(&self, dst: &str, src: &str) -> String {
        let src_op = self.map_operand(src);
        if src_op != "cl" && !src_op.chars().all(|c| c.is_ascii_digit()) {
            format!(
                "    mov cl, {}\n    rcl {}, cl\n",
                src_op,
                self.map_operand(dst)
            )
        } else {
            format!("    rcl {}, {}\n", self.map_operand(dst), src_op)
        }
    }
    fn generate_rcr(&self, dst: &str, src: &str) -> String {
        let src_op = self.map_operand(src);
        if src_op != "cl" && !src_op.chars().all(|c| c.is_ascii_digit()) {
            format!(
                "    mov cl, {}\n    rcr {}, cl\n",
                src_op,
                self.map_operand(dst)
            )
        } else {
            format!("    rcr {}, {}\n", self.map_operand(dst), src_op)
        }
    }

    fn generate_imul(&self, dst: &str, src: &str) -> String {
        format!(
            "    imul {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }
    fn generate_idiv(&self, dst: &str, src: &str) -> String {
        // Same pattern as generate_div:
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        let mut result = String::new();
        let need_save_edx = dst_reg != "edx" && src_op != "edx";
        if need_save_edx {
            result.push_str("    push edx\n");
        }
        if dst_reg != "eax" {
            result.push_str(&format!("    mov eax, {}\n", dst_reg));
        }
        result.push_str("    cdq\n");
        result.push_str(&format!("    idiv {}\n", src_op));
        if dst_reg != "eax" {
            result.push_str(&format!("    mov {}, eax\n", dst_reg));
        }
        if need_save_edx {
            result.push_str("    pop edx\n");
        }
        result
    }
    fn generate_mod(&self, dst: &str, src: &str) -> String {
        // Store result (remainder) in dst, like idiv but copy edx to dst
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        let mut result = String::new();
        let need_save_edx = dst_reg != "edx" && src_op != "edx";
        if need_save_edx {
            result.push_str("    push edx\n");
        }
        result.push_str(&format!("    mov eax, {}\n", dst_reg));
        result.push_str("    cdq\n");
        result.push_str(&format!("    idiv {}\n", src_op));
        if dst_reg != "edx" {
            result.push_str(&format!("    mov {}, edx\n", dst_reg));
        }
        if need_save_edx {
            result.push_str("    pop edx\n");
        }
        result
    }

    fn generate_cmov_ov(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 6) % 10000;
            format!(
                "    jo .Lcmovo_set_{}\n    jmp .Lcmovo_end_{}\n.Lcmovo_set_{}:\n    mov {}, {}\n.Lcmovo_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovo {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_no(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 7) % 10000;
            format!(
                "    jno .Lcmovno_set_{}\n    jmp .Lcmovno_end_{}\n.Lcmovno_set_{}:\n    mov {}, {}\n.Lcmovno_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovno {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_s(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 8) % 10000;
            format!(
                "    js .Lcmovs_set_{}\n    jmp .Lcmovs_end_{}\n.Lcmovs_set_{}:\n    mov {}, {}\n.Lcmovs_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovs {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_ns(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 9) % 10000;
            format!(
                "    jns .Lcmovns_set_{}\n    jmp .Lcmovns_end_{}\n.Lcmovns_set_{}:\n    mov {}, {}\n.Lcmovns_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovns {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_p(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 10) % 10000;
            format!(
                "    jp .Lcmovp_set_{}\n    jmp .Lcmovp_end_{}\n.Lcmovp_set_{}:\n    mov {}, {}\n.Lcmovp_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovp {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_np(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 11) % 10000;
            format!(
                "    jnp .Lcmovnp_set_{}\n    jmp .Lcmovnp_end_{}\n.Lcmovnp_set_{}:\n    mov {}, {}\n.Lcmovnp_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovnp {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_a(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 12) % 10000;
            format!(
                "    ja .Lcmova_set_{}\n    jmp .Lcmova_end_{}\n.Lcmova_set_{}:\n    mov {}, {}\n.Lcmova_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmova {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_ae(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 13) % 10000;
            format!(
                "    jae .Lcmovae_set_{}\n    jmp .Lcmovae_end_{}\n.Lcmovae_set_{}:\n    mov {}, {}\n.Lcmovae_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovae {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_b(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 14) % 10000;
            format!(
                "    jb .Lcmovb_set_{}\n    jmp .Lcmovb_end_{}\n.Lcmovb_set_{}:\n    mov {}, {}\n.Lcmovb_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovb {}, {}\n", dst_reg, src_op)
        }
    }
    fn generate_cmov_be(&self, dst: &str, src: &str) -> String {
        let dst_reg = self.map_operand(dst);
        let src_op = self.map_operand(src);
        if src.chars().all(|c| c.is_ascii_digit() || c == '-') {
            let hash = (dst.len() + src.len() + 15) % 10000;
            format!(
                "    jbe .Lcmovbe_set_{}\n    jmp .Lcmovbe_end_{}\n.Lcmovbe_set_{}:\n    mov {}, {}\n.Lcmovbe_end_{}:\n",
                hash, hash, hash, dst_reg, src_op, hash
            )
        } else {
            format!("    cmovbe {}, {}\n", dst_reg, src_op)
        }
    }

    fn generate_pusha(&self) -> String {
        // PUSHA pushes all general-purpose registers
        "    pusha\n".to_string()
    }
    fn generate_popa(&self) -> String {
        // POPA pops all general-purpose registers
        "    popa\n".to_string()
    }

    fn generate_enter(&self, frame_size: &str, nesting_level: &str) -> String {
        format!("    enter {}, {}\n", frame_size, nesting_level)
    }
    fn generate_leave(&self) -> String {
        "    leave\n".to_string()
    }

    // Most advanced instructions are not available in 32-bit or have limited support
    fn generate_andn(&self, dst: &str, src: &str) -> String {
        // BMI1 not typically available in 32-bit, simulate with NOT + AND
        format!(
            "    mov {}, {}\n    not {}\n    and {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_bextr(&self, dst: &str, src: &str, _imm: &str) -> String {
        // Not available in 32-bit, provide comment
        format!(
            "    # BEXTR not available in 32-bit\n    mov {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }
    fn generate_bsf(&self, dst: &str, src: &str) -> String {
        format!(
            "    bsf {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }
    fn generate_bsr(&self, dst: &str, src: &str) -> String {
        format!(
            "    bsr {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }
    fn generate_bt(&self, dst: &str, bit: &str) -> String {
        format!(
            "    bt {}, {}\n",
            self.map_operand(dst),
            self.map_operand(bit)
        )
    }
    fn generate_btr(&self, dst: &str, bit: &str) -> String {
        format!(
            "    btr {}, {}\n",
            self.map_operand(dst),
            self.map_operand(bit)
        )
    }
    fn generate_bts(&self, dst: &str, bit: &str) -> String {
        format!(
            "    bts {}, {}\n",
            self.map_operand(dst),
            self.map_operand(bit)
        )
    }
    fn generate_btc(&self, dst: &str, bit: &str) -> String {
        format!(
            "    btc {}, {}\n",
            self.map_operand(dst),
            self.map_operand(bit)
        )
    }

    fn generate_set_eq(&self, dst: &str) -> String {
        format!("    setz {}\n", self.map_operand(dst))
    }
    fn generate_set_ne(&self, dst: &str) -> String {
        format!("    setnz {}\n", self.map_operand(dst))
    }
    fn generate_set_lt(&self, dst: &str) -> String {
        format!("    setl {}\n", self.map_operand(dst))
    }
    fn generate_set_le(&self, dst: &str) -> String {
        format!("    setle {}\n", self.map_operand(dst))
    }
    fn generate_set_gt(&self, dst: &str) -> String {
        format!("    setg {}\n", self.map_operand(dst))
    }
    fn generate_set_ge(&self, dst: &str) -> String {
        format!("    setge {}\n", self.map_operand(dst))
    }
    fn generate_set_ov(&self, dst: &str) -> String {
        format!("    seto {}\n", self.map_operand(dst))
    }
    fn generate_set_no(&self, dst: &str) -> String {
        format!("    setno {}\n", self.map_operand(dst))
    }
    fn generate_set_s(&self, dst: &str) -> String {
        format!("    sets {}\n", self.map_operand(dst))
    }
    fn generate_set_ns(&self, dst: &str) -> String {
        format!("    setns {}\n", self.map_operand(dst))
    }
    fn generate_set_p(&self, dst: &str) -> String {
        format!("    setp {}\n", self.map_operand(dst))
    }
    fn generate_set_np(&self, dst: &str) -> String {
        format!("    setnp {}\n", self.map_operand(dst))
    }
    fn generate_set_a(&self, dst: &str) -> String {
        format!("    seta {}\n", self.map_operand(dst))
    }
    fn generate_set_ae(&self, dst: &str) -> String {
        format!("    setae {}\n", self.map_operand(dst))
    }
    fn generate_set_b(&self, dst: &str) -> String {
        format!("    setb {}\n", self.map_operand(dst))
    }
    fn generate_set_be(&self, dst: &str) -> String {
        format!("    setbe {}\n", self.map_operand(dst))
    }

    fn generate_cmps(&self, _src1: &str, _src2: &str) -> String {
        "    cmpsd\n".to_string()
    }
    fn generate_scas(&self, _src: &str, _val: &str) -> String {
        "    scasd\n".to_string()
    }
    fn generate_stos(&self, _dst: &str, _src: &str) -> String {
        "    stosd\n".to_string()
    }
    fn generate_lods(&self, _dst: &str, _src: &str) -> String {
        "    lodsd\n".to_string()
    }
    fn generate_movs(&self, _dst: &str, _src: &str) -> String {
        "    movsd\n".to_string()
    }

    fn generate_cbw(&self, _dst: &str) -> String {
        "    cbw\n".to_string()
    }
    fn generate_cwd(&self, _dst: &str) -> String {
        "    cwd\n".to_string()
    }
    fn generate_cdq(&self, _dst: &str) -> String {
        "    cdq\n".to_string()
    }
    fn generate_cqo(&self, _dst: &str) -> String {
        // CQO not available in 32-bit, use CDQ instead
        "    cdq\n".to_string()
    }
    fn generate_cwde(&self, _dst: &str) -> String {
        "    cwde\n".to_string()
    }
    fn generate_cdqe(&self, _dst: &str) -> String {
        // CDQE not available in 32-bit, use CWDE instead
        "    cwde\n".to_string()
    }

    fn generate_jo(&self, label: &str) -> String {
        format!("    jo {}\n", label)
    }
    fn generate_jno(&self, label: &str) -> String {
        format!("    jno {}\n", label)
    }
    fn generate_js(&self, label: &str) -> String {
        format!("    js {}\n", label)
    }
    fn generate_jns(&self, label: &str) -> String {
        format!("    jns {}\n", label)
    }
    fn generate_jp(&self, label: &str) -> String {
        format!("    jp {}\n", label)
    }
    fn generate_jnp(&self, label: &str) -> String {
        format!("    jnp {}\n", label)
    }
    fn generate_ja(&self, label: &str) -> String {
        format!("    ja {}\n", label)
    }
    fn generate_jae(&self, label: &str) -> String {
        format!("    jae {}\n", label)
    }
    fn generate_jb(&self, label: &str) -> String {
        format!("    jb {}\n", label)
    }
    fn generate_jbe(&self, label: &str) -> String {
        format!("    jbe {}\n", label)
    }

    fn generate_loop_eq(&self, label: &str) -> String {
        format!("    loope {}\n", label)
    }
    fn generate_loop_ne(&self, label: &str) -> String {
        format!("    loopne {}\n", label)
    }

    // Utility
    fn generate_label(&self, name: &str) -> String {
        format!("{}:\n", name)
    }
    fn generate_cpuid(&self) -> String {
        "    cpuid\n".to_string()
    }
    fn generate_lfence(&self) -> String {
        // Not available in older 32-bit processors
        "    # lfence not available in 32-bit\n".to_string()
    }
    fn generate_sfence(&self) -> String {
        // Not available in older 32-bit processors
        "    # sfence not available in 32-bit\n".to_string()
    }
    fn generate_mfence(&self) -> String {
        // Not available in older 32-bit processors
        "    # mfence not available in 32-bit\n".to_string()
    }
    fn generate_prefetch(&self, addr: &str) -> String {
        // Limited prefetch support in 32-bit
        format!("    # prefetch {}\n", self.map_memory_operand(addr))
    }
    fn generate_clflush(&self, addr: &str) -> String {
        format!("    clflush {}\n", self.map_memory_operand(addr))
    }
    fn generate_clwb(&self, addr: &str) -> String {
        // Not available in 32-bit
        format!(
            "    # clwb not available in 32-bit: {}\n",
            self.map_memory_operand(addr)
        )
    }

    // Memory/Register mapping functions
    fn map_operand(&self, operand: &str) -> String {
        // Handle immediate values (numbers)
        if operand.chars().all(|c| c.is_ascii_digit() || c == '-') {
            return operand.to_string();
        }

        // Handle memory references
        if operand.starts_with('[') && operand.ends_with(']') {
            return self.map_memory_operand(operand);
        }

        // Map register names
        if let Some(mapped) = self.register_map.get(operand) {
            mapped.clone()
        } else {
            operand.to_string()
        }
    }

    fn map_memory_operand(&self, operand: &str) -> String {
        if operand.starts_with('[') && operand.ends_with(']') {
            let inner = &operand[1..operand.len() - 1].trim();

            // Handle complex addressing modes (base + index + displacement)
            if inner.contains('+') || inner.contains('-') {
                let parts: Vec<&str> = inner.split_whitespace().collect();
                let mut result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    if let Some(mapped) = self.register_map.get(*part) {
                        result.push_str(mapped);
                    } else {
                        result.push_str(part);
                    }
                    if i < parts.len() - 1 {
                        result.push(' ');
                    }
                }
                format!("[{}]", result)
            } else if let Some(mapped) = self.register_map.get(&inner.to_string()) {
                format!("[{}]", mapped)
            } else {
                format!("[{}]", inner)
            }
        } else {
            operand.to_string()
        }
    }
}
