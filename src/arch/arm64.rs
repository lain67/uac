use super::*;

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

        // General-purpose registers
        register_map.insert("r6".to_string(), "x6".to_string());
        register_map.insert("r7".to_string(), "x7".to_string());
        register_map.insert("r8".to_string(), "x8".to_string());
        register_map.insert("r9".to_string(), "x9".to_string());
        register_map.insert("r10".to_string(), "x10".to_string());
        register_map.insert("r11".to_string(), "x11".to_string());
        register_map.insert("r12".to_string(), "x12".to_string());
        register_map.insert("r13".to_string(), "x13".to_string());

        // Special purpose registers
        register_map.insert("sp".to_string(), "sp".to_string());
        register_map.insert("sb".to_string(), "x29".to_string()); // frame pointer
        register_map.insert("ip".to_string(), "pc".to_string());

        ARM64CodeGen { register_map }
    }
}

impl ArchCodeGen for ARM64CodeGen {
    fn get_register_map(&self) -> HashMap<String, String> {
        self.register_map.clone()
    }

    fn get_syntax_header(&self) -> String {
        "\n".to_string() // ARM64 assembly doesn't need special syntax declarations
    }

    fn generate_mov(&self, dst: &str, src: &str) -> String {
        format!(
            "    mov {}, {}\n",
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_lea(&self, dst: &str, src: &str) -> String {
        // ARM64 uses ADR/ADRP for address calculation
        format!(
            "    adr {}, {}\n",
            self.map_operand(dst),
            self.map_memory_operand(src)
        )
    }

    fn generate_load(&self, dst: &str, src: &str) -> String {
        format!(
            "    ldr {}, {}\n",
            self.map_operand(dst),
            self.map_memory_operand(src)
        )
    }

    fn generate_store(&self, dst: &str, src: &str) -> String {
        format!(
            "    str {}, {}\n",
            self.map_operand(src),
            self.map_memory_operand(dst)
        )
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
        format!(
            "    lsl {}, {}, {}\n",
            self.map_operand(dst),
            self.map_operand(dst),
            self.map_operand(src)
        )
    }

    fn generate_shr(&self, dst: &str, src: &str) -> String {
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
            "open" => "1024",
            "close" => "57",
            _ => "0",
        };
        format!("    li a7, {}\n    ecall\n", syscall_num)
    }

    fn map_operand(&self, operand: &str) -> String {
        if let Some(mapped) = self.register_map.get(operand) {
            mapped.clone()
        } else if operand.starts_with('[') && operand.ends_with(']') {
            self.map_memory_operand(operand)
        } else {
            operand.to_string()
        }
    }

    fn map_memory_operand(&self, operand: &str) -> String {
        if operand.starts_with('[') && operand.ends_with(']') {
            let inner = &operand[1..operand.len() - 1].trim();
            if let Some(mapped) = self.register_map.get(inner as &str) {
                format!("0({})", mapped)
            } else {
                format!("0({})", inner)
            }
        } else {
            operand.to_string()
        }
    }
}