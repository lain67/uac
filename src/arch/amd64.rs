use super::*;
use std::collections::HashMap;

pub struct AMD64CodeGen {
    register_map: HashMap<String, String>,
}

impl AMD64CodeGen {
    pub fn new() -> Self {
        let mut register_map = HashMap::new();

        // Function argument registers (System V ABI)
        register_map.insert("r0".to_string(), "rdi".to_string()); // 1st arg
        register_map.insert("r1".to_string(), "rsi".to_string()); // 2nd arg
        register_map.insert("r2".to_string(), "rdx".to_string()); // 3rd arg
        register_map.insert("r3".to_string(), "rcx".to_string()); // 4th arg
        register_map.insert("r4".to_string(), "r8".to_string()); // 5th arg
        register_map.insert("r5".to_string(), "r9".to_string()); // 6th arg

        // General-purpose registers (avoiding conflicts with argument registers)
        register_map.insert("r6".to_string(), "rax".to_string());
        register_map.insert("r7".to_string(), "rbx".to_string());
        register_map.insert("r8".to_string(), "r10".to_string());
        register_map.insert("r9".to_string(), "r11".to_string());
        register_map.insert("r10".to_string(), "r12".to_string());
        register_map.insert("r11".to_string(), "r13".to_string());
        register_map.insert("r12".to_string(), "r14".to_string());
        register_map.insert("r13".to_string(), "r15".to_string());

        // Special purpose registers
        register_map.insert("sp".to_string(), "rsp".to_string());
        register_map.insert("sb".to_string(), "rbp".to_string());
        register_map.insert("ip".to_string(), "rip".to_string());

        AMD64CodeGen { register_map }
    }
}

impl ArchCodeGen for AMD64CodeGen {
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
            "    mov {}, QWORD PTR {}\n",
            self.map_operand(dst),
            self.map_memory_operand(src)
        )
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        format!(
            "    mov QWORD PTR {}, {}\n",
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

        let need_save_rdx = dst_reg != "rdx" && src_op != "rdx";
        if need_save_rdx {
            result.push_str("    push rdx\n");
        }

        if dst_reg != "rax" {
            result.push_str(&format!("    mov rax, {}\n", dst_reg));
        }

        result.push_str("    cqo\n");

        result.push_str(&format!("    idiv {}\n", src_op));

        if dst_reg != "rax" {
            result.push_str(&format!("    mov {}, rax\n", dst_reg));
        }

        if need_save_rdx {
            result.push_str("    pop rdx\n");
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
        let syscall_num = match name {
            "read" => "0",
            "write" => "1",
            "open" => "2",
            "close" => "3",
            "exit" => "60",
            "mmap" => "9",
            "munmap" => "11",
            "brk" => "12",
            _ => {
                return format!(
                    "    # Unknown syscall: {}\n    mov rax, 0\n    syscall\n",
                    name
                );
            }
        };
        format!("    mov rax, {}\n    syscall\n", syscall_num)
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
