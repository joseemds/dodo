use std::env;
use dodo::lexer;

fn main() {
    let args : Vec<String> = env::args().collect();
    match args.as_slice() {
      [_bin_path] => lexer::run_repl(),
      [_, file] => lexer::run_file(file),
      _ => println!("usage: dodo [file]"),
    }
}
