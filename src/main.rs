extern crate bytes;

use std::fs;
use bytes::Bytes;
mod string;
mod data;
use data::function;
// sorry for stealing

fn main() {

  let file = "main.son";

  /*match fs::read_to_string(file) {
    Ok(code) => println!("OK"),
    Err(error) => println!("error: {}", error),
  }*/

  let code = fs::read(file);

  if let Err(_e) = code {
    panic!("Error: file {} doesn't exist", file);
  }
  
  let code = Bytes::from(code.unwrap());
  let mut indx: usize = 5usize;
  assert_eq!(code[..5], [111, 42, 83, 111, 78]); // o*SoN
  let fun = function::Fun::new(&code, &mut indx);
  fun.exec(&&code, &mut indx);
}
