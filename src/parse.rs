use std::collections::HashMap;
use std::fmt;
use std::fs;

use serde_json;

#[derive(Debug, serde::Deserialize)]
pub struct Operand {
    pub name: String,
    pub bytes: Option<u8>,
    pub increment: Option<bool>,
    pub decrement: Option<bool>,
    pub immediate: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct Metadata {
    pub mnemonic: String,
    pub bytes: u8,
    pub cycles: Vec<u8>,
    pub operands: Vec<Operand>,
    pub flags: HashMap<String, String>,
}

type Instructions = HashMap<String, Metadata>;

#[derive(Debug, serde::Deserialize)]
pub struct InstructionBank {
    pub unprefixed: Instructions,
    pub cbprefixed: Instructions,
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut name = self.name.clone();

        match self.increment {
            Some(_k) => name += "+",
            None => (),
        }

        match self.decrement {
            Some(_k) => name += "-",
            None => (),
        }

        if self.immediate {
            write!(f, "{}", name)
        } else {
            write!(f, "({})", name)
        }
    }
}

impl fmt::Display for Metadata {
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

    return bank;
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
        };

        let b = Operand {
            name: String::from("HL"),
            increment: Some(true),
            decrement: None,
            bytes: None,
            immediate: false,
        };

        let c = Operand {
            name: String::from("HL"),
            increment: None,
            decrement: Some(true),
            bytes: None,
            immediate: false,
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
