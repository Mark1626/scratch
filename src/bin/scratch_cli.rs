use scratch::parser::parse::parse_file;

use std::fs;

fn main() {
  let file_name = "/Users/nimalanm/Documents/open-source/personal/crawler_dsl/examples/basic";
  let file_content = fs::read_to_string(file_name).expect("unable to read file");
  let parsed = parse_file(&file_content).next().unwrap();
  println!("Parsed {}", parsed);
}
