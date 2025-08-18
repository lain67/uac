pub mod linux;
pub mod macos;
pub mod windows;

use crate::{
    core::{DataSize, Section},
    platform::{linux::LinuxPlatform, macos::MacOSPlatform, windows::WindowsPlatform},
};

#[derive(Debug, Clone)]
pub enum Format {
    ELF,
    COFF,
    MachO,
    XCOFF,
    A,
    Custom,
}

#[derive(Debug, Clone)]
pub enum Platform {
    Linux,
    Windows,
    MacOS,
    BSD,
    Solaris,
}

pub trait PlatformCodeGen {
    fn get_section_prefix(&self, section: &Section) -> String;
    fn get_global_directive(&self, symbol: &str) -> String;
    fn get_extern_directive(&self, symbol: &str) -> String;
    fn format_data_directive(&self, size: DataSize, name: &str, values: &[String]) -> String;
    fn format_reserve_directive(&self, name: &str, size: &String) -> String;
    fn format_equ_directive(&self, name: &str, value: &str) -> String;
}

pub fn create_platform_codegen(platform: &Platform) -> Box<dyn PlatformCodeGen> {
    match platform {
        Platform::Linux => Box::new(LinuxPlatform),
        Platform::Windows => Box::new(WindowsPlatform),
        Platform::MacOS => Box::new(MacOSPlatform),
        _ => {
            eprintln!(
                "Error: Platform {:?} is not currently implemented",
                platform
            );
            std::process::exit(1);
        }
    }
}
