use std::{collections::HashMap, process};

use crate::{
    arch::{
        amd32::AMD32CodeGen, amd64::AMD64CodeGen, arm32::ARM32CodeGen, arm64::ARM64CodeGen,
        risc_v::RISCVCodeGen,
    },
    core::{Section, TargetTriple},
    platform::Platform,
};

pub mod amd32;
pub mod amd64;
pub mod arm32;
pub mod arm64;
pub mod powerpc64;
pub mod risc_v;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    /// Aliases: x86-64, x64, amd, amd64, intel64
    ///
    /// Supported on:
    /// - BSDs (DragonFlyBSD, FreeBSD, NetBSD, OpenBSD)
    /// - DOS
    /// - Linux  (>=2.4)
    /// - macOS (>= 10.6)
    /// - Solaris (>=10)
    /// - Windows (>= XP Pro x86 ED, Windows Server 2003 )
    AMD64,

    /// Aliases: arm, arm64, aarch64, armv8-a, armv8
    ///
    /// Supported on:
    /// - Linux (>=3.7)
    /// - Android (>=5.0)
    /// - iOS (>=7, all modern iPhones/iPads)
    /// - macOS (>=11.0, Apple Silicon)
    /// - Windows (>=10, ARM edition)
    ARM64,

    /// Aliases: riscv64, riscv, riscv64gc
    ///
    /// Supported on:
    /// - Linux (>=4.15 mainline)
    /// - BSDs (partial/experimental)
    /// - Bare-metal/embedded
    RISCV,

    /// Aliases: ppc64, powerpc64, ppc64le
    ///
    /// Supported on:
    /// - Linux (>=2.6, especially on IBM POWER servers)
    /// - AIX
    /// - BSDs (some support)
    /// - macOS (<=10.5, legacy only, 32-bit/64-bit split)
    PowerPC64,

    /// Aliases: x86, i386, ia-32, 32-bit x86
    ///
    /// Supported on:
    /// - Linux (>=1.0)
    /// - BSDs
    /// - DOS
    /// - Windows (>=3.1)
    /// - macOS (<=10.6, legacy only)
    AMD32,

    /// Aliases: arm, armv7, armhf, arm32
    ///
    /// Supported on:
    /// - Linux (>=2.6)
    /// - Android (<=4.x era devices)
    /// - iOS (<=10, legacy 32-bit iPhones/iPads)
    /// - Embedded devices
    ARM32,

    /// Aliases: mips, mips32, mipsel, mips32r2
    ///
    /// Supported on:
    /// - Linux (>=2.4, especially in routers, embedded devices)
    /// - BSDs (partial)
    /// - Some proprietary embedded OSes
    MIPS32,

    /// Aliases: sparc64, ultraSPARC
    ///
    /// Supported on:
    /// - Solaris (>=2.6)
    /// - OpenBSD, NetBSD (SPARC64 ports)
    /// - Linux (some server/legacy support)
    SPARC64,

    /// Aliases: ia-64, itanium
    ///
    /// Supported on:
    /// - Linux (>=2.4, discontinued in >=5.19)
    /// - Windows Server (2003–2008, Itanium editions only)
    /// - HP-UX
    IA64,

    /// Aliases: alpha, decalpha
    ///
    /// Supported on:
    /// - Linux (2.0–4.x, dropped upstream)
    /// - BSDs (some legacy ports)
    /// - VMS (Digital/Compaq/HP)
    Alpha,

    /// Aliases: hppa, pa-risc
    ///
    /// Supported on:
    /// - HP-UX
    /// - Linux (>=2.6, legacy support only)
    /// - NetBSD/OpenBSD (PA-RISC ports)
    HPPA,

    /// Aliases: k68000, k68k (Motorola 68000 family)
    ///
    /// Supported on:
    /// - Classic Mac OS (pre-OS X)
    /// - AmigaOS
    /// - Linux (m68k port)
    /// - Embedded/retro systems
    K68,

    /// Aliases: avr, atmega, arduino
    ///
    /// Supported on:
    /// - Bare-metal (AVR microcontrollers, Arduino)
    /// - Some RTOS targets
    AVR,

    /// Aliases: msp430
    ///
    /// Supported on:
    /// - Bare-metal (TI MSP430 microcontrollers)
    /// - RTOS in embedded applications
    MSP430,

    /// Aliases: superh, sh2, sh3, sh4
    ///
    /// Supported on:
    /// - Sega Dreamcast, Saturn
    /// - Linux (SuperH port, legacy)
    /// - RTOS in embedded devices
    SH,

    /// Aliases: vax
    ///
    /// Supported on:
    /// - OpenVMS (VAX era)
    /// - BSDs (historical ports)
    /// - NetBSD (still maintained for VAX)
    VAX,

    /// Aliases: nios2, niosii
    ///
    /// Supported on:
    /// - Embedded Linux
    /// - Bare-metal (Altera/Intel FPGA soft-core CPUs)
    NIOSII,

    /// Aliases: xtensa
    ///
    /// Supported on:
    /// - Linux (ESP32 targets, limited)
    /// - Bare-metal/RTOS (ESP-IDF, FreeRTOS)
    Xtensa,

    /// Aliases: arc, arc32, archs
    ///
    /// Supported on:
    /// - Linux (>=3.9, ARC processors)
    /// - Embedded RTOS
    ARC,

    /// Aliases: z80, zilog80
    ///
    /// Supported on:
    /// - CP/M
    /// - Retro/hobbyist OSes
    /// - Embedded (calculators, 8-bit systems)
    Z80,
}

