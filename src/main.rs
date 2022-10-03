mod parse;

fn main() {
    let _instructions = parse::parse_from_file("./opcodes.json");
}
