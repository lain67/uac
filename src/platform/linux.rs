use super::*;

pub struct LinuxPlatform;

impl PlatformCodeGen for LinuxPlatform {
    fn get_section_prefix(&self, section: &Section) -> String {
        match section {
            Section::Text => ".section .text\n".to_string(),
            Section::Data => ".section .data\n".to_string(),
            Section::Bss => ".section .bss\n".to_string(),
            Section::Rodata => ".section .rodata\n".to_string(),
        }
    }

    fn get_global_directive(&self, symbol: &str) -> String {
        format!(".global {}\n", symbol)
    }

    fn get_extern_directive(&self, symbol: &str) -> String {
        format!(".extern {}\n", symbol)
    }

    fn format_data_directive(&self, size: DataSize, name: &str, values: &[String]) -> String {
        let directive = match size {
            DataSize::Byte => ".byte",
            DataSize::Word => ".word",
            DataSize::Dword => ".long",
            DataSize::Qword => ".quad",
        };

        let mut result = format!("{}:\n", name);
        result.push_str(&format!("    {} {}\n", directive, values.join(", ")));
        result
    }

    fn format_reserve_directive(&self, name: &str, size: &String) -> String {
        let mut result = String::new();
        if name != "anonymous" {
            result.push_str(&format!("{}:\n", name));
        }
        result.push_str(&format!("    .space {}\n", size));
        result
    }

    fn format_equ_directive(&self, name: &str, value: &str) -> String {
        format!(".equ {}, {}\n", name, value)
    }
}
