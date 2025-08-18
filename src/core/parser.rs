use super::*;

pub struct Parser {
    lines: Vec<String>,
    current_section: Section,
    constants: HashMap<String, String>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
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

    pub fn parse(&mut self) -> Result<Vec<Instruction>, String> {
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
