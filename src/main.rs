extern crate bytes;

use bytes::Bytes;
use std::{collections::HashMap, fs};
mod data;
mod pool; // lol dis module broken
use data::{function::Fun, types::DataType, value::val};
// sorry for stealing

fn main() {
  let mut callable: HashMap<String, Fun> = HashMap::new();
  let mut variable: HashMap<String, DataType> = HashMap::new();

  let code = match fs::read("main.son") {
    Ok(dat) => Bytes::from(dat),
    Err(error) => panic!("Fatal Error: {}", error), // The difference between unwrap and having Fatal Error infront
  };

  let mut indx: usize = 5usize;
  assert_eq!(code[..5], b"o*SoN"[..]); // magic number for SoN
  let fun = Fun::define(&code, &mut indx, &mut callable);
  if let DataType::Str(elf) = callable[&fun].exec(&&code) {
    print!("{}", elf);
  }
  let var = val(&code, &mut indx, &mut variable);
  println!("Function data: {:#x?}\nVariable data: {:#x?}", callable[&fun], variable[&var]);

  //
}
