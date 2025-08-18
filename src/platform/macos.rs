use super::*;
pub struct MacOSPlatform;

impl PlatformCodeGen for MacOSPlatform {
    fn get_section_prefix(&self, section: &Section) -> String {
        match section {
            Section::Text => ".section __TEXT,__text\n".to_string(),
            Section::Data => ".section __DATA,__data\n".to_string(),
            Section::Bss => ".section __DATA,__bss\n".to_string(),
            Section::Rodata => ".section __TEXT,__const\n".to_string(),
        }
    }

    fn get_global_directive(&self, symbol: &str) -> String {
        format!(".globl _{}\n", symbol) // macOS prefixes symbols with underscore
    }

    fn get_extern_directive(&self, symbol: &str) -> String {
        format!(".extern _{}\n", symbol)
    }

    fn format_data_directive(&self, size: DataSize, name: &str, values: &[String]) -> String {
        let directive = match size {
            DataSize::Byte => ".byte",
            DataSize::Word => ".word",
            DataSize::Dword => ".long",
            DataSize::Qword => ".quad",
        };

        let mut result = format!("_{}:\n", name); // macOS symbol prefix
        result.push_str(&format!("    {} {}\n", directive, values.join(", ")));
        result
    }

    fn format_reserve_directive(&self, name: &str, size: &String) -> String {
        let mut result = String::new();
        if name != "anonymous" {
            result.push_str(&format!("_{}:\n", name));
        }
        result.push_str(&format!("    .space {}\n", size));
        result
    }

    fn format_equ_directive(&self, name: &str, value: &str) -> String {
        format!(".equ _{}, {}\n", name, value)
    }
}