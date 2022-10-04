mod parse;

fn main() {
    let _instructions = parse::parse_from_file("./instructions.json");
    println!("{:?}", _instructions.unprefixed["0x00"]);
}
