pub mod arch;
pub mod core;
pub mod platform;
use std::env;
use std::fs;
use std::process;

use crate::core::TargetTriple;
use crate::core::codegen::CodeGenerator;
use crate::core::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.ua> [-o output.s] [-t target]", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let mut output_file = "output.s".to_string();
    let mut architecture = TargetTriple::linux_amd64();

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("Error: -o requires an output filename");
                    process::exit(1);
                }
            }
            "-t" => {
                if i + 1 < args.len() {
                    architecture = match args[i + 1].as_str() {
                        "x86_64_linux" | "amd64_linux" => TargetTriple::linux_amd64(),
                        "arm64_linux" | "aarch64_linux" => TargetTriple::linux_arm64(),
                        // "riscv64_linux" => TargetTriple::linux_riscv64(),
                        "x86_64_windows" | "amd64_windows" => TargetTriple::windows_amd64(),
                        "arm64_windows" | "aarch64_windows" => TargetTriple::windows_arm64(),
                        // "riscv64_windows" => TargetTriple::windows_riscv64(),
                        "x86_64_macos" | "amd64_macos" => TargetTriple::macos_amd64(),
                        "arm64_macos" | "aarch64_macos" => TargetTriple::macos_arm64(),
                        // "riscv64_macos" => TargetTriple::macos_riscv64(),
                        _ => {
                            eprintln!(
                                "Error: {} architecture isn't currently supported",
                                args[i + 1]
                            );
                            process::exit(1);
                        }
                    };
                    i += 2;
                } else {
                    eprintln!("Error: -t requires a target");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unknown option {}", args[i]);
                process::exit(1);
            }
        }
    }

    let input_content = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file '{}': {}", input_file, err);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(&input_content);
    let instructions = match parser.parse() {
        Ok(instructions) => instructions,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    };

    let code_generator = CodeGenerator::new(architecture);
    let asm_code = code_generator.generate(&instructions);

    if let Err(err) = fs::write(&output_file, asm_code) {
        eprintln!("Error writing output file '{}': {}", output_file, err);
        process::exit(1);
    }

    println!(
        "Successfully compiled '{}' to '{}'",
        input_file, output_file
    );
}
