extern crate bytes;

use std::fs;
use bytes::Bytes;
mod data;
mod pool; // lol dis module broken
use data::function::Fun;
// sorry for stealing

fn main() {
  let file = "main.son";
  let code = match fs::read(file) {
    Ok(dat) => Bytes::from(dat),
    Err(error) => panic!("Fatal Error: {}", error),
  };
  
  let mut indx: usize = 5usize;
  assert_eq!(code[..5], b"o*SoN"[..]); // magic number for SoN
  let fun = Fun::new(&code, &mut indx);
  fun.exec(&&code);
}
