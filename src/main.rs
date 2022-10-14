mod parse;

fn main() {
    let instructions = parse::parse_from_file("./instructions.json");
    let bytes = parse::load_cartridge("./snake.gb");

    parse::disassemble(0x150, &bytes, &instructions, 8);
}
