use std::fs::read_to_string;
use std::io::stdin;


enum Token {
  Ident(String),
  String(String),
  Number(f64),
  LeftParen,
  RightParen,
  Plus,
  Minus,
  Star,
  Slash,
  Dot,
  Comma,
  Semicolon,
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Nil,
  True,
  False,
  This,
  Super,
  Class,
  And,
  Or,
  If,
  Else,
  Return,
  Fun,
  For,
  While,
  Var,
  Print,
  Eof,
}

fn get_inputs(buf: &str) -> Vec<char> {
  let mut inputs = Vec::new();
  let mut chars = buf.chars();
  loop {
    match chars.next() {
      Some(x) => inputs.push(x),
      None => { inputs.push('\0'); break}
    }
  }
  inputs
}

pub fn run_file(filename: &str) {
    let file = read_to_string(filename).unwrap();
    let _inputs = get_inputs(&file);
    println!("file content {}", file);
}

pub fn run_repl() -> (){
    loop {
        println!(">");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        println!("{}", input)
    }
}
