pub mod linux;
pub mod macos;
pub mod windows;

use crate::{
    arch::Architecture,
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
    MZ,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Linux,
    Windows,
    MacOS,
    BSD,
    Solaris,
    DOS,
    Embedded,
}

pub trait PlatformCodeGen {
    fn get_section_prefix(&self, section: &Section) -> String;
    fn get_global_directive(&self, symbol: &str) -> String;
    fn get_extern_directive(&self, symbol: &str) -> String;
    fn format_data_directive(&self, size: DataSize, name: &str, values: &[String]) -> String;
    fn format_reserve_directive(&self, name: &str, size: &String) -> String;
    fn format_equ_directive(&self, name: &str, value: &str) -> String;
    fn set_architecture(&mut self, arch: Architecture);
}

pub fn create_platform_codegen(
    platform: &Platform,
    arch: &Architecture,
) -> Box<dyn PlatformCodeGen> {
    match platform {
        Platform::Linux => {
            let mut linux_platform = LinuxPlatform::new();
            linux_platform.set_architecture(*arch);
            Box::new(linux_platform)
        }
        Platform::Windows => {
            let mut windows_platform = WindowsPlatform::new();
            windows_platform.set_architecture(*arch);
            Box::new(windows_platform)
        }
        Platform::MacOS => {
            let mut macos_platform = MacOSPlatform::new();
            macos_platform.set_architecture(*arch);
            Box::new(macos_platform)
        }
        _ => {
            eprintln!(
                "Error: Platform {:?} is not currently implemented",
                platform
            );
            std::process::exit(1);
        }
    }
}