pub trait ArchCodeGen {
    fn get_register_map(&self) -> HashMap<String, String>;
    fn get_syntax_header(&self) -> String;

    //
    // Data Movement
    //
    fn generate_mov(&self, dst: &str, src: &str) -> String;
    fn generate_lea(&self, dst: &str, src: &str) -> String;
    fn generate_load(&self, dst: &str, src: &str) -> String;
    fn generate_store(&self, dst: &str, src: &str) -> String;

    //
    // Conditional Moves
    //
    fn generate_cmov_eq(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_ne(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_lt(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_le(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_gt(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_ge(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_ov(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_no(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_s(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_ns(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_p(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_np(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_a(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_ae(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_b(&self, dst: &str, src: &str) -> String;
    fn generate_cmov_be(&self, dst: &str, src: &str) -> String;

    //
    // Stack Operations
    //
    fn generate_push(&self, src: &str) -> String;
    fn generate_pop(&self, dst: &str) -> String;
    fn generate_pusha(&self) -> String;
    fn generate_popa(&self) -> String;
    fn generate_enter(&self, frame_size: &str, nesting_level: &str) -> String;
    fn generate_leave(&self) -> String;

    //
    // Arithmetic Operations
    //
    fn generate_add(&self, dst: &str, src: &str) -> String;
    fn generate_sub(&self, dst: &str, src: &str) -> String;
    fn generate_mul(&self, dst: &str, src: &str) -> String;
    fn generate_imul(&self, dst: &str, src: &str) -> String;
    fn generate_div(&self, dst: &str, src: &str) -> String;
    fn generate_idiv(&self, dst: &str, src: &str) -> String;
    fn generate_mod(&self, dst: &str, src: &str) -> String;
    fn generate_inc(&self, dst: &str) -> String;
    fn generate_dec(&self, dst: &str) -> String;
    fn generate_neg(&self, dst: &str) -> String;

    //
    // Logical & Bitwise Operations
    //
    fn generate_and(&self, dst: &str, src: &str) -> String;
    fn generate_or(&self, dst: &str, src: &str) -> String;
    fn generate_xor(&self, dst: &str, src: &str) -> String;
    fn generate_not(&self, dst: &str) -> String;
    fn generate_andn(&self, dst: &str, src: &str) -> String;
    fn generate_shl(&self, dst: &str, src: &str) -> String;
    fn generate_shr(&self, dst: &str, src: &str) -> String;
    fn generate_sal(&self, dst: &str, src: &str) -> String;
    fn generate_sar(&self, dst: &str, src: &str) -> String;
    fn generate_rol(&self, dst: &str, src: &str) -> String;
    fn generate_ror(&self, dst: &str, src: &str) -> String;
    fn generate_rcl(&self, dst: &str, src: &str) -> String;
    fn generate_rcr(&self, dst: &str, src: &str) -> String;
    fn generate_bextr(&self, dst: &str, src: &str, imm: &str) -> String;
    fn generate_bsf(&self, dst: &str, src: &str) -> String;
    fn generate_bsr(&self, dst: &str, src: &str) -> String;

    //
    // Comparison & Conditional Sets
    //
    fn generate_cmp(&self, op1: &str, op2: &str) -> String;
    fn generate_test(&self, op1: &str, op2: &str) -> String;
    fn generate_bt(&self, dst: &str, bit: &str) -> String;
    fn generate_btr(&self, dst: &str, bit: &str) -> String;
    fn generate_bts(&self, dst: &str, bit: &str) -> String;
    fn generate_btc(&self, dst: &str, bit: &str) -> String;
    fn generate_set_eq(&self, dst: &str) -> String;
    fn generate_set_ne(&self, dst: &str) -> String;
    fn generate_set_lt(&self, dst: &str) -> String;
    fn generate_set_le(&self, dst: &str) -> String;
    fn generate_set_gt(&self, dst: &str) -> String;
    fn generate_set_ge(&self, dst: &str) -> String;
    fn generate_set_ov(&self, dst: &str) -> String;
    fn generate_set_no(&self, dst: &str) -> String;
    fn generate_set_s(&self, dst: &str) -> String;
    fn generate_set_ns(&self, dst: &str) -> String;
    fn generate_set_p(&self, dst: &str) -> String;
    fn generate_set_np(&self, dst: &str) -> String;
    fn generate_set_a(&self, dst: &str) -> String;
    fn generate_set_ae(&self, dst: &str) -> String;
    fn generate_set_b(&self, dst: &str) -> String;
    fn generate_set_be(&self, dst: &str) -> String;

    //
    // String Operations
    //
    fn generate_cmps(&self, src1: &str, src2: &str) -> String;
    fn generate_scas(&self, src: &str, val: &str) -> String;
    fn generate_stos(&self, dst: &str, src: &str) -> String;
    fn generate_lods(&self, dst: &str, src: &str) -> String;
    fn generate_movs(&self, dst: &str, src: &str) -> String;

    //
    // Data Conversion
    //
    fn generate_cbw(&self, dst: &str) -> String;
    fn generate_cwd(&self, dst: &str) -> String;
    fn generate_cdq(&self, dst: &str) -> String;
    fn generate_cqo(&self, dst: &str) -> String;
    fn generate_cwde(&self, dst: &str) -> String;
    fn generate_cdqe(&self, dst: &str) -> String;

    //
    // Control Flow
    //
    fn generate_jmp(&self, label: &str) -> String;
    fn generate_je(&self, label: &str) -> String;
    fn generate_jne(&self, label: &str) -> String;
    fn generate_jl(&self, label: &str) -> String;
    fn generate_jle(&self, label: &str) -> String;
    fn generate_jg(&self, label: &str) -> String;
    fn generate_jge(&self, label: &str) -> String;
    fn generate_jo(&self, label: &str) -> String;
    fn generate_jno(&self, label: &str) -> String;
    fn generate_js(&self, label: &str) -> String;
    fn generate_jns(&self, label: &str) -> String;
    fn generate_jp(&self, label: &str) -> String;
    fn generate_jnp(&self, label: &str) -> String;
    fn generate_ja(&self, label: &str) -> String;
    fn generate_jae(&self, label: &str) -> String;
    fn generate_jb(&self, label: &str) -> String;
    fn generate_jbe(&self, label: &str) -> String;
    fn generate_loop_eq(&self, label: &str) -> String;
    fn generate_loop_ne(&self, label: &str) -> String;
    fn generate_call(&self, func: &str) -> String;
    fn generate_ret(&self) -> String;

    //
    // I/O Operations
    //
    fn generate_in(&self, dst: &str, port: &str) -> String;
    fn generate_out(&self, port: &str, src: &str) -> String;
    fn generate_ins(&self, dst: &str, port: &str) -> String;
    fn generate_outs(&self, port: &str, src: &str) -> String;

    //
    // System & CPU Operations
    //
    fn generate_cpuid(&self) -> String;
    fn generate_lfence(&self) -> String;
    fn generate_sfence(&self) -> String;
    fn generate_mfence(&self) -> String;
    fn generate_prefetch(&self, addr: &str) -> String;
    fn generate_clflush(&self, addr: &str) -> String;
    fn generate_clwb(&self, addr: &str) -> String;

    //
    // System Calls
    //
    fn generate_syscall(&self, name: &str) -> String;

    //
    // Directives
    //
    fn generate_global(&self, symbol: &str) -> String;
    fn generate_extern(&self, symbol: &str) -> String;
    fn generate_align(&self, n: &str) -> String;

    //
    // Data Definition
    //
    fn generate_data_byte(&self, name: &str, values: &[String]) -> String;
    fn generate_data_word(&self, name: &str, values: &[String]) -> String;
    fn generate_data_dword(&self, name: &str, values: &[String]) -> String;
    fn generate_data_qword(&self, name: &str, values: &[String]) -> String;

    //
    // Memory Reservation
    //
    fn generate_reserve_byte(&self, name: &str, count: &str) -> String;
    fn generate_reserve_word(&self, name: &str, count: &str) -> String;
    fn generate_reserve_dword(&self, name: &str, count: &str) -> String;
    fn generate_reserve_qword(&self, name: &str, count: &str) -> String;

    //
    // Constants and Sections
    //
    fn generate_equ(&self, name: &str, value: &str) -> String;
    fn generate_section(&self, section: &Section) -> String;
    fn generate_label(&self, name: &str) -> String;

    //
    // Utility Methods
    //
    fn map_operand(&self, operand: &str) -> String;
    fn map_memory_operand(&self, operand: &str) -> String;
}

pub fn create_arch_codegen(architecture: &Architecture) -> Box<dyn ArchCodeGen> {
    match architecture {
        Architecture::AMD64 => Box::new(AMD64CodeGen::new()),
        Architecture::AMD32 => Box::new(AMD32CodeGen::new()),
        Architecture::ARM64 => Box::new(ARM64CodeGen::new()),
        Architecture::ARM32 => Box::new(ARM32CodeGen::new()),
        Architecture::RISCV => Box::new(RISCVCodeGen::new()),
        // Architecture::PowerPC64 => Box::new(PowerPC64CodeGen::new()),
        _ => {
            eprintln!(
                "Error: Architecture {:?} is not currently implemented",
                architecture
            );
            process::exit(1);
        }
    }
}

struct ArchInfo {
    aliases: &'static [&'static str],
    supported: &'static [Platform],
}

fn arch_db() -> HashMap<Architecture, ArchInfo> {
    use crate::platform::Platform::*;
    use Architecture::*;

    HashMap::from([
        (
            AMD64,
            ArchInfo {
                aliases: &["x86_64", "amd64", "amd", "x64", "intel64"],
                supported: &[Linux, Windows, MacOS, BSD, Solaris, DOS],
            },
        ),
        (
            ARM64,
            ArchInfo {
                aliases: &["arm64", "aarch64", "arm", "armv8_a", "armv8"],
                supported: &[Linux, Windows, MacOS, Embedded],
            },
        ),
        (
            RISCV,
            ArchInfo {
                aliases: &["riscv64", "riscv", "riscv64gc"],
                supported: &[Linux, BSD, Embedded],
            },
        ),
        (
            PowerPC64,
            ArchInfo {
                aliases: &["ppc64", "ppc64le", "powerpc64"],
                supported: &[Linux, BSD, MacOS, Embedded],
            },
        ),
        (
            AMD32,
            ArchInfo {
                aliases: &["x86", "i386", "ia32", "ia-32", "amd32"],
                supported: &[Linux, Windows, MacOS, BSD, DOS],
            },
        ),
        (
            ARM32,
            ArchInfo {
                aliases: &["arm", "armv7", "armhf", "arm32"],
                supported: &[Linux, MacOS, Embedded],
            },
        ),
        (
            MIPS32,
            ArchInfo {
                aliases: &["mips", "mips32", "mipsel", "mips32r2"],
                supported: &[Linux, BSD, Embedded],
            },
        ),
        (
            SPARC64,
            ArchInfo {
                aliases: &["sparc64", "ultrasparc"],
                supported: &[Linux, BSD, Solaris],
            },
        ),
        (
            IA64,
            ArchInfo {
                aliases: &["ia64", "itanium"],
                supported: &[Linux, Windows],
            },
        ),
        (
            Alpha,
            ArchInfo {
                aliases: &["alpha", "decalpha"],
                supported: &[Linux, BSD],
            },
        ),
        (
            HPPA,
            ArchInfo {
                aliases: &["hppa", "pa-risc"],
                supported: &[Linux, BSD],
            },
        ),
        (
            K68,
            ArchInfo {
                aliases: &["m68k", "68000", "k68"],
                supported: &[Linux, MacOS, Embedded],
            },
        ),
        (
            AVR,
            ArchInfo {
                aliases: &["avr", "atmega"],
                supported: &[Embedded],
            },
        ),
        (
            MSP430,
            ArchInfo {
                aliases: &["msp430"],
                supported: &[Embedded],
            },
        ),
        (
            SH,
            ArchInfo {
                aliases: &["superh", "sh2", "sh3", "sh4"],
                supported: &[Linux, Embedded],
            },
        ),
        (
            VAX,
            ArchInfo {
                aliases: &["vax"],
                supported: &[BSD],
            },
        ),
        (
            NIOSII,
            ArchInfo {
                aliases: &["nios2", "niosii"],
                supported: &[Linux, Embedded],
            },
        ),
        (
            Xtensa,
            ArchInfo {
                aliases: &["xtensa"],
                supported: &[Linux, Embedded],
            },
        ),
        (
            ARC,
            ArchInfo {
                aliases: &["arc", "arc32", "archs"],
                supported: &[Linux, Embedded],
            },
        ),
        (
            Z80,
            ArchInfo {
                aliases: &["z80", "zilog80"],
                supported: &[Embedded],
            },
        ),
    ])
}

/// Resolve an architecture + OS combo from input like "arm64_linux"
pub fn parse_target(input: &str) -> Option<TargetTriple> {
    let db = arch_db();

    let mut parts: Vec<&str> = input.split('_').collect();
    if parts.len() < 2 {
        return None;
    }

    let os_part = parts.pop().unwrap();
    let arch_part = parts.join("_");

    let os = match os_part {
        "linux" => Platform::Linux,
        "windows" => Platform::Windows,
        "macos" => Platform::MacOS,
        "bsd" => Platform::BSD,
        "solaris" => Platform::Solaris,
        "dos" => Platform::DOS,
        "embedded" => Platform::Embedded,
        _ => return None,
    };

    for (arch, info) in db.iter() {
        if info
            .aliases
            .iter()
            .any(|&a| a.eq_ignore_ascii_case(&arch_part))
        {
            if info.supported.contains(&os) {
                let triple = TargetTriple::new(arch.clone(), os);
                return Some(triple);
            } else {
                eprintln!("Error: {arch:?} not supported on {os:?}");
                return None;
            }
        }
    }

    None
}
