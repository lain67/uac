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

        // Data definitions
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

        // Memory reservations
        if line.contains(" resb ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "resb" {
                let name = parts[0].clone();
                let value = parts[2].clone();
                return Ok(Some(Instruction::ReserveByte(name, value)));
            }
        }

        if line.contains(" resw ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "resw" {
                let name = parts[0].clone();
                let value = parts[2].clone();
                return Ok(Some(Instruction::ReserveWord(name, value)));
            }
        }

        if line.contains(" resd ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "resd" {
                let name = parts[0].clone();
                let value = parts[2].clone();
                return Ok(Some(Instruction::ReserveDword(name, value)));
            }
        }

        if line.contains(" resq ") {
            let parts = self.parse_data_line(line);
            if parts.len() >= 3 && parts[1] == "resq" {
                let name = parts[0].clone();
                let value = parts[2].clone();
                return Ok(Some(Instruction::ReserveQword(name, value)));
            }
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }

        // Constants
        if parts.len() >= 3 && parts[1] == "equ" {
            let name = parts[0].to_string();
            let value = parts[2].to_string();
            self.constants.insert(name.clone(), value.clone());
            return Ok(Some(Instruction::Equ(name, value)));
        }

        let cmd = parts[0];
        match cmd {
            // Data Movement
            "mov" => Ok(Some(Instruction::Mov(self.get_two(&parts)?))),
            "lea" => Ok(Some(Instruction::Lea(self.get_two(&parts)?))),
            "load" => Ok(Some(Instruction::Load(self.get_two(&parts)?))),
            "store" => Ok(Some(Instruction::Store(self.get_two(&parts)?))),
            
            // Conditional Moves
            "cmoveq" | "cmovz" => Ok(Some(Instruction::CmovEq(self.get_two(&parts)?))),
            "cmovne" | "cmovnz" => Ok(Some(Instruction::CmovNe(self.get_two(&parts)?))),
            "cmovlt" | "cmovl" => Ok(Some(Instruction::CmovLt(self.get_two(&parts)?))),
            "cmovle" => Ok(Some(Instruction::CmovLe(self.get_two(&parts)?))),
            "cmovgt" | "cmovg" => Ok(Some(Instruction::CmovGt(self.get_two(&parts)?))),
            "cmovge" => Ok(Some(Instruction::CmovGe(self.get_two(&parts)?))),
            "cmovov" | "cmovo" => Ok(Some(Instruction::CmovOv(self.get_two(&parts)?))),
            "cmovno" => Ok(Some(Instruction::CmovNo(self.get_two(&parts)?))),
            "cmovs" => Ok(Some(Instruction::CmovS(self.get_two(&parts)?))),
            "cmovns" => Ok(Some(Instruction::CmovNs(self.get_two(&parts)?))),
            "cmovp" | "cmovpe" => Ok(Some(Instruction::CmovP(self.get_two(&parts)?))),
            "cmovnp" | "cmovpo" => Ok(Some(Instruction::CmovNp(self.get_two(&parts)?))),
            "cmova" | "cmovnbe" => Ok(Some(Instruction::CmovA(self.get_two(&parts)?))),
            "cmovae" | "cmovnb" | "cmovnc" => Ok(Some(Instruction::CmovAe(self.get_two(&parts)?))),
            "cmovb" | "cmovc" | "cmovnae" => Ok(Some(Instruction::CmovB(self.get_two(&parts)?))),
            "cmovbe" | "cmovna" => Ok(Some(Instruction::CmovBe(self.get_two(&parts)?))),
            
            // Stack Operations
            "push" => Ok(Some(Instruction::Push(self.get_one(&parts)?))),
            "pop" => Ok(Some(Instruction::Pop(self.get_one(&parts)?))),
            "pusha" | "pushad" => Ok(Some(Instruction::Pusha)),
            "popa" | "popad" => Ok(Some(Instruction::Popa)),
            "enter" => Ok(Some(Instruction::Enter(self.get_two(&parts)?))),
            "leave" => Ok(Some(Instruction::Leave)),
            
            // Arithmetic Operations
            "add" => Ok(Some(Instruction::Add(self.get_two(&parts)?))),
            "sub" => Ok(Some(Instruction::Sub(self.get_two(&parts)?))),
            "mul" => Ok(Some(Instruction::Mul(self.get_two(&parts)?))),
            "imul" => Ok(Some(Instruction::Imul(self.get_two(&parts)?))),
            "div" => Ok(Some(Instruction::Div(self.get_two(&parts)?))),
            "idiv" => Ok(Some(Instruction::Idiv(self.get_two(&parts)?))),
            "mod" => Ok(Some(Instruction::Mod(self.get_two(&parts)?))),
            "inc" => Ok(Some(Instruction::Inc(self.get_one(&parts)?))),
            "dec" => Ok(Some(Instruction::Dec(self.get_one(&parts)?))),
            "neg" => Ok(Some(Instruction::Neg(self.get_one(&parts)?))),
            
            // Logical & Bitwise Operations
            "and" => Ok(Some(Instruction::And(self.get_two(&parts)?))),
            "or" => Ok(Some(Instruction::Or(self.get_two(&parts)?))),
            "xor" => Ok(Some(Instruction::Xor(self.get_two(&parts)?))),
            "not" => Ok(Some(Instruction::Not(self.get_one(&parts)?))),
            "andn" => Ok(Some(Instruction::Andn(self.get_two(&parts)?))),
            "shl" | "sal" => Ok(Some(Instruction::Shl(self.get_two(&parts)?))),
            "shr" => Ok(Some(Instruction::Shr(self.get_two(&parts)?))),
            "sar" => Ok(Some(Instruction::Sar(self.get_two(&parts)?))),
            "rol" => Ok(Some(Instruction::Rol(self.get_two(&parts)?))),
            "ror" => Ok(Some(Instruction::Ror(self.get_two(&parts)?))),
            "rcl" => Ok(Some(Instruction::Rcl(self.get_two(&parts)?))),
            "rcr" => Ok(Some(Instruction::Rcr(self.get_two(&parts)?))),
            "bextr" => Ok(Some(Instruction::Bextr(self.get_three(&parts)?))),
            "bsf" => Ok(Some(Instruction::Bsf(self.get_two(&parts)?))),
            "bsr" => Ok(Some(Instruction::Bsr(self.get_two(&parts)?))),
            
            // Comparison & Conditional Sets
            "cmp" => Ok(Some(Instruction::Cmp(self.get_two(&parts)?))),
            "test" => Ok(Some(Instruction::Test(self.get_two(&parts)?))),
            "bt" => Ok(Some(Instruction::Bt(self.get_two(&parts)?))),
            "btr" => Ok(Some(Instruction::Btr(self.get_two(&parts)?))),
            "bts" => Ok(Some(Instruction::Bts(self.get_two(&parts)?))),
            "btc" => Ok(Some(Instruction::Btc(self.get_two(&parts)?))),
            "seteq" | "setz" => Ok(Some(Instruction::SetEq(self.get_one(&parts)?))),
            "setne" | "setnz" => Ok(Some(Instruction::SetNe(self.get_one(&parts)?))),
            "setlt" | "setl" => Ok(Some(Instruction::SetLt(self.get_one(&parts)?))),
            "setle" => Ok(Some(Instruction::SetLe(self.get_one(&parts)?))),
            "setgt" | "setg" => Ok(Some(Instruction::SetGt(self.get_one(&parts)?))),
            "setge" => Ok(Some(Instruction::SetGe(self.get_one(&parts)?))),
            "setov" | "seto" => Ok(Some(Instruction::SetOv(self.get_one(&parts)?))),
            "setno" => Ok(Some(Instruction::SetNo(self.get_one(&parts)?))),
            "sets" => Ok(Some(Instruction::SetS(self.get_one(&parts)?))),
            "setns" => Ok(Some(Instruction::SetNs(self.get_one(&parts)?))),
            "setp" | "setpe" => Ok(Some(Instruction::SetP(self.get_one(&parts)?))),
            "setnp" | "setpo" => Ok(Some(Instruction::SetNp(self.get_one(&parts)?))),
            "seta" | "setnbe" => Ok(Some(Instruction::SetA(self.get_one(&parts)?))),
            "setae" | "setnb" | "setnc" => Ok(Some(Instruction::SetAe(self.get_one(&parts)?))),
            "setb" | "setc" | "setnae" => Ok(Some(Instruction::SetB(self.get_one(&parts)?))),
            "setbe" | "setna" => Ok(Some(Instruction::SetBe(self.get_one(&parts)?))),
            
            // String Operations
            "cmps" | "cmpsb" | "cmpsw" | "cmpsd" | "cmpsq" => Ok(Some(Instruction::Cmps(self.get_two(&parts)?))),
            "scas" | "scasb" | "scasw" | "scasd" | "scasq" => Ok(Some(Instruction::Scas(self.get_two(&parts)?))),
            "stos" | "stosb" | "stosw" | "stosd" | "stosq" => Ok(Some(Instruction::Stos(self.get_two(&parts)?))),
            "lods" | "lodsb" | "lodsw" | "lodsd" | "lodsq" => Ok(Some(Instruction::Lods(self.get_two(&parts)?))),
            "movs" | "movsb" | "movsw" | "movsd" | "movsq" => Ok(Some(Instruction::Movs(self.get_two(&parts)?))),
            
            // Data Conversion
            "cbw" => Ok(Some(Instruction::Cbw(self.get_one(&parts)?))),
            "cwd" => Ok(Some(Instruction::Cwd(self.get_one(&parts)?))),
            "cdq" => Ok(Some(Instruction::Cdq(self.get_one(&parts)?))),
            "cqo" => Ok(Some(Instruction::Cqo(self.get_one(&parts)?))),
            "cwde" => Ok(Some(Instruction::Cwde(self.get_one(&parts)?))),
            "cdqe" => Ok(Some(Instruction::Cdqe(self.get_one(&parts)?))),
            
            // Control Flow
            "jmp" => Ok(Some(Instruction::Jmp(self.get_one(&parts)?))),
            "je" | "jz" => Ok(Some(Instruction::Je(self.get_one(&parts)?))),
            "jne" | "jnz" => Ok(Some(Instruction::Jne(self.get_one(&parts)?))),
            "jl" | "jnge" => Ok(Some(Instruction::Jl(self.get_one(&parts)?))),
            "jle" | "jng" => Ok(Some(Instruction::Jle(self.get_one(&parts)?))),
            "jg" | "jnle" => Ok(Some(Instruction::Jg(self.get_one(&parts)?))),
            "jge" | "jnl" => Ok(Some(Instruction::Jge(self.get_one(&parts)?))),
            "jo" => Ok(Some(Instruction::Jo(self.get_one(&parts)?))),
            "jno" => Ok(Some(Instruction::Jno(self.get_one(&parts)?))),
            "js" => Ok(Some(Instruction::Js(self.get_one(&parts)?))),
            "jns" => Ok(Some(Instruction::Jns(self.get_one(&parts)?))),
            "jp" | "jpe" => Ok(Some(Instruction::Jp(self.get_one(&parts)?))),
            "jnp" | "jpo" => Ok(Some(Instruction::Jnp(self.get_one(&parts)?))),
            "ja" | "jnbe" => Ok(Some(Instruction::Ja(self.get_one(&parts)?))),
            "jae" | "jnb" | "jnc" => Ok(Some(Instruction::Jae(self.get_one(&parts)?))),
            "jb" | "jc" | "jnae" => Ok(Some(Instruction::Jb(self.get_one(&parts)?))),
            "jbe" | "jna" => Ok(Some(Instruction::Jbe(self.get_one(&parts)?))),
            "loopeq" | "loopz" => Ok(Some(Instruction::LoopEq(self.get_one(&parts)?))),
            "loopne" | "loopnz" => Ok(Some(Instruction::LoopNe(self.get_one(&parts)?))),
            "call" => Ok(Some(Instruction::Call(self.get_one(&parts)?))),
            "ret" | "retn" => Ok(Some(Instruction::Ret)),
            
            // I/O Operations
            "in" => Ok(Some(Instruction::In(self.get_two(&parts)?))),
            "out" => Ok(Some(Instruction::Out(self.get_two(&parts)?))),
            "ins" | "insb" | "insw" | "insd" => Ok(Some(Instruction::Ins(self.get_two(&parts)?))),
            "outs" | "outsb" | "outsw" | "outsd" => Ok(Some(Instruction::Outs(self.get_two(&parts)?))),
            
            // System & CPU Operations
            "cpuid" => Ok(Some(Instruction::Cpuid)),
            "lfence" => Ok(Some(Instruction::Lfence)),
            "sfence" => Ok(Some(Instruction::Sfence)),
            "mfence" => Ok(Some(Instruction::Mfence)),
            "prefetch" | "prefetcht0" | "prefetcht1" | "prefetcht2" | "prefetchnta" => {
                Ok(Some(Instruction::Prefetch(self.get_one(&parts)?)))
            }
            "clflush" => Ok(Some(Instruction::Clflush(self.get_one(&parts)?))),
            "clwb" => Ok(Some(Instruction::Clwb(self.get_one(&parts)?))),
            
            // System Calls
            "syscall" => Ok(Some(Instruction::Syscall(self.get_one(&parts)?))),
            
            // Directives
            "global" => Ok(Some(Instruction::Global(self.get_one(&parts)?))),
            "extern" => Ok(Some(Instruction::Extern(self.get_one(&parts)?))),
            "align" => Ok(Some(Instruction::Align(self.get_one(&parts)?))),
            
            _ => Err(format!("Unknown instruction: {}", cmd)),
        }
    }

    fn clean_operand(&self, operand: &str) -> String {
        operand.trim_end_matches(',').to_string()
    }

    fn check_parts(&self, size: usize, parts: &Vec<&str>) -> Result<(), String> {
        if parts.len() < size {
            return Err(format!("{} requires {} operands", parts[0], size - 1));
        }
        Ok(())
    }

    fn get_two(&self, parts: &Vec<&str>) -> Result<(String, String), String> {
        self.check_parts(3, &parts)?;
        let dst = self.clean_operand(parts[1]);
        let src = self.clean_operand(parts[2]);
        Ok((dst, src))
    }

    fn get_three(&self, parts: &Vec<&str>) -> Result<(String, String, String), String> {
        self.check_parts(4, &parts)?;
        let first = self.clean_operand(parts[1]);
        let second = self.clean_operand(parts[2]);
        let third = self.clean_operand(parts[3]);
        Ok((first, second, third))
    }

    fn get_one(&self, parts: &Vec<&str>) -> Result<String, String> {
        self.check_parts(2, &parts)?;
        Ok(self.clean_operand(parts[1]))
    }
}