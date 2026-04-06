fn main() {
    let contents = std::fs::read_to_string("src/index.js").unwrap();

    for line in contents.lines() {
        if line.contains("from '") {
            let path = line.split('\'').nth(1).unwrap();
            println!("{}", path);
        } else if line.contains("from \"") {
            let path = line.split('"').nth(1).unwrap();
            println!("{}", path);
        }
    }
}
