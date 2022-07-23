use std::fs::read_to_string;

pub fn run_file(filename: &str) {
    let file = read_to_string(filename);
    match file {
      Ok(content) => println!("file contents: {}", content),
      Err(e) => panic!("{}", e),
    }
}
