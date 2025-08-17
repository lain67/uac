use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Clone)]
pub enum Architecture {
    AMD64,
    ARM64,
    RISCV,
    PowerPC64,
    IA32,
    ARM32,
    MIPS32,
    SPARC64,
    IA64,
    Alpha,
    HPPA,
    K68,
    AVR,
    MSP430,
    SH,
    VAX,
    NIOSII,
    Xtensa,
    ARC,
    Z80,
}

pub enum Format {
    ELF,
    COFF,
    MachO,
    XCOFF,
    A,
    Custom,
}

#[derive(Debug, Clone)]
enum Section {
    Text,
    Data,
    Bss,
    Rodata,
}

#[derive(Debug, Clone)]
enum Instruction {
    Label(String),
    Mov(String, String),
    Lea(String, String),
    Load(String, String),
    Store(String, String),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Inc(String),
    Dec(String),
    Neg(String),
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Not(String),
    Shl(String, String),
    Shr(String, String),
    Cmp(String, String),
    Test(String, String),
    Jmp(String),
    Je(String),
    Jne(String),
    Jg(String),
    Jl(String),
    Jge(String),
    Jle(String),
    Call(String),
    Ret,
    Syscall(String),
    Global(String),
    Extern(String),
    DataByte(String, Vec<String>),
    DataWord(String, Vec<String>),
    DataDword(String, Vec<String>),
    DataQword(String, Vec<String>),
    ReserveByte(String, String),
    Equ(String, String),
    Section(Section),
}

struct Parser {
    lines: Vec<String>,
    current_section: Section,
    constants: HashMap<String, String>,
}

impl Parser {
    fn new(input: &str) -> Self {
        let lines: Vec<String> = input
            .lines()
            .map(|line| {
                let line = if let Some(pos) = line.find(';') {
                    &line[..pos]
                } else {
                    line
                };
                line.trim().to_string()
            })
            .filter(|line| !line.is_empty())
            .collect();

        Parser {
            lines,
            current_section: Section::Text,
            constants: HashMap::new(),
        }
    }

    fn parse(&mut self) -> Result<Vec<Instruction>, String> {
        let mut instructions = Vec::new();

        for i in 0..self.lines.len() {
            let line = self.lines[i].clone();
            if line.starts_with("section") {
                let section = self.parse_section(&line)?;
                if let Some(section_instr) = section {
                    instructions.push(section_instr);
                }
                continue;
            }

            let instruction = self.parse_instruction(&line)?;
            if let Some(instr) = instruction {
                instructions.push(instr);
            }
        }

        Ok(instructions)
    }

