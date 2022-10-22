mod parse;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Relative path pointing to GameBoy ROM
    #[arg(short, long, required(true))]
    file: String,

    /// Entry address for disassembly
    #[arg(short, long, default_value_t = 0x150, value_parser = clap::builder::ValueParser::new(parse_hex_string))]
    entry: usize,

    /// Amount of instructions to decode from starting point
    #[arg(short, long, default_value_t = 8)]
    amount: usize,
}

fn parse_hex_string(s: &str) -> Result<usize, std::num::ParseIntError> {
    let mut base = 10;
    let mut numbers = s;

    if s.starts_with("0x") || s.starts_with("0X") {
        base = 16;
        numbers = &s[2..];
    }

    usize::from_str_radix(numbers, base)
}

fn main() {
    let args = Args::parse();

    let instructions = parse::parse_from_file("./instructions.json");
    let bytes = parse::load_cartridge(&args.file);

    parse::disassemble(args.entry, &bytes, &instructions, args.amount);
}
