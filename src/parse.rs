use std::collections::HashMap;
use std::fs;

use serde_json;

#[derive(Debug, serde::Deserialize)]
pub struct Operand {
    pub name: String,
    pub bytes: Option<u8>,
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

pub fn parse_from_file(location: &str) -> Instructions {
    let data = fs::read_to_string(location).expect("Unable to read file");
    let instructions: Instructions = serde_json::from_str(&data).expect("Unable to parse data");

    return instructions;
}
