use std::fs;

fn main() {
    let content = fs::read_to_string("./Carggo.toml").expect("Can't read toml file");
    println!("{}", content);
}
