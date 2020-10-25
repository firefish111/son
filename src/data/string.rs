use crate::pool::get_index;
use super::types::DataType;

pub fn get_str(dat: &bytes::Bytes, indx: &mut usize) -> Option<usize> {
  let end: usize = *indx + dat[*indx] as usize + 1;
  if dat[end] != 0u8 {
    println!("Warning: string at index {} is not null-terminated, instead it is terminated with {}", indx, dat[end]);
  }
  let out = String::from(std::str::from_utf8(&dat[*indx+1..end]).unwrap());
  *indx = end;
  return get_index(DataType::Str(out));
}