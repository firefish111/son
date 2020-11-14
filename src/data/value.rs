use super::types::*;
use std::collections::HashMap;

pub fn val(dat: &bytes::Bytes, mut indx: &mut usize, list: &mut HashMap<String, DataType>) {
  assert_eq!(&dat[*indx], &0x8Eu8);
}
