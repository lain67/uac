use super::*;
pub struct WindowsPlatform;

impl PlatformCodeGen for WindowsPlatform {
    fn get_section_prefix(&self, section: &Section) -> String {
        match section {
            Section::Text => ".section .text,\"xr\"\n".to_string(),
            Section::Data => ".section .data,\"rw\"\n".to_string(),
            Section::Bss => ".section .bss,\"rw\"\n".to_string(),
            Section::Rodata => ".section .rdata,\"r\"\n".to_string(),
            Section::Custom(section) => format!(".section .{},\"r\"\n", section),
        }
    }

    fn get_global_directive(&self, symbol: &str) -> String {
        format!(
            ".globl {}\n.def {}; .scl 2; .type 32; .endef\n",
            symbol, symbol
        )
    }

    fn get_extern_directive(&self, symbol: &str) -> String {
        format!(".extern {}\n", symbol)
    }

    fn format_data_directive(&self, size: DataSize, name: &str, values: &[String]) -> String {
        let directive = match size {
            DataSize::Byte => ".byte",
            DataSize::Word => ".word", // windows assembler should handle .word
            DataSize::Dword => ".long",
            DataSize::Qword => ".quad",
        };

        let mut result = String::new();
        match size {
            DataSize::Word => result.push_str(".align 2\n"),
            DataSize::Dword => result.push_str(".align 4\n"),
            DataSize::Qword => result.push_str(".align 8\n"),
            _ => {}
        }

        result.push_str(&format!("{}:\n", name));
        result.push_str(&format!("    {} {}\n", directive, values.join(", ")));
        result
    }

    fn format_reserve_directive(&self, name: &str, size: &String) -> String {
        let mut result = String::new();

        if let Ok(size_val) = size.parse::<usize>() {
            if size_val >= 8 {
                result.push_str(".align 8\n");
            } else if size_val >= 4 {
                result.push_str(".align 4\n");
            } else if size_val >= 2 {
                result.push_str(".align 2\n");
            }
        }

        if name != "anonymous" {
            result.push_str(&format!("{}:\n", name));
        }
        // or .fill, depending on assembler
        result.push_str(&format!("    .space {}\n", size));
        result
    }

    fn format_equ_directive(&self, name: &str, value: &str) -> String {
        format!(".equ {}, {}\n", name, value)
    }
}
