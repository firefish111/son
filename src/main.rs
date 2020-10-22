use std::fs;
// use std::env;
mod string;
// sorry for stealing

fn main() {
  let file = "/home/runner/SoN-VM/main.son";
  let code = fs::read_to_string(file);

  if let Err(_e) = code {
    panic!("Error: file {} doesn't exist", file);
  }
  
  let code = code.unwrap().into_bytes();
  assert_eq!(code[..5], [111, 42, 83, 111, 78]); // o*SoN
  print!("{}", crate::string::get_str(code, &mut (5usize)));
}