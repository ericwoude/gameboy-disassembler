use std::collections::HashMap;
use std::fmt;
use std::fs;

extern crate hex_string;
use hex_string::u8_to_hex_string;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Operand {
    pub name: String,
    pub bytes: Option<u8>,
    pub increment: Option<bool>,
    pub decrement: Option<bool>,
    pub immediate: bool,
    pub value: Option<u16>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Instruction {
    pub mnemonic: String,
    pub bytes: u8,
    pub cycles: Vec<u8>,
    pub operands: Vec<Operand>,
    pub flags: HashMap<String, String>,
}

type Instructions = HashMap<String, Instruction>;

#[derive(Debug, serde::Deserialize)]
pub struct InstructionBank {
    pub unprefixed: Instructions,
    pub cbprefixed: Instructions,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_some() {
            if self.immediate {
                write!(f, "{:#02X}", self.value.unwrap())
            } else {
                write!(f, "({:#02X})", self.value.unwrap())
            }
        } else {
            let mut name = self.name.clone();

            if self.increment.is_some() {
                name += "+"
            }

            if self.decrement.is_some() {
                name += "-"
            }

            if self.immediate {
                write!(f, "{}", name)
            } else {
                write!(f, "({})", name)
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operand_string = self
            .operands
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        if operand_string.is_empty() {
            write!(f, "{}", self.mnemonic)
        } else {
            write!(f, "{} {}", self.mnemonic, operand_string)
        }
    }
}

pub fn parse_from_file(location: &str) -> InstructionBank {
    let data = fs::read_to_string(location).expect("Unable to read file");
    let bank: InstructionBank = serde_json::from_str(&data).expect("Unable to parse data");

    bank
}

pub fn load_cartridge(location: &str) -> Vec<u8> {
    std::fs::read(location).unwrap()
}

// Refactor: deserialize json as hexstring so becomes obsolete.
fn make_opcode_string(opcode: &u8) -> String {
    let mut s = String::from("0x");

    let chars: Vec<char> = Vec::from(u8_to_hex_string(opcode));

    for ch in chars {
        s.push(ch.to_ascii_uppercase());
    }

    s
}

pub fn decode(
    mut address: usize,
    bytes: &[u8],
    instructions: &InstructionBank,
) -> (usize, Instruction) {
    let opcode = bytes[address];
    address += 1;

    let meta_instruction = if opcode == 0xCB {
        let s = make_opcode_string(&bytes[address]);
        address += 1;
        &instructions.cbprefixed[&s]
    } else {
        let s = make_opcode_string(&opcode);
        &instructions.unprefixed[&s]
    };

    // Reconstruct a new instruction with added value detail for operands
    let decoded_operands = meta_instruction
        .operands
        .iter()
        .map(|o| match o.bytes {
            Some(2) => {
                let v: [u8; 2] = bytes[address..address + 2]
                    .try_into()
                    .expect("Out of bounds");
                address += 2;
                Operand {
                    name: o.name.clone(),
                    bytes: o.bytes,
                    increment: o.increment,
                    decrement: o.decrement,
                    immediate: o.immediate,
                    value: Some(u16::from_le_bytes(v)),
                }
            }
            Some(1) => {
                let val = bytes[address];
                address += 1;
                Operand {
                    name: o.name.clone(),
                    bytes: o.bytes,
                    increment: o.increment,
                    decrement: o.decrement,
                    immediate: o.immediate,
                    value: Some(u16::from(val)),
                }
            }
            Some(_) => o.clone(),
            None => o.clone(),
        })
        .collect::<Vec<Operand>>();

    let decoded_instruction = Instruction {
        mnemonic: meta_instruction.mnemonic.clone(),
        bytes: meta_instruction.bytes,
        cycles: meta_instruction.cycles.clone(),
        operands: decoded_operands,
        flags: meta_instruction.flags.clone(),
    };

    (address, decoded_instruction)
}

pub fn disassemble(
    start_address: usize,
    bytes: &[u8],
    instructions: &InstructionBank,
    amount_of_instructions: usize,
) {
    let mut address = start_address;

    for _ in 0..amount_of_instructions {
        let (new_address, instruction) = decode(address, bytes, instructions);
        println!("0x{:02X?} {}", address, instruction);
        address = new_address;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_operand() {
        let a = Operand {
            name: String::from("BC"),
            increment: None,
            decrement: None,
            bytes: None,
            immediate: true,
            value: None,
        };

        let b = Operand {
            name: String::from("HL"),
            increment: Some(true),
            decrement: None,
            bytes: None,
            immediate: false,
            value: None,
        };

        let c = Operand {
            name: String::from("HL"),
            increment: None,
            decrement: Some(true),
            bytes: None,
            immediate: false,
            value: None,
        };

        assert_eq!("BC", a.to_string());
        assert_eq!("(HL+)", b.to_string());
        assert_eq!("(HL-)", c.to_string());
    }

    #[test]
    fn display_metadata() {
        let instructions = parse_from_file("./instructions.json");

        // unprefixed
        assert_eq!("LD BC, d16", instructions.unprefixed["0x01"].to_string());
        assert_eq!("LD (HL-), A", instructions.unprefixed["0x32"].to_string());
        assert_eq!("HALT", instructions.unprefixed["0x76"].to_string());
        assert_eq!("POP DE", instructions.unprefixed["0xD1"].to_string());

        // cbprefixed
        assert_eq!("RLC (HL)", instructions.cbprefixed["0x06"].to_string());
        assert_eq!("BIT 0, (HL)", instructions.cbprefixed["0x46"].to_string());
        assert_eq!("BIT 6, A", instructions.cbprefixed["0x77"].to_string());
        assert_eq!("SET 6, B", instructions.cbprefixed["0xF0"].to_string());
    }
}
