extern crate bytes;

use std::{fs, collections::HashMap};
use bytes::Bytes;
mod data;
mod pool; // lol dis module broken
use data::{function::Fun, types::DataType};
// sorry for stealing

fn main() {
  let mut callable: HashMap<String, Fun> = HashMap::new();

  let code = match fs::read("main.son") {
    Ok(dat) => Bytes::from(dat),
    Err(error) => panic!("Fatal Error: {}", error),
  };
  
  let mut indx: usize = 5usize;
  assert_eq!(code[..5], b"o*SoN"[..]); // magic number for SoN
  let fun = Fun::new(&code, &mut indx, &mut callable);
  callable[&fun].exec(&&code);
}
