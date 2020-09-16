use std::env;

fn main() {
   let port = env::var("PORT").unwrap_or("3000".to_string());
   println!("{}", port);
}
