use std::fs::read_to_string;
use std::io::stdin;

pub fn run_file(filename: &str) {
    let file = read_to_string(filename).unwrap();
    println!("file content {}", file)
}

pub fn run_repl() -> (){
    loop {
        println!(">");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        println!("{}", input)
    }
}
