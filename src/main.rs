use std::fs;

fn main() {
    let content = fs::read_to_string("./Cargo.toml").unwrap();
    println!("{}", content);
}
