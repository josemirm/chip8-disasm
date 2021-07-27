mod opcodes;

use std;
use std::env;

fn disasm(fname: &String) -> Result<String, std::io::Error> {
    let mut out: String = String::new();
    let mut code_line: u16 = 0x200;

    let bytes: Vec<u8> = std::fs::read(fname)?;
    let mut i: usize = 0;

    while i < bytes.len() {
        let mut opcode: u16 = bytes[i] as u16;
        opcode <<= 8;
        opcode |= bytes[i + 1] as u16;

        out += opcodes::decode_opcode(code_line, opcode).as_str();

        code_line += 2;

        // This is necessary to not repeat bytes
        i += 2;
    }

    Ok(out)
}

fn main() -> Result<(), std::io::Error> {
    println!("\nChip-8 Disassembler");
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Only input file in arguments
        2 => {
            let out: String = disasm(&args[1])?;
            println!("{}", out);
        }

        // Input and output file in arguments
        3 => {
            println!("(unimplemented");
        }

        // Other cases
        _ => println!(
            "- Expected from the arguments:\n\
                    \tBinary Chip-8 input file\n\
                    \tAssembly text output file"
        ),
    }

    Ok(())
}