    fn parse_section(&mut self, line: &str) -> Result<Option<Instruction>, String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err("Invalid section declaration".to_string());
        }

        match parts[1] {
            ".text" => {
                self.current_section = Section::Text;
                Ok(Some(Instruction::Section(Section::Text)))
            }
            ".data" => {
                self.current_section = Section::Data;
                Ok(Some(Instruction::Section(Section::Data)))
            }
            ".bss" => {
                self.current_section = Section::Bss;
                Ok(Some(Instruction::Section(Section::Bss)))
            }
            ".rodata" => {
                self.current_section = Section::Rodata;
                Ok(Some(Instruction::Section(Section::Rodata)))
            }
            _ => Err(format!("Unknown section: {}", parts[1])),
        }
    }

    fn parse_data_line(&self, line: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '"' && (current.is_empty() || !current.ends_with('\\')) {
                in_quotes = !in_quotes;
                current.push(c);
            } else if c == ',' && !in_quotes {
                if !current.trim().is_empty() {
                    parts.push(current.trim().to_string());
                }
                current.clear();
            } else if c.is_whitespace() && !in_quotes {
                if !current.trim().is_empty() && !current.ends_with(',') {
                    parts.push(current.trim().to_string());
                    current.clear();
                }
            } else {
                current.push(c);
            }
        }

        if !current.trim().is_empty() {
            parts.push(current.trim().to_string());
        }

        parts
    }

    fn parse_instruction(&mut self, line: &str) -> Result<Option<Instruction>, String> {
        if line.ends_with(':') {
            let label = line[..line.len() - 1].to_string();
            return Ok(Some(Instruction::Label(label)));
        }

        if line.contains(" db ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "db" {
                let name = parts[0].clone();
                let values = parts[2..].to_vec();
                return Ok(Some(Instruction::DataByte(name, values)));
            }
        }

        if line.contains(" dw ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "dw" {
                let name = parts[0].clone();
                let values = parts[2..].to_vec();
                return Ok(Some(Instruction::DataWord(name, values)));
            }
        }

        if line.contains(" dd ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "dd" {
                let name = parts[0].clone();
                let values = parts[2..].to_vec();
                return Ok(Some(Instruction::DataDword(name, values)));
            }
        }

        if line.contains(" dq ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "dq" {
                let name = parts[0].clone();
                let values = parts[2..].to_vec();
                return Ok(Some(Instruction::DataQword(name, values)));
            }
        }

        if line.contains(" resb ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "resb" {
                let name = parts[0].clone();
                let value = parts[2].clone();
                return Ok(Some(Instruction::ReserveByte(name, value)));
            }
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }

        if parts.len() >= 3 && parts[1] == "equ" {
            let name = parts[0].to_string();
            let value = parts[2].to_string();
            self.constants.insert(name.clone(), value.clone());
            return Ok(Some(Instruction::Equ(name, value)));
        }

        let cmd = parts[0];
        match cmd {
            "mov" => {
                if parts.len() < 3 {
                    return Err("mov requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Mov(dst, src)))
            }
            "lea" => {
                if parts.len() < 3 {
                    return Err("lea requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Lea(dst, src)))
            }
            "load" => {
                if parts.len() < 3 {
                    return Err("load requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Load(dst, src)))
            }
            "store" => {
                if parts.len() < 3 {
                    return Err("store requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Store(dst, src)))
            }
            "add" => {
                if parts.len() < 3 {
                    return Err("add requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Add(dst, src)))
            }
            "sub" => {
                if parts.len() < 3 {
                    return Err("sub requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Sub(dst, src)))
            }
            "mul" => {
                if parts.len() < 3 {
                    return Err("mul requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Mul(dst, src)))
            }
            "div" => {
                if parts.len() < 3 {
                    return Err("div requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Div(dst, src)))
            }
            "inc" => {
                if parts.len() < 2 {
                    return Err("inc requires 1 operand".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                Ok(Some(Instruction::Inc(dst)))
            }
            "dec" => {
                if parts.len() < 2 {
                    return Err("dec requires 1 operand".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                Ok(Some(Instruction::Dec(dst)))
            }
            "neg" => {
                if parts.len() < 2 {
                    return Err("neg requires 1 operand".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                Ok(Some(Instruction::Neg(dst)))
            }
            "and" => {
                if parts.len() < 3 {
                    return Err("and requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::And(dst, src)))
            }
            "or" => {
                if parts.len() < 3 {
                    return Err("or requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Or(dst, src)))
            }
            "xor" => {
                if parts.len() < 3 {
                    return Err("xor requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Xor(dst, src)))
            }
            "not" => {
                if parts.len() < 2 {
                    return Err("not requires 1 operand".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                Ok(Some(Instruction::Not(dst)))
            }
            "shl" => {
                if parts.len() < 3 {
                    return Err("shl requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Shl(dst, src)))
            }
            "shr" => {
                if parts.len() < 3 {
                    return Err("shr requires 2 operands".to_string());
                }
                let dst = self.clean_operand(parts[1]);
                let src = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Shr(dst, src)))
            }
            "cmp" => {
                if parts.len() < 3 {
                    return Err("cmp requires 2 operands".to_string());
                }
                let op1 = self.clean_operand(parts[1]);
                let op2 = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Cmp(op1, op2)))
            }
            "test" => {
                if parts.len() < 3 {
                    return Err("test requires 2 operands".to_string());
                }
                let op1 = self.clean_operand(parts[1]);
                let op2 = self.clean_operand(parts[2]);
                Ok(Some(Instruction::Test(op1, op2)))
            }
            "jmp" => {
                if parts.len() < 2 {
                    return Err("jmp requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Jmp(parts[1].to_string())))
            }
            "je" => {
                if parts.len() < 2 {
                    return Err("je requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Je(parts[1].to_string())))
            }
            "jne" => {
                if parts.len() < 2 {
                    return Err("jne requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Jne(parts[1].to_string())))
            }
            "jg" => {
                if parts.len() < 2 {
                    return Err("jg requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Jg(parts[1].to_string())))
            }
            "jl" => {
                if parts.len() < 2 {
                    return Err("jl requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Jl(parts[1].to_string())))
            }
            "jge" => {
                if parts.len() < 2 {
                    return Err("jge requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Jge(parts[1].to_string())))
            }
            "jle" => {
                if parts.len() < 2 {
                    return Err("jle requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Jle(parts[1].to_string())))
            }
            "call" => {
                if parts.len() < 2 {
                    return Err("call requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Call(parts[1].to_string())))
            }
            "ret" => Ok(Some(Instruction::Ret)),
            "syscall" => {
                if parts.len() < 2 {
                    return Err("syscall requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Syscall(parts[1].to_string())))
            }
            "global" => {
                if parts.len() < 2 {
                    return Err("global requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Global(parts[1].to_string())))
            }
            "extern" => {
                if parts.len() < 2 {
                    return Err("extern requires 1 operand".to_string());
                }
                Ok(Some(Instruction::Extern(parts[1].to_string())))
            }
            _ => Err(format!("Unknown instruction: {}", cmd)),
        }
    }

    fn clean_operand(&self, operand: &str) -> String {
        operand.trim_end_matches(',').to_string()
    }
}

struct CodeGenerator {
    register_map: HashMap<String, String>,
}

impl CodeGenerator {
    fn new(architecture: &Architecture) -> Self {
        let mut register_map = HashMap::new();

        match architecture {
            Architecture::AMD64 => {
                register_map.insert("r0".to_string(), "rdi".to_string()); // 1st arg
                register_map.insert("r1".to_string(), "rsi".to_string()); // 2nd arg
                register_map.insert("r2".to_string(), "rdx".to_string()); // 3rd arg
                register_map.insert("r3".to_string(), "r10".to_string()); // 4th arg
                register_map.insert("r4".to_string(), "r8".to_string()); // 5th arg
                register_map.insert("r5".to_string(), "r9".to_string()); // 6th arg

                // General-purpose temporaries (keep some common choices)
                register_map.insert("r6".to_string(), "rax".to_string());
                register_map.insert("r7".to_string(), "rbx".to_string());
                register_map.insert("r8".to_string(), "rcx".to_string());
                register_map.insert("r9".to_string(), "r11".to_string());
                register_map.insert("r10".to_string(), "r12".to_string());
                register_map.insert("r11".to_string(), "r13".to_string());
                register_map.insert("r12".to_string(), "r14".to_string());
                register_map.insert("r13".to_string(), "r15".to_string());

                // Special purpose registers
                register_map.insert("sp".to_string(), "rsp".to_string());
                register_map.insert("sb".to_string(), "rbp".to_string());
                register_map.insert("ip".to_string(), "rip".to_string());
            }
            _ => {
                eprintln!("Error: Only x86_64 architecture is currently implemented");
                process::exit(1);
            }
        }

        CodeGenerator { register_map }
    }
    fn generate(&self, instructions: &[Instruction]) -> String {
        let mut output = String::new();
        output.push_str(".intel_syntax noprefix\n\n");

        for instruction in instructions {
            match instruction {
                Instruction::Section(section) => match section {
                    Section::Text => output.push_str(".section .text\n"),
                    Section::Data => output.push_str(".section .data\n"),
                    Section::Bss => output.push_str(".section .bss\n"),
                    Section::Rodata => output.push_str(".section .rodata\n"),
                },
                Instruction::Label(name) => {
                    output.push_str(&format!("{}:\n", name));
                }
                Instruction::Mov(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    mov {}, {}\n", dst_reg, src_op));
                }
                Instruction::Lea(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_memory_operand(src);
                    output.push_str(&format!("    lea {}, {}\n", dst_reg, src_op));
                }
                Instruction::Load(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_mem = self.map_memory_operand(src);
                    output.push_str(&format!("    mov {}, {}\n", dst_reg, src_mem));
                }
                Instruction::Store(dst, src) => {
                    let dst_mem = self.map_memory_operand(dst);
                    let src_reg = self.map_operand(src);
                    output.push_str(&format!("    mov {}, {}\n", dst_mem, src_reg));
                }
                Instruction::Add(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    add {}, {}\n", dst_reg, src_op));
                }
                Instruction::Sub(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    sub {}, {}\n", dst_reg, src_op));
                }
                Instruction::Mul(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    imul {}, {}\n", dst_reg, src_op));
                }
                Instruction::Div(dst, src) => {
                    // For signed division (idiv) x86-64 expects dividend in RAX and
                    // sign-extension in RDX:RAX (use cqo), then idiv <divisor>.
                    // We'll move the dividend to rax if needed, cqo, idiv, and place
                    // the quotient back into the destination register (unless dest is rax).
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);

                    // If dividend is not already in rax, move it there.
                    if dst_reg != "rax" {
                        output.push_str(&format!("    mov rax, {}\n", dst_reg));
                    }
                    output.push_str("    cqo\n"); // sign-extend rax -> rdx:rax
                    output.push_str(&format!("    idiv {}\n", src_op));
                    // Quotient in rax; move back if dst isn't rax
                    if dst_reg != "rax" {
                        output.push_str(&format!("    mov {}, rax\n", dst_reg));
                    }
                }
                Instruction::Inc(dst) => {
                    let dst_reg = self.map_operand(dst);
                    output.push_str(&format!("    inc {}\n", dst_reg));
                }
                Instruction::Dec(dst) => {
                    let dst_reg = self.map_operand(dst);
                    output.push_str(&format!("    dec {}\n", dst_reg));
                }
                Instruction::Neg(dst) => {
                    let dst_reg = self.map_operand(dst);
                    output.push_str(&format!("    neg {}\n", dst_reg));
                }
                Instruction::And(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    and {}, {}\n", dst_reg, src_op));
                }
                Instruction::Or(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    or {}, {}\n", dst_reg, src_op));
                }
                Instruction::Xor(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    xor {}, {}\n", dst_reg, src_op));
                }
                Instruction::Not(dst) => {
                    let dst_reg = self.map_operand(dst);
                    output.push_str(&format!("    not {}\n", dst_reg));
                }
                Instruction::Shl(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    shl {}, {}\n", dst_reg, src_op));
                }
                Instruction::Shr(dst, src) => {
                    let dst_reg = self.map_operand(dst);
                    let src_op = self.map_operand(src);
                    output.push_str(&format!("    shr {}, {}\n", dst_reg, src_op));
                }
                Instruction::Cmp(op1, op2) => {
                    let op1_reg = self.map_operand(op1);
                    let op2_op = self.map_operand(op2);
                    output.push_str(&format!("    cmp {}, {}\n", op1_reg, op2_op));
                }
                Instruction::Test(op1, op2) => {
                    let op1_reg = self.map_operand(op1);
                    let op2_op = self.map_operand(op2);
                    output.push_str(&format!("    test {}, {}\n", op1_reg, op2_op));
                }
                Instruction::Jmp(label) => {
                    output.push_str(&format!("    jmp {}\n", label));
                }
                Instruction::Je(label) => {
                    output.push_str(&format!("    je {}\n", label));
                }
                Instruction::Jne(label) => {
                    output.push_str(&format!("    jne {}\n", label));
                }
                Instruction::Jg(label) => {
                    output.push_str(&format!("    jg {}\n", label));
                }
                Instruction::Jl(label) => {
                    output.push_str(&format!("    jl {}\n", label));
                }
                Instruction::Jge(label) => {
                    output.push_str(&format!("    jge {}\n", label));
                }
                Instruction::Jle(label) => {
                    output.push_str(&format!("    jle {}\n", label));
                }
                Instruction::Call(func) => {
                    output.push_str(&format!("    call {}\n", func));
                }
                Instruction::Ret => {
                    output.push_str("    ret\n");
                }
                Instruction::Syscall(name) => {
                    let syscall_num = match name.as_str() {
                        "read" => "0",
                        "write" => "1",
                        "exit" => "60",
                        "open" => "2",
                        "close" => "3",
                        _ => "0",
                    };
                    // Make sure the user code has moved args into rdi/rsi/rdx/etc.
                    // Now set syscall number into rax and invoke syscall.
                    output.push_str(&format!("    mov rax, {}\n", syscall_num));
                    output.push_str("    syscall\n");
                }
                Instruction::Global(symbol) => {
                    output.push_str(&format!(".global {}\n", symbol));
                }
                Instruction::Extern(symbol) => {
                    output.push_str(&format!(".extern {}\n", symbol));
                }
                Instruction::DataByte(name, values) => {
                    output.push_str(&format!("{}:\n", name));
                    let mut all_bytes = Vec::new();
                    for val in values.iter() {
                        let formatted = self.format_data_value(val);
                        // If it's a comma-separated list of bytes, split and add each
                        if formatted.contains(", ") {
                            all_bytes.extend(formatted.split(", ").map(|s| s.to_string()));
                        } else {
                            all_bytes.push(formatted);
                        }
                    }
                    output.push_str(&format!("    .byte {}\n", all_bytes.join(", ")));
                }
                Instruction::DataWord(name, values) => {
                    output.push_str(&format!("{}:\n", name));
                    output.push_str("    .word ");
                    for (i, val) in values.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        output.push_str(&self.format_data_value(val));
                    }
                    output.push('\n');
                }
                Instruction::DataDword(name, values) => {
                    output.push_str(&format!("{}:\n", name));
                    output.push_str("    .long ");
                    for (i, val) in values.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        output.push_str(&self.format_data_value(val));
                    }
                    output.push('\n');
                }
                Instruction::DataQword(name, values) => {
                    output.push_str(&format!("{}:\n", name));
                    output.push_str("    .quad ");
                    for (i, val) in values.iter().enumerate() {
                        if i > 0 {
                            output.push_str(", ");
                        }
                        output.push_str(&self.format_data_value(val));
                    }
                    output.push('\n');
                }
                Instruction::ReserveByte(name, size) => {
                    if name != "anonymous" {
                        output.push_str(&format!("{}:\n", name));
                    }
                    output.push_str(&format!("    .space {}\n", size));
                }
                Instruction::Equ(name, value) => {
                    output.push_str(&format!(".equ {}, {}\n", name, value));
                }
            }
        }

        output
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
                format!("[{}]", mapped)
            } else {
                format!("[{}]", inner)
            }
        } else {
            operand.to_string()
        }
    }

    fn format_data_value(&self, value: &str) -> String {
        let trimmed = value.trim();
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            // Handle string literals properly - convert to individual bytes
            let string_content = &trimmed[1..trimmed.len() - 1];
            let mut result = Vec::new();
            let mut chars = string_content.chars();

            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(next_char) = chars.next() {
                        match next_char {
                            'n' => result.push("10".to_string()),  // newline
                            't' => result.push("9".to_string()),   // tab
                            'r' => result.push("13".to_string()),  // carriage return
                            '\\' => result.push("92".to_string()), // backslash
                            '"' => result.push("34".to_string()),  // quote
                            _ => {
                                // unknown escape: emit both bytes of the two characters
                                result.push((c as u8).to_string());
                                result.push((next_char as u8).to_string());
                            }
                        }
                    } else {
                        // trailing backslash: emit its byte
                        result.push((c as u8).to_string());
                    }
                } else {
                    result.push((c as u8).to_string());
                }
            }
            result.join(", ")
        } else if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
            trimmed.to_string()
        } else if trimmed.chars().all(|c| c.is_ascii_digit()) {
            trimmed.to_string()
        } else {
            // For other values like identifiers, return as-is
            trimmed.to_string()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <input.ua> [-o output.s] [-a architecture] [-p platform]",
            args[0]
        );
        process::exit(1);
    }

    let input_file = &args[1];
    let mut output_file = "output.s".to_string();
    let mut architecture = Architecture::AMD64;
    let mut _platform = "linux".to_string();

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: -o requires an output filename");
                    process::exit(1);
                }
            }
            "-a" => {
                if i + 1 < args.len() {
                    architecture = match args[i + 1].as_str() {
                        "x86_64" | "amd64" => Architecture::AMD64,
                        _ => {
                            eprintln!("Error: Only x86_64 architecture is currently supported");
                            process::exit(1);
                        }
                    };
                    i += 2;
                } else {
                    eprintln!("Error: -a requires an architecture");
                    process::exit(1);
                }
            }
            "-p" => {
                if i + 1 < args.len() {
                    match args[i + 1].as_str() {
                        "linux" => _platform = "linux".to_string(),
                        _ => {
                            eprintln!("Error: Only Linux platform is currently supported");
                            process::exit(1);
                        }
                    }
                    i += 2;
                } else {
                    eprintln!("Error: -p requires a platform");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unknown option {}", args[i]);
                process::exit(1);
            }
        }
    }

    let input_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file '{}': {}", input_file, err);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(&input_content);
    let instructions = match parser.parse() {
        Ok(instructions) => instructions,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    };

    let code_generator = CodeGenerator::new(&architecture);
    let asm_code = code_generator.generate(&instructions);

    if let Err(err) = fs::write(&output_file, asm_code) {
        eprintln!("Error writing output file '{}': {}", output_file, err);
        process::exit(1);
    }

    println!(
        "Successfully compiled '{}' to '{}'",
        input_file, output_file
    );
}
