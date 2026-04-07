mod parser;
use crate::parser::build_graph;

fn main() {
    let imports = build_graph("src/index.js");
    println!("{:?}", imports);
}
