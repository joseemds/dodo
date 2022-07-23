use std::env;
use dodo::lexer;

fn main() {
    let args : Vec<String> = env::args().collect();
    match args.as_slice() {
      [] => println!("usage: dodo [file]"),
      [_, file] => lexer::run_file(file),
      _ => println!("usage: dodo [file]"),
    }
}
