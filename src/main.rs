mod parse;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, required(true))]
    file: String,

    #[arg(short, long, default_value_t = 0x150)]
    entry: usize,
}

fn main() {
    let args = Args::parse();

    let instructions = parse::parse_from_file("./instructions.json");
    let bytes = parse::load_cartridge(&args.file);

    parse::disassemble(args.entry, &bytes, &instructions, 8);
}
