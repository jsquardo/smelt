mod parser;
use crate::parser::parse_imports;

fn main() {
    let imports = parse_imports("src/index.js");
    println!("{:?}", imports);
}
