use super::*;

pub struct MacOSPlatform;

impl PlatformCodeGen for MacOSPlatform {
    fn get_section_prefix(&self, section: &Section) -> String {
        match section {
            Section::Text => ".text\n".to_string(),
            Section::Data => ".data\n".to_string(),
            Section::Bss => ".bss\n".to_string(),
            Section::Rodata => ".const\n".to_string(),
        }
    }

    fn get_global_directive(&self, symbol: &str) -> String {
        format!(".globl _{}\n", symbol)
    }

    fn get_extern_directive(&self, symbol: &str) -> String {
        format!(".extern _{}\n", symbol)
    }

    fn format_data_directive(&self, size: DataSize, name: &str, values: &[String]) -> String {
        let directive = match size {
            DataSize::Byte => ".byte",
            DataSize::Word => ".short",
            DataSize::Dword => ".long",
            DataSize::Qword => ".quad",
        };

        let mut result = String::new();

        match size {
            DataSize::Word => result.push_str(".p2align 1\n"),
            DataSize::Dword => result.push_str(".p2align 2\n"),
            DataSize::Qword => result.push_str(".p2align 3\n"),
            _ => {}
        }

        result.push_str(&format!("_{}:\n", name));
        result.push_str(&format!("    {} {}\n", directive, values.join(", ")));
        result
    }

    fn format_reserve_directive(&self, name: &str, size: &String) -> String {
        let mut result = String::new();

        if let Ok(size_val) = size.parse::<usize>() {
            if size_val >= 8 {
                result.push_str(".p2align 3\n");
            } else if size_val >= 4 {
                result.push_str(".p2align 2\n");
            } else if size_val >= 2 {
                result.push_str(".p2align 1\n");
            }
        }

        if name != "anonymous" {
            result.push_str(&format!("_{}:\n", name));
        }
        result.push_str(&format!("    .space {}\n", size));
        result
    }

    fn format_equ_directive(&self, name: &str, value: &str) -> String {
        format!(".set _{}, {}\n", name, value)
    }
}
