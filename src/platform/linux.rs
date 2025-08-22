use super::*;
use crate::arch::Architecture;

pub struct LinuxPlatform {
    architecture: Architecture,
}

impl LinuxPlatform {
    pub fn new() -> Self {
        LinuxPlatform {
            architecture: Architecture::AMD64, // default
        }
    }
}

impl PlatformCodeGen for LinuxPlatform {
    fn get_section_prefix(&self, section: &Section) -> String {
        let progbits_suffix = match self.architecture {
            Architecture::ARM32 => "%progbits",
            _ => "@progbits",
        };
        let nobits_suffix = match self.architecture {
            Architecture::ARM32 => "%nobits",
            _ => "@nobits",
        };

        match section {
            Section::Text => format!(".section .text,\"ax\",{}\n", progbits_suffix),
            Section::Data => format!(".section .data,\"aw\",{}\n", progbits_suffix),
            Section::Bss => format!(".section .bss,\"aw\",{}\n", nobits_suffix),
            Section::Rodata => format!(".section .rodata,\"a\",{}\n", progbits_suffix),
            Section::Custom(section) => {
                format!(".section .{},\"a\",{}\n", section, progbits_suffix)
            }
        }
    }

    fn get_global_directive(&self, symbol: &str) -> String {
        let function_suffix = match self.architecture {
            Architecture::ARM32 => "%function",
            _ => "@function",
        };
        format!(".globl {}\n.type {}, {}\n", symbol, symbol, function_suffix)
    }

    fn get_extern_directive(&self, symbol: &str) -> String {
        format!(".extern {}\n", symbol)
    }

    fn format_data_directive(&self, size: DataSize, name: &str, values: &[String]) -> String {
        let directive = match size {
            DataSize::Byte => ".byte",
            DataSize::Word => {
                if self.architecture == Architecture::ARM32 {
                    ".hword"
                } else {
                    ".2byte"
                }
            }
            DataSize::Dword => {
                if self.architecture == Architecture::ARM32 {
                    ".word"
                } else {
                    ".4byte"
                }
            }
            DataSize::Qword => {
                if self.architecture == Architecture::ARM32 {
                    ".quad"
                } else {
                    ".8byte"
                }
            }
        };

        let object_suffix = match self.architecture {
            Architecture::ARM32 => "%object",
            _ => "@object",
        };

        let mut result = String::new();
        match size {
            DataSize::Word => result.push_str(".align 2\n"),
            DataSize::Dword => result.push_str(".align 4\n"),
            DataSize::Qword => {
                if self.architecture != Architecture::ARM32 {
                    result.push_str(".align 8\n")
                }
            }
            _ => {}
        }

        result.push_str(&format!("{}:\n", name));
        result.push_str(&format!(".type {}, {}\n", name, object_suffix));
        result.push_str(&format!("    {} {}\n", directive, values.join(", ")));
        result.push_str(&format!(".size {}, .-{}\n", name, name));
        result
    }

    fn format_reserve_directive(&self, name: &str, size: &String) -> String {
        let mut result = String::new();
        let object_suffix = match self.architecture {
            Architecture::ARM32 => "%object",
            _ => "@object",
        };

        if let Ok(size_val) = size.parse::<usize>() {
            if size_val >= 8 && self.architecture != Architecture::ARM32 {
                result.push_str(".align 8\n");
            } else if size_val >= 4 {
                result.push_str(".align 4\n");
            } else if size_val >= 2 {
                result.push_str(".align 2\n");
            }
        }

        if name != "anonymous" {
            result.push_str(&format!("{}:\n", name));
            result.push_str(&format!(".type {}, {}\n", name, object_suffix));
        }
        result.push_str(&format!("    .space {}\n", size));
        if name != "anonymous" {
            result.push_str(&format!(".size {}, {}\n", name, size));
        }
        result
    }

    fn format_equ_directive(&self, name: &str, value: &str) -> String {
        format!(".set {}, {}\n", name, value)
    }

    fn set_architecture(&mut self, arch: Architecture) {
        self.architecture = arch;
    }
}
