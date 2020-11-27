use super::types::*;
use super::string::get_str;
use crate::pool;
use std::collections::HashMap;

pub fn val(
  dat: &bytes::Bytes,
  mut indx: &mut usize,
  list: &mut HashMap<String, DataType>,
) -> String {
  assert_eq!(&dat[*indx], &0x8Eu8);
  let main = DataType::get_typ(&dat[*indx + 1]);

  *indx += 2;
  let name: String = if let DataType::Str(nm) = pool::get_val(get_str(&dat, &mut indx)) {
    nm.clone()
  } else {
    panic!("Fatal Error: Not a string")
  };

  list.insert(name.clone(), main.upd(&dat[*indx])); // TODO: typing
  name
}
