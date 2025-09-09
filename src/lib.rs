mod arch;
mod core;
mod platform;
mod abs;

use crate::core::{codegen::CodeGenerator, parser::Parser};

pub use crate::arch::Architecture;
pub use crate::core::TargetTriple;
pub use crate::platform::Platform;

/// Compile UASM into the target architecture, format and platform
pub fn compiler_uasm(uasm: String, target: TargetTriple) -> Result<String, String> {
    let mut parser = Parser::new(&uasm);
    let instructions = parser.parse()?;
    let code_generator = CodeGenerator::new(target);
    let asm_code = code_generator.generate(&instructions);
    Ok(asm_code)
}

/// Compile UASM into Linux on target architecture
pub fn compile_uasm_linux(uasm: String, arch: Architecture) -> Result<String, String> {
    let target = TargetTriple::new(arch, Platform::Linux);
    compiler_uasm(uasm, target)
}

/// Compile UASM into macOS on target architecture
pub fn compile_uasm_mac(uasm: String, arch: Architecture) -> Result<String, String> {
    let target = TargetTriple::new(arch, Platform::MacOS);
    compiler_uasm(uasm, target)
}

/// Compile UASM into Windows on target architecture
pub fn compile_uasm_wind(uasm: String, arch: Architecture) -> Result<String, String> {
    let target = TargetTriple::new(arch, Platform::Windows);
    compiler_uasm(uasm, target)
}
