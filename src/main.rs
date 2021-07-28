mod opcodes;
mod utils;

use std;
use std::env;
use std::fs::File;
use std::io::Write;

const PROGRAM_MSG: &str =
    "Chip-8 Disassembler by josemirm. Check more at https://github.com/josemirm/chip8-disasm";

fn disasm(fname: &String) -> Result<String, std::io::Error> {
    const FIRST_CODE_LINE: u16 = 0x200;

    let bytes: Vec<u8> = std::fs::read(fname)?;
    let mut out: String = String::new();

    let mut i: usize = 0;
    while i < bytes.len() {
        let mut opcode: u16 = bytes[i] as u16;
        opcode <<= 8;
        opcode |= bytes[i + 1] as u16;

        out += opcodes::decode_opcode(FIRST_CODE_LINE + (i as u16), opcode).as_str();

        // Increments two bytes because all instructions have two bytes.
        i += 2;
    }

    Ok(out)
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut out: String = String::new();
    if args.len() > 1 {
        out = disasm(&args[1])?;
    }

    match args.len() {
        // Only input file in arguments
        2 => println!("; {}\n\n{}", PROGRAM_MSG, out),

        // Input and output file in arguments
        3 => {
            let mut file = File::create(&args[2])?;
            let msg: String = String::from(format!("; {}\n\n", PROGRAM_MSG));
            file.write_all(msg.as_bytes())?;
            file.write_all(out.as_bytes())?;
        }

        // Other cases
        _ => {
            let prog_name: String = utils::trim_filename_path(&args[0]);
            println!("{}\n", PROGRAM_MSG);
            println!("Usage: {} <input_file> <output_file>", prog_name);
            println!("If no output file is given, it will print the text in screen.\n");
        }
    }

    Ok(())
}
