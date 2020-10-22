pub fn get_str(bytes: Vec<u8>, indx: &mut usize) -> String {
  let end: usize = *indx + bytes[*indx] as usize + 1;
  if bytes[end] != 0u8 {
    println!("Warning: string at index {} is not null-terminated, instead it is terminated with {}", indx, bytes[end]);
  }
  let out = String::from(std::str::from_utf8(&bytes[*indx+1..end]).unwrap());
  *indx = end;
  return out;
}