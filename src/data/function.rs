use super::string::get_str;
use super::types::DataType;
use crate::pool;
use std::collections::HashMap;

#[allow(dead_code)] // will fix SoonTM
#[derive(Debug)]
pub struct Fun<'q> {
  start: usize,
  length: usize,
  sig: HashMap<&'q String, DataType>,
  ret: DataType,
}

impl Fun<'_> {
  pub fn define<'a>(dat: &'a bytes::Bytes, mut indx: &mut usize, list: &mut HashMap<String, Fun>) -> String {
    assert_eq!(&dat[*indx], &0xDFu8);
    let mut sig = HashMap::new();
    let ret: DataType = DataType::get_typ(&dat[*indx + 1]);

    *indx += 2;
    let name: String = if let DataType::Str(nm) = pool::get_val(get_str(&dat, &mut indx)) {
      nm.clone()
    } else {
      panic!("Fatal Error: Not a string")
    };

    *indx += 1;
    if dat[*indx] != 0u8 {
      for _param in 0..dat[*indx - 1] {
        let typ = DataType::get_typ(&dat[*indx]);
        *indx += 1;
        if let DataType::Str(name) = pool::get_val(get_str(&&dat, indx)) {
          sig.insert(name, typ);
        } else {
          panic!("Parameter name not supplied");
        }
      }
    }
    let start = *indx;

    let mut length = 0;
    while dat[*indx..=*indx + 1] != (b";;"[..]) {
      *indx += 1;
      length += 1;
    }
    list.insert(
      name.clone(),
      Fun {start, length, sig, ret},
    );
    name
  }
  pub fn exec(&self, code: &bytes::Bytes) -> &DataType {
    let mut x = self.start + 1;
    pool::get_val(get_str(&code, &mut x))
  }
}
