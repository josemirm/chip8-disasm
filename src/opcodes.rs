pub fn decode_opcode(code_line: u16, opcode: u16) -> String {
    format!(
        "{:#06X}:\t {}\n",
        code_line,
        opcode_to_string(opcode).as_str()
    )
}

// Formats of the opcodes used to get their arguments
fn op_fmt_0nnn(opcode: u16) -> String {
    format!("{:#06X}", opcode & 0x0FFFu16)
}

fn op_fmt_0xkk(opcode: u16) -> String {
    format!(
        "V[{:X}], {:#04X}",
        (opcode & 0x0F00) >> 8,
        (opcode & 0x00FF) >> 4
    )
}

fn op_fmt_0xy0(opcode: u16) -> String {
    format!(
        "V[{:X}], V[{:X}]",
        opcode & 0x0F00 >> 8,
        opcode & 0x00F0 >> 4
    )
}

fn op_fmt_0x00(opcode: u16) -> String {
    format!("V[{:X}]", (opcode & 0x0F00) >> 12)
}

// Conversion from opcode to string
fn opcode_to_string(opcode: u16) -> String {
    match opcode {
        0x00E0 => String::from("CLS"),
        0x00EE => String::from("RET"),
        i if i < 0x0FFF => format!("SYS {}", op_fmt_0nnn(opcode)),
        i if (i & 0xF000) == 0x1000 => format!("JP {}", op_fmt_0nnn(opcode)),
        i if (i & 0xF000) == 0x2000 => format!("CALL {}", op_fmt_0nnn(opcode)),
        i if (i & 0xF000) == 0x3000 => format!("SE {} ", op_fmt_0xkk(opcode)),
        i if (i & 0xF000) == 0x4000 => format!("SNE {}] ", op_fmt_0xkk(opcode)),
        i if (i & 0xF000) == 0x5000 => format!("SE {}] ", op_fmt_0xy0(opcode)),
        i if (i & 0xF000) == 0x6000 => format!("LD {}", op_fmt_0xkk(opcode)),
        i if (i & 0xF000) == 0x7000 => format!("ADD {}", op_fmt_0xkk(opcode)),

        i if (i & 0xF00F) == 0x8000 => format!("LD {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x8001 => format!("OR {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x8002 => format!("AND {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x8003 => format!("XOR {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x8004 => format!("ADD {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x8005 => format!("SUB {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x8006 => format!("SHR {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x8007 => format!("SUBN {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF00F) == 0x800E => format!("SHL {}", op_fmt_0xy0(opcode)),

        i if (i & 0xF000) == 0x9000 => format!("SNE {}", op_fmt_0xy0(opcode)),
        i if (i & 0xF000) == 0xA000 => format!("LD I, {}", op_fmt_0nnn(opcode)),
        i if (i & 0xF000) == 0xB000 => format!("JMP V[0], {}", op_fmt_0nnn(opcode)),
        i if (i & 0xF000) == 0xC000 => format!("LD {}", op_fmt_0xkk(opcode)),
        i if (i & 0xF000) == 0xD000 => {
            format!("DRW {}, {:#03x}", op_fmt_0xy0(opcode), opcode & 0x000Fu16)
        }

        i if (i & 0xF0FF) == 0xE09E => format!("SKP {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xE0A1 => format!("SKPN {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF007 => format!("LD {}, DT", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF00A => format!("LD {}, K", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF015 => format!("LD DT, {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF018 => format!("LD ST, {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF01E => format!("ADD I, {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF029 => format!("LD F, {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF033 => format!("LD B, {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF055 => format!("LD I, {}", op_fmt_0x00(opcode)),
        i if (i & 0xF0FF) == 0xF065 => format!("LD {}, I", op_fmt_0x00(opcode)),

        _ => String::from("(unknown)"),
    }
}
